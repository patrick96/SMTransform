#!/usr/bin/env python3

import fileinput
import json
import argparse
import subprocess
import sys
import signal
from subprocess import TimeoutExpired
from dataclasses import dataclass
from typing import Optional
from enum import Enum

TIMEOUT = 10


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


@dataclass
class Input:
    base: str
    seed: int
    round: int
    smtlib: str
    status: str

    @staticmethod
    def get(j: dict):
        return Input(
            base=j['base'],
            seed=int(j['seed']),
            round=int(j['round']),
            smtlib=j['smtlib'],
            status=j['status'],
        )

    def id(self):
        return f"{self.base.replace('/', '_')}_{self.seed}_{self.round}"


class ResultKind(Enum):
    Success = 0
    Failure = 1
    Timeout = 2
    Signal = 3
    Unknown = 4
    Unsound = 5


@dataclass
class RunResult:
    input: Input
    cmd: [str]
    stdout: [str]
    stderr: [str]
    exitcode: int

    def to_json(self):
        data = self.__dict__
        data['input'] = self.input.__dict__
        data['kind'] = self.get_kind().name
        return json.dumps(data)

    def is_unsound(self):
        return False

    @staticmethod
    def get(input: Input, cmd: [str], stdout: [str], stderr: [str],
            exitcode: int):
        if exitcode == 0:
            if stdout == [input.status] and not stderr:
                return SuccessResult(input=input,
                                     cmd=cmd,
                                     stdout=stdout,
                                     stderr=stderr,
                                     exitcode=exitcode)

            if stdout == ["unknown"]:
                return UnknownResult(input=input,
                                     cmd=cmd,
                                     stdout=stdout,
                                     stderr=stderr,
                                     exitcode=exitcode)

            if stdout != [input.status]:
                return UnsoundResult(input=input,
                                     cmd=cmd,
                                     stdout=stdout,
                                     stderr=stderr,
                                     exitcode=exitcode)

            return FailureResult(input=input,
                                 cmd=cmd,
                                 stdout=stdout,
                                 stderr=stderr,
                                 exitcode=exitcode)

        elif exitcode > 0:
            return FailureResult(input=input,
                                 cmd=cmd,
                                 stdout=stdout,
                                 stderr=stderr,
                                 exitcode=exitcode)
        elif exitcode < 0:
            sig = signal.Signals(abs(exitcode)).name
            return SignalResult(input=input,
                                cmd=cmd,
                                stdout=stdout,
                                stderr=stderr,
                                exitcode=exitcode,
                                signal=sig)

    def is_success(self):
        return False

    def is_timeout(self):
        return False

    def is_signal(self):
        return False

    def is_unknown(self):
        return False

    def get_kind(self) -> ResultKind:
        raise NotImplementedError()

    def get_char(self) -> str:
        raise NotImplementedError()


@dataclass
class TimeoutResult(RunResult):
    timeout: int

    def is_timeout(self):
        return True

    def get_kind(self) -> ResultKind:
        return ResultKind.Timeout

    def get_char(self) -> str:
        return "T"


@dataclass
class SignalResult(RunResult):
    """self.exitcode < 0"""
    signal: str

    def is_signal(self):
        return True

    def get_kind(self) -> ResultKind:
        return ResultKind.Signal

    def get_char(self) -> str:
        return "S"


@dataclass
class FailureResult(RunResult):
    """
    Any failure that doesn't match other cases

    For example:

    * self.exitcode > 0
    * self.stderr != []

    """

    def get_kind(self) -> ResultKind:
        return ResultKind.Failure

    def get_char(self) -> str:
        return "F"


@dataclass
class SuccessResult(RunResult):
    """
    Completely correct run

    * self.exitcode = 0
    * self.stderr == []
    * self.stdout == [status]

    """

    def is_success(self):
        return True

    def get_kind(self) -> ResultKind:
        return ResultKind.Success

    def get_char(self) -> str:
        return "."

@dataclass
class UnknownResult(RunResult):
    """self.stdout == [unknown]"""

    def is_unknown(self):
        return True

    def get_kind(self) -> ResultKind:
        return ResultKind.Unknown

    def get_char(self) -> str:
        return "U"

@dataclass
class UnsoundResult(RunResult):
    """self.stdout == [unknown]"""

    def is_unsound(self):
        return True

    def get_kind(self) -> ResultKind:
        return ResultKind.Unsound

    def get_char(self) -> str:
        return "#"


def run_cmd(cmd, input: Input) -> RunResult:
    assert cmd, "No command given"
    try:
        proc = subprocess.run(cmd,
                              timeout=TIMEOUT,
                              capture_output=True,
                              input=input.smtlib,
                              text=True)
        code = proc.returncode

        stdout = list(proc.stdout.splitlines())
        stderr = list(proc.stderr.splitlines())

        return RunResult.get(input, cmd, stdout, stderr, code)
    except TimeoutExpired as ex:
        return TimeoutResult(input=input,
                             cmd=cmd,
                             stdout=list(ex.stdout.decode().splitlines()) if ex.stdout else [],
                             stderr=list(ex.stderr.decode().splitlines()) if ex.stderr else [],
                             exitcode=0,
                             timeout=TIMEOUT)


def on_input(cmd: [str], line: str) -> Optional[RunResult]:
    input = Input.get(json.loads(line))
    result = run_cmd(cmd, input)
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description='Run some command and collect failures')

    parser.add_argument('remaining',
                        nargs=argparse.REMAINDER,
                        help='Command to run program')

    args = parser.parse_args()

    cmd = args.remaining

    if len(cmd) > 0 and cmd[0] == '--':
        cmd = cmd[1:]

    if len(cmd) == 0:
        eprint("No command given")
        sys.exit(1)

    for line in fileinput.input(['-']):
        result = on_input(cmd, line)

        if not result.is_success() and not result.is_timeout():
            print(result.to_json())

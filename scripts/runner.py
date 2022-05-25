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
from pathlib import Path

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
        return f"{self.base}_{self.seed}_{self.round}"


class ResultKind(Enum):
    Success = 0
    Failure = 1
    Timeout = 2
    Signal = 3


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

    def dump(self, dir: Path):
        with open(dir / (self.input.id() + ".json"), "w") as f:
            f.write(self.to_json())

    def is_unsound(self):
        return self.stdout and self.stdout != [self.input.status]

    @staticmethod
    def get(input: Input, cmd: [str], stdout: [str], stderr: [str],
            exitcode: int):
        if exitcode == 0:
            return SuccessResult(input=input,
                                 cmd=cmd,
                                 stdout=stdout,
                                 stderr=stderr,
                                 exitcode=0)
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

    def get_kind(self) -> ResultKind:
        raise NotImplementedError()


@dataclass
class TimeoutResult(RunResult):
    timeout: int

    def is_timeout(self):
        return True

    def get_kind(self) -> ResultKind:
        return ResultKind.Timeout


@dataclass
class SignalResult(RunResult):
    """self.exitcode < 0"""
    signal: str

    def is_signal(self):
        return True

    def get_kind(self) -> ResultKind:
        return ResultKind.Signal


@dataclass
class FailureResult(RunResult):
    """self.exitcode > 0"""

    def get_kind(self) -> ResultKind:
        return ResultKind.Failure


@dataclass
class SuccessResult(RunResult):
    """self.exitcode == 0"""

    def is_success(self):
        return True

    def get_kind(self) -> ResultKind:
        return ResultKind.Success


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
                             stdout=ex.stdout if ex.stdout else [],
                             stderr=ex.stderr if ex.stderr else [],
                             exitcode=0,
                             timeout=TIMEOUT)


def on_input(cmd: [str], line: str) -> Optional[RunResult]:
    input = Input.get(json.loads(line))
    result = run_cmd(cmd, input)

    if not result.is_success() or result.is_unsound() or result.stderr:
        return result

    return None


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

        if result:
            data = result.__dict__
            data['type'] = type(result).__name__

            print(data.to_json())

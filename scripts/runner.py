#!/usr/bin/env python3

import fileinput
import json
import argparse
import subprocess
import sys
import signal
from subprocess import TimeoutExpired
from dataclasses import dataclass

# TODO post-process RunResult:
# TODO detect sanitizer error
# TODO detect correct/wrong output (requires oracle)
# TODO may require per-solver class
# TODO detect solver (error ...) messages

TIMEOUT = 10


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


@dataclass
class RunResult:
    cmd: [str]
    stdout: [str]
    stderr: [str]
    exitcode: int

    @staticmethod
    def get(cmd: [str], stdout: [str], stderr: [str], exitcode: int):
        if exitcode == 0:
            return SuccessResult(cmd=cmd,
                                 stdout=stdout,
                                 stderr=stderr,
                                 exitcode=0)
        elif exitcode > 0:
            return FailureResult(cmd=cmd,
                                 stdout=stdout,
                                 stderr=stderr,
                                 exitcode=exitcode)
        elif exitcode < 0:
            sig = signal.Signals(abs(exitcode)).name
            return SignalResult(cmd=cmd,
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


@dataclass
class TimeoutResult(RunResult):
    timeout: int

    def is_timeout(self):
        return True


@dataclass
class SignalResult(RunResult):
    """self.exitcode < 0"""
    signal: str

    def is_signal(self):
        return True


@dataclass
class FailureResult(RunResult):
    """self.exitcode > 0"""


@dataclass
class SuccessResult(RunResult):
    """self.exitcode == 0"""

    def is_success(self):
        return True


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


def run_cmd(cmd, input):
    assert cmd, "No command given"
    try:
        proc = subprocess.run(cmd,
                              timeout=TIMEOUT,
                              capture_output=True,
                              input=input,
                              text=True)
        code = proc.returncode

        stdout = list(proc.stdout.splitlines())
        stderr = list(proc.stderr.splitlines())

        return RunResult.get(cmd, stdout, stderr, code)
    except TimeoutExpired as ex:
        return TimeoutResult(cmd=cmd,
                             stdout=ex.stdout if ex.stdout else [],
                             stderr=ex.stderr if ex.stderr else [],
                             exitcode=0,
                             timeout=TIMEOUT)


def on_input(input: Input):
    result = run_cmd(cmd, input.smtlib)

    if not result.is_success() or result.stdout != [input.status]:
        data = {
            'input': input.__dict__,
            'result': result.__dict__,
        }

        data['result']['type'] = type(result).__name__

        print(json.dumps(data))


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
        on_input(Input.get(json.loads(line)))

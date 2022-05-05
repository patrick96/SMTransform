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

# TODO post-process RunResult:
# TODO detect sanitizer error
# TODO detect correct/wrong output (requires oracle)
# TODO may require per-solver class
# TODO detect solver (error ...) messages


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


@dataclass
class RunResult:
    cmd: [str]
    stdout: [str]
    stderr: [str]
    exitcode: int
    signal: Optional[str]

    timeout: Optional[TimeoutExpired]

    def print(self):
        eprint(f'result: {self.type()}', end='')

        if self.timeout:
            eprint(f' ({self.timeout.timeout})')
        elif self.signal:
            eprint(f' ({self.signal})')
        else:
            eprint()

        eprint(f'exitcode: {self.exitcode}')

        eprint('stdout:')
        eprint("\n".join(self.stdout))
        eprint('stderr:')
        eprint("\n".join(self.stderr))

    def type(self):
        if self.timeout:
            return "timeout"
        elif self.signal:
            return "signal"
        elif self.exitcode != 0:
            return "failure"
        else:
            return "success"


def run_cmd(cmd, input):
    assert cmd, "No command given"

    try:
        proc = subprocess.run(
            cmd, timeout=10, capture_output=True, input=input, text=True)
        code = proc.returncode

        sig = signal.Signals(abs(code)).name if code < 0 else None

        stdout = list(proc.stdout.splitlines())
        stderr = list(proc.stderr.splitlines())

        return RunResult(cmd=cmd,
                         stdout=stdout,
                         stderr=stderr,
                         exitcode=code,
                         signal=sig,
                         timeout=None)

    except TimeoutExpired as ex:
        return RunResult(cmd=cmd,
                         stdout=ex.stdout if ex.stdout else [],
                         stderr=ex.stderr if ex.stderr else [],
                         exitcode=0,
                         signal=None,
                         timeout=ex)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description='Run some command and collect failures')

    parser.add_argument('remaining', nargs=argparse.REMAINDER,
                        help='Command to run program')

    args = parser.parse_args()

    cmd = args.remaining

    if len(cmd) > 0 and cmd[0] == '--':
        cmd = cmd[1:]

    if len(cmd) == 0:
        eprint("No command given")
        sys.exit(1)

    for line in fileinput.input(['-']):
        j = json.loads(line)

        result = run_cmd(cmd, j['smtlib'])

        if result.type() != 'success' or result.stdout != [j['status']]:
            if result.type() == 'timeout':
                print('timeout')
                continue

            print(j['smtlib'])
            result.print()
            print(json.dumps(result, default=lambda o: o.__dict__, indent=4))
            sys.exit(1)

#!/usr/bin/env python3

import argparse
import subprocess
import sys
import json

from pathlib import Path
from subprocess import PIPE


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def run(cmd: Path, runner: [str], seed: Path, rounds: int,
        iterations: int) -> [dict]:

    outputs = []

    for prng_seed in range(iterations):
        print(f'\rUsing seed {seed} {prng_seed + 1}/{iterations}', end='')
        gen_cmd = [
            cmd, "--json", "--rounds",
            str(rounds), "--seed",
            str(prng_seed), seed
        ]

        proc = subprocess.Popen(gen_cmd,
                                stdout=PIPE,
                                stderr=subprocess.DEVNULL,
                                text=True)

        output = subprocess.check_output(runner, stdin=proc.stdout,
                                         text=True).splitlines()
        proc.wait()

        for line in output:
            outputs.append(json.loads(line))

    print()

    return outputs


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description='Repeatedly runs formula generator with different seeds')

    parser.add_argument('--seeds',
                        help='Folder with seed formulas',
                        type=Path,
                        required=True)
    parser.add_argument('--gen',
                        help='Generator executable',
                        type=Path,
                        required=True)
    parser.add_argument(
        '--iterations',
        help='How many times to run the command with different seeds',
        type=int,
        required=True)
    parser.add_argument('--rounds',
                        help='--rounds argument of generator',
                        type=int,
                        required=True)
    parser.add_argument('remaining',
                        nargs=argparse.REMAINDER,
                        help='Solver command')

    args = parser.parse_args()

    cmd = args.remaining

    if len(cmd) > 0 and cmd[0] == '--':
        cmd = cmd[1:]

    if len(cmd) == 0:
        eprint("No command given")
        sys.exit(1)

    seeds = list(args.seeds.glob("*.smt2"))

    if len(seeds) == 0:
        eprint(f"No seeds found in {args.seeds.resolve()}")
        sys.exit(1)

    runner_cmd = Path(__file__).parent / "runner.py"

    outputs = []

    for seed in seeds:
        outputs += run(args.gen, [runner_cmd, "--"] + cmd, seed, args.rounds,
                       args.iterations)

    for output in outputs:
        import pprint
        pprint.pprint(output)

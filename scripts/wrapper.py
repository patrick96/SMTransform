#!/usr/bin/env python3

import argparse
import subprocess
import sys

from pathlib import Path
from subprocess import PIPE

import runner
from runner import RunResult, ResultKind

def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def run(cmd: Path, solver: [str], seed: Path, rounds: int,
        iterations: int) -> [RunResult]:

    run_results = []

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

        for line in proc.stdout:
            run_result = runner.on_input(solver, line)

            if run_result:
                run_results.append(run_result)

        proc.wait()

    print()

    return run_results


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description='Repeatedly runs formula generator with different seeds')

    parser.add_argument('--seeds',
                        help='Folder with seed formulas',
                        type=Path,
                        required=True)
    parser.add_argument('--out',
                        help='Folder for dumping files, must be empty',
                        type=Path,
                        required=False)
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

    out : Path = args.out

    if out is not None:
        if not out.exists():
            out.mkdir(parents=True)

        if not out.is_dir():
            eprint(f"Output folder '{out}' is not a folder")
            sys.exit(1)

        if not out.is_dir():
            eprint(f"Output folder '{out}' is not a folder")
            sys.exit(1)

        if any(out.iterdir()):
            eprint(f"Output folder '{out}' is not empty")
            sys.exit(1)


    results: dict[ResultKind, int] = {}

    for kind in list(ResultKind):
        results[kind] = 0

    for seed in seeds:
        run_results: [RunResult] = run(args.gen, cmd, seed, args.rounds,
                                       args.iterations)

        for run_result in run_results:
            kind = run_result.get_kind()
            results[kind] += 1

            if kind != ResultKind.Success:
                dir = out / kind.name
                dir.mkdir(exist_ok=True)

                run_result.dump(dir)


    for (kind, num_results) in results.items():
        print(f'{kind.name}: {num_results}')

    runs_per_iter = args.rounds + 1
    runs_per_seed = args.iterations * runs_per_iter
    num_seeds = len(seeds)
    total_runs = num_seeds * runs_per_seed

    print(f'Runs per iteration: {runs_per_iter}')
    print(f'Runs per seed: {runs_per_seed}')
    print(f'Seeds: {num_seeds}')
    print(f'Total runs: {total_runs}')

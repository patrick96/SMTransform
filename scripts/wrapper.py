#!/usr/bin/env python3

import argparse
import subprocess
import sys

from pathlib import Path
from subprocess import PIPE
from collections import Counter

import runner
from runner import RunResult, ResultKind


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def dump(result: RunResult, out: Path):
    # Only dump if a path was specified
    if not out:
        return

    kind = result.get_kind()

    # Don't drop success or timeout results (they're not interesting)
    if kind in [ResultKind.Success, ResultKind.Timeout]:
        return

    dir = out / kind.name
    dir.mkdir(exist_ok=True, parents=True)

    with open(dir / (result.input.id() + ".json"), "w") as f:
        f.write(result.to_json())


def run(cmd: Path, solver: [str], seed: Path, rounds: int,
        iterations: int, out: Path) -> [RunResult]:

    run_results: Counter[ResultKind, int] = Counter({})

    for iter in range(iterations):
        prng_seed = iter
        print(
            f'\33[2K\r\rUsing seed {seed} {prng_seed + 1}/{iterations}: ', end='')
        gen_cmd = [
            cmd, "--json", "--rounds",
            str(rounds), "--seed",
            str(prng_seed), seed
        ]

        with subprocess.Popen(gen_cmd, stdout=PIPE, stderr=subprocess.DEVNULL, text=True) as proc:
            for line in proc.stdout:
                run_result = runner.on_input(solver, line)
                run_results += Counter({run_result.get_kind(): 1})
                dump(run_result, out)
                print(run_result.get_char(), end='', flush=True)

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

    out: Path = args.out

    if out is not None:
        if not out.exists():
            out.mkdir(parents=True)

        if not out.is_dir():
            eprint(f"Output folder '{out}' is not a folder")
            sys.exit(1)

        if any(out.iterdir()):
            eprint(f"Output folder '{out}' is not empty")
            sys.exit(1)

    else:
        eprint("Warning: Running without dumping of results!")

    results: Counter[ResultKind, int] = Counter({})

    try:
        for seed in seeds:
            results += run(args.gen, cmd, seed, args.rounds, args.iterations, out)
    except KeyboardInterrupt:
        print("Received keyboard interrupt")

    for kind in list(ResultKind):
        print(f'{kind.name}: {results.get(kind, 0)}')

    runs_per_iter = args.rounds + 1
    runs_per_seed = args.iterations * runs_per_iter
    num_seeds = len(seeds)

    print(f'Runs per iteration (approx): {runs_per_iter}')
    print(f'Runs per seed (approx): {runs_per_seed}')
    print(f'Seeds: {num_seeds}')
    print(f'Total runs: {sum(1 for _ in results.elements())}')

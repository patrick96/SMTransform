#!/usr/bin/env python3
"""
Dispatches jobs on a cluster.

This was written with the paths and assumptions of the cluster where the
experiments were run.
"""

import itertools
import argparse
import os
import sys
import subprocess
from pathlib import Path


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


REPO_ROOT = Path(__file__).parent.parent.relative_to(Path.cwd())

WRAPPER = REPO_ROOT / "scripts" / "wrapper.py"
GEN = REPO_ROOT / "target" / "release" / "smtransform"

HOME = Path.home()
SOLVERS = HOME / 'smtransform' / 'solvers'


def get_bsub(num: int, seeds: Path, cmd: [str]) -> (str, [str]):
    jobname = f'job{num}'
    return jobname, [
        'bsub',
        '-W', '6:00',
        '-J', jobname,
        '-oo', f'out/out-{num}.log',
        '-eo', f'out/err-{num}.log',
        str(WRAPPER),
        '--seeds', str(seeds),
        '--out', f'out/{jobname}',
        '--gen', str(GEN),
        '--iterations', '20',
        '--rounds', '10',
        '--'
    ] + cmd


class Z3Runner:

    @staticmethod
    def get_exec(asan: bool) -> Path:
        folder_name = f'z3-{"asan-" if asan else ""}8c95dff33'
        return SOLVERS / folder_name / 'usr' / 'local' / 'bin' / 'z3'

    @staticmethod
    def get_cmd(asan: bool) -> (dict[str, str], [str]):
        return ({}, [str(Z3Runner.get_exec(asan)), '/dev/stdin'])


class Yices2Runner:

    @staticmethod
    def get_base(asan: bool) -> Path:
        folder_name = f'yices2-{"asan-" if asan else ""}09f16210'
        return SOLVERS / folder_name

    @staticmethod
    def get_exec(asan: bool) -> Path:
        return Yices2Runner.get_base(asan) / 'usr' / 'local' / 'bin' / 'yices-smt2'

    @staticmethod
    def get_cmd(asan: bool) -> (dict[str, str], [str]):
        ld_library_path = os.environ.get("LD_LIBRARY_PATH", "")
        ld_library_path += f":{HOME}/smtransform/libpoly/build/pkg/usr/local/lib"
        ld_library_path += f":{HOME}/smtransform/cudd/cudd/.libs"
        return (
            {"LD_LIBRARY_PATH": ld_library_path},
            [str(Yices2Runner.get_exec(asan)), '/dev/stdin']
        )


class CVC5Runner:

    @staticmethod
    def get_base(asan: bool) -> Path:
        folder_name = f'cvc5-{"asan-" if asan else ""}43f6eb50c'
        return SOLVERS / folder_name

    @staticmethod
    def get_exec(asan: bool) -> Path:
        return CVC5Runner.get_base(asan) / 'usr' / 'local' / 'bin' / 'cvc5'

    @staticmethod
    def get_cmd(asan: bool) -> (dict[str, str], [str]):
        ld_library_path = os.environ.get("LD_LIBRARY_PATH", "")
        ld_library_path += f":{CVC5Runner.get_exec(asan).parent.parent / 'lib64'}"
        return (
            {"LD_LIBRARY_PATH": ld_library_path},
            [str(CVC5Runner.get_exec(asan)), '--lang', 'smt2.6', '-']
        )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Dispatches jobs on a cluster')

    parser.add_argument('--seeds',
                        help='Folder with subfolders, each containing seed formulas',
                        type=Path,
                        required=True)

    parser.add_argument('-n', '--dry-run',
                        help="Don't submit any jobs",
                        action=argparse.BooleanOptionalAction,
                        required=False)

    parser.add_argument('--nums',
                        help="Only run jobs with the given numbers (can be ranges as well)",
                        nargs='+',
                        type=str,
                        required=False)

    args = parser.parse_args()

    runners = [Z3Runner, Yices2Runner, CVC5Runner]

    num = 0

    seeds: Path = args.seeds
    dry_run: bool = args.dry_run

    job_numbers = set()

    for nums_str in args.nums:
        split = nums_str.split(',')

        if len(split) == 1:
            job_numbers.add(int(split[0]))
        elif len(split) == 2:
            job_numbers |= set(range(int(split[0]), int(split[1]) + 1))
        elif len(split) == 3:
            job_numbers |= set(range(int(split[0]), int(split[1]) + 1, int(split[2])))
        else:
            eprint(f'"{nums_str}" is not a valid range')
            sys.exit(1)

    if job_numbers:
        eprint(f'Only running jobs: {job_numbers}')
    else:
        eprint("Running all jobs")

    if dry_run:
        eprint("Dry-run mode")

    for (runner, asan) in itertools.product(runners, [True, False]):
        for seed_folder in sorted(seeds.iterdir()):
            if not seed_folder.is_dir():
                continue

            if not job_numbers or num in job_numbers:
                (env, solver_cmd) = runner.get_cmd(asan)
                (name, cmd) = get_bsub(num, seed_folder, solver_cmd)

                print(
                    f'{name}: {runner.__name__} {"ASAN" if asan else "RELEASE"}, {seed_folder}, env: {env}')

                my_env = os.environ.copy()
                my_env.update(env)

                print(' '.join(cmd))
                if not dry_run:
                    subprocess.run(cmd, check=True, env=my_env, stdout=sys.stderr)

            num += 1

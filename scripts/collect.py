#!/usr/bin/env python3

import mmap
import argparse
import sys
import shutil
from pathlib import Path

# 10KB
MAX_SIZE = 10 * 1024


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description='Collect SMT formulas from a folder')

    parser.add_argument('--out',
                        help='Folder for copying files, must be empty',
                        type=Path,
                        required=True)
    parser.add_argument('path',
                        type=Path,
                        help='Directory with smtlib formulas')

    args = parser.parse_args()

    out = args.out

    if not out.exists():
        out.mkdir(parents=True)

    if not out.is_dir():
        eprint(f"Output folder '{out}' is not a folder")
        sys.exit(1)

    if any(out.iterdir()):
        eprint(f"Output folder '{out}' is not empty")
        sys.exit(1)

    input: Path = args.path

    for file in input.glob("**/*.smt2"):
        if not file.is_file() or file.is_symlink():
            continue

        # Skip files that are too large
        if file.stat().st_size >= MAX_SIZE:
            continue

        with open(file, 'rb', 0) as f:
            with mmap.mmap(f.fileno(), 0, access=mmap.ACCESS_READ) as s:
                # Skip files that are not satisfiable
                if s.find(b'set-info :status sat') == -1:
                    continue

        out_name = str(file.relative_to(input)).replace('/', '_')
        shutil.copy2(file, out / out_name)
        print(f'{out_name}')

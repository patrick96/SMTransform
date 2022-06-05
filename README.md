# SMTransform

*SMTransform* is framework with the goal to find unsoundness bugs in SMT
solvers through fuzzing.
From a satisfiable seed formula, *SMTransform* incrementally generates new
formulas which are satisfiable by construction.

## Installation

### Requirements

* [Cargo](https://github.com/rust-lang/cargo/)
* Python 3.9+

The project is used and developed on Linux, but it should work on any platform
supported by rust and python.

### Building

Compiling the project is as easy as running

```bash
cargo build --release
```

The `smtransform` executable will then be placed in `target/release/smtransform`.

## Usage

```
USAGE:
    smtransform [OPTIONS] <FILE>

ARGS:
    <FILE>    File containing seed formula in the SMT-LIB2 format

OPTIONS:
    -h, --help               Print help information
        --json               Print generated formulas wrapped in JSON object
        --print-original     Also print the seed formula
        --rounds <ROUNDS>    How many formulas to generate [default: 1]
        --seed <SEED>        Seed for the PRNG [default: 0]
    -V, --version            Print version information
```

Sample usage:

```
$ cat in.smt2
(set-info :status sat)
(set-logic QF_NIA)
(declare-fun x () Int)
(declare-fun y () Int)
(assert (= x 3))
(assert (= y 9))
(check-sat)
(exit)

$ ./target/release/smtransform in.smt2
(set-logic QF_NIA)
(set-info :status sat)
(declare-fun x () Int)
(declare-fun y () Int)
(declare-fun v0 () Int)
(assert (= (- v0 y) 3))
(assert (= (- v0 x) 9))
(check-sat)
(exit)
```

A large collection of seed formulas is available in the [SMT-LIB benchmark
repository](https://clc-gitlab.cs.uiowa.edu:2443/SMT-LIB-benchmarks).

### Helper Scripts

#### `scripts/runner.py`

Pipe the JSON output from the `smtransform` executable into the runner script:

```bash
./target/release/smtransform --json in.smt2 | ./scripts/runner.py -- z3 /dev/stdin
```

This will run each generated formula through the given SMT solver (the solver
command should accept a formula on stdin) and report each failure.

### `scripts/wrapper.py`

Runs the generator for multiple seed formulas, passes each formula to
`runner.py`, and collects failures in the given output directory.

```
$ ./scripts/wrapper.py \
    --seeds seeds \
    --out out \
    --gen target/release/smtransform \
    --iterations 20 \
    --rounds 10 \
    -- z3 /dev/stdin
Using seed seeds/20170427-VeryMax_ITS_From_T2__ex36.t2_fixed__p23162_safety_0.smt2 20/20: T..........
Using seed seeds/20170427-VeryMax_ITS_From_T2__ex36.t2_fixed__p23504_safety_0.smt2 20/20: TTTTT......
Using seed seeds/20170427-VeryMax_ITS_From_T2__s1.t2_fixed__p15649_safety_0.smt2 20/20: ...........
Using seed seeds/20170427-VeryMax_SAT14_279.smt2 20/20: T..........
Using seed seeds/AProVE_aproveSMT1179149253451050796.smt2 20/20: ..T......T.
Using seed seeds/AProVE_aproveSMT2322179511678559502.smt2 20/20: ....TTT.TTT
Using seed seeds/AProVE_aproveSMT2821114236865559656.smt2 20/20: T......TT..
Using seed seeds/AProVE_aproveSMT4248959283647504950.smt2 20/20: ..........T
Using seed seeds/AProVE_aproveSMT4556648541890562734.smt2 20/20: .T.........
Using seed seeds/AProVE_aproveSMT5479381539249843197.smt2 20/20: ......TTTTT
Success: 1611
Failure: 3
Timeout: 586
Signal: 0
Unknown: 0
Unsound: 0
Runs per iteration (approx): 11
Runs per seed (approx): 220
Seeds: 10
Total runs: 2200
```

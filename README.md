# SMTransform
Find soundness bugs in SMT solvers through equivalent transformations

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

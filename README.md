# pyage-rust Python module for age

[![CircleCI branch](https://img.shields.io/circleci/project/github/kushaldas/pyage/master.svg)](https://circleci.com/gh/kushaldas/workflows/pyage/tree/master)

age is a simple, secure and modern encryption tool with small explicit keys, no
config options, and UNIX-style composability. The format specification is at
[age-encryption.org/v1](https://age-encryption.org/v1).

To discuss the spec or other age related topics, please email
[the mailing list](https://groups.google.com/d/forum/age-dev) at
age-dev@googlegroups.com. age was designed by
[@Benjojo12](https://twitter.com/Benjojo12) and
[@FiloSottile](https://twitter.com/FiloSottile).

The reference interoperable Golang implementation is available at
[filippo.io/age](https://filippo.io/age).

This Python module is written using the Rust crate [age](https://crates.io/crates/age).

**NOTE:** Please do not use in production, this module is getting heavily developed.


![demo](https://kushaldas.in/images/pyage-rust1.gif)

## Installation

```
python3 -m pip install pyage-rust
```

Right now there are prebuild wheels available only for Python 3.7 and Python 3.8 in Linux and MacOS.


## How to build?

You will need Rustup nightly toolchain from https://rustup.rs

```
python3 -m venv .venv
source .venv/bin/activate
python3 -m pip install requirements-dev.txt
maturin build
```

The above commands will create a wheel file in `target/wheels` directory.

If you want to build for development only then use `maturin develop` command.

## Usage

Please go through the [documentation](https://pyage-rust.readthedocs.io/en/latest/) for usage.


## LICENSE: MIT

Check the LICENSE file for details.

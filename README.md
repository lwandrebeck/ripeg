# ripeg
**Incremental packrat Parsing Expression Grammar in Rust**

[![Crates.io](https://img.shields.io/crates/v/ripeg)](https://crates.io/crates/ripeg)
[![docs](https://img.shields.io/docsrs/ripeg)](https://docs.rs/ripeg/latest/ripeg/)
![maintained](https://img.shields.io/badge/Maintained%3F-yes-green.svg)

[![ripeg Continuous Integration](https://github.com/lwandrebeck/ripeg/actions/workflows/rust.yml/badge.svg)](https://github.com/lwandrebeck/ripeg/actions/workflows/rust.yml)
[![Coverage Status](https://coveralls.io/repos/github/lwandrebeck/ripeg/badge.svg?branch=main)](https://coveralls.io/github/lwandrebeck/ripeg?branch=main)
[![codecov](https://codecov.io/gh/lwandrebeck/ripeg/branch/main/graph/badge.svg?token=QCVCQMLQP2)](https://codecov.io/gh/lwandrebeck/ripeg)

![downloads](https://img.shields.io/crates/d/ripeg)
![GitHub contributors](https://img.shields.io/github/contributors/lwandrebeck/ripeg)
![commits](https://img.shields.io/github/commit-activity/m/lwandrebeck/ripeg)

[![License: GPL v3+](https://img.shields.io/badge/License-GPL%20v3+-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.59+-blue.svg)](#rust-version-requirements)
[![dependency status](https://deps.rs/repo/github/lwandrebeck/ripeg/status.svg)](https://deps.rs/repo/github/lwandrebeck/ripeg)

[![issues](https://img.shields.io/github/issues/lwandrebeck/ripeg.svg)](https://github.com/lwandrebeck/ripeg/issues)
[![closed](https://img.shields.io/github/issues-closed/lwandrebeck/ripeg.svg)](https://github.com/lwandrebeck/ripeg/issues?q=is%3Aissue+is%3Aclosed)
[![pr](https://img.shields.io/github/issues-pr/lwandrebeck/ripeg.svg)](https://github.com/lwandrebeck/ripeg/pulls)
[![prc](https://img.shields.io/github/issues-pr-closed/lwandrebeck/ripeg.svg)](https://github.com/lwandrebeck/ripeg/pulls?q=is%3Apr+is%3Aclosed)

This project intends to be a Rust port of https://github.com/zyedidia/gpeg/ which is itself inspired by http://www.inf.puc-rio.br/~roberto/lpeg/. One of existing research publications about incremental PEG is https://zyedidia.github.io/preprints/gpeg_sle21.pdf

## Contributing
PRs are more than welcome, I’d like a lot this project not to be a one-random-guy work. Please read [Contributing](https://github.com/lwandrebeck/ripeg/blob/main/CONTRIBUTING.md)

## Code of Conduct
Please follow the [CoC from rust-lang](https://www.rust-lang.org/policies/code-of-conduct). I’ll be glad if I can avoid any kind of moderation, I have better to do with my time, thanks !

## Current status
Nothing to see here yet. CI and such are being configured before code development begins.

## Planned Roadmap
* 20220319 0.1.0 : ripeg crate published so name is reserved.
* 202203?? 0.1.1 : release with CI, code coverage configured, and use of cargo-nextest.
* 2022???? 0.1.2 : release with functions/methods prototypes and todo!(); bodies.
* 202????? 0.1.3..n: replace todo!(); function/methods bodies with real code.
* 202????? 0.2.0 : 1st usable version.
* 202????? 0.x.y : optimize
* 202????? 1.0.0 : stable release.
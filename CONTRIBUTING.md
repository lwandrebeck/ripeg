# Contributing to ripeg

Thanks a lot for contributing to this project!

The following is a set of guidelines for contributing to ripeg.

**Since the project is young**: consider those best practices prone to change. Please suggest improvements!

## Basics

### License

The project uses the [GPL3](http://www.gnu.org/licenses/gpl-3.0.html) license. By contributing to this project you agree to license
your changes under this license.

## What to do

### Issues

There is plenty of [features missing](https://github.com/lwandrebeck/ripeg/labels/enhancement) (I’ll try to create such issues soon) and possibly bugs might be already there. Feel free to add new [issues](https://github.com/lwandrebeck/ripeg/issues)
and to wrangle over those already [open](https://github.com/lwandrebeck/ripeg/issues?q=is%3Aissue+is%3Aopen+) and help fixing them.

### Code

Only one module has been implemented for now, feel free to submit your PRs. Work is ongoing on isa module now, if that can help to prevent duplicate work.

### Tests

Tests are mandatory for a PR to be accepted. We want coverage to flirt with 100%. Luckily for us, having a complete test coverage is far easier to get with a brand new project.

Please Add your tests as doctests for *every* method.

### Benchmark

Benches are mandatory for a PR to be accepted. We want coverage to flirt with 100%. Luckily for us, having a complete bench coverage is far easier to get with a brand new project.

### Documentation

Complete documentation is mandatory for a PR to be accepted. We want doc to be complete so newcomers feel quickly at home when getting their hands on ripeg. Luckily for us, having a complete doc coverage is far easier to get with a brand new project.

## Style

### Issue style

Try to write at least 3 short paragraphs describing what were you trying to achieve, what is not working and the step by step actions that lead to the unwanted outcome.

If possible provide:

- a code snippet or a link to a [gist](https://gist.github.com) showcasing the problem, if is a library usage issue.
- a backtrace, if it is a crash.
- a sample file, if it is a decoding or encoding issue.

### Coding style

The normal rust coding style is checked by [rustfmt](https://github.com/rust-lang-nursery/rustfmt).
Readable code is the first step on having good and safe libraries.

To avoid slight differences appearing in nightly versions, please
use the following command to run rustfmt: `cargo +stable fmt` before submitting a PR.

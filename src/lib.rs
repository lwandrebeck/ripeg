// Copyright (C) 2022 Laurent Wandrebeck
//
// This file is part of ripeg.
//
// ripeg is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ripeg is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ripeg.  If not, see <http://www.gnu.org/licenses/>.

//! ripeg. Incremental packrat Parsing Expression Grammar in Rust.
//!
//! ripeg is a general purpose parser written in Rust, with a focus on accessibility, correctness,
//! and performance. It uses parsing expression grammar (or [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar)) as input, which are similar in
//! spirit to regular expressions, but which offers the enhanced expressivity needed to parse
//! complex languages. Its incremental packrat parsing (optionnal) feature makes it very suitable for IDEs.
//! Regular expressions support is not planned.
//!
//! ## Getting started
//!
//! The recommanded way to start parsing with ripeg is to read the official [book](https://lwandrebeck.github.io/ripeg)
//!
//! Other helpful resources:
//!
//! * API reference on [docs.rs](https://docs.rs/ripeg)
//!

// Any PR emitting warnings when it comes to documentation won’t be accepted.
#[warn(missing_docs)]
#[warn(rustdoc::missing_doc_code_examples)]
pub mod charset;
pub mod isa;

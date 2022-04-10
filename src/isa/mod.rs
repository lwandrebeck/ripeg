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

//! isa provides types for all instructions of ripeg VM. aka
//! Instruction Set Architecture.

use crate::charset::NormalSet;

/// Instruction Pointer
/// Can get either next instruction address to be executed or None (failure state)
pub enum IP {
    /// Fail state
    None,
    /// actual position in the current program
    Index(usize),
}

/// List of Instructions
pub enum Instr {
    /// Advances ip and consume n u8 from subject if possible, fails otherwise.
    /// Fails only by reaching the end of the subject.
    Any(usize),
    /// pops backtrack entry from the stack, updates sp and jumps to usize
    BackCommit(usize),
    /// Pushes the next [`IP`] to the stack as a return address and jumps to usize.
    /// Used to implement non-terminals.
    Call(usize),
    /// Increment [`IP`] and Consume u8 if it matches, ip becomes None otherwise.
    Char(u8),
    /// Pushes a backtrack entry storing u8 and sp so that parser can backtrack
    /// to this position later and parse a different pattern.
    Choice(usize),
    /// Pops the top entry off the stack and jumps to usize. Allows the machine to
    /// commit to a state and discard a backtrack entry.
    Commit(usize),
    /// Ends matching and accepts the subject.
    End,
    /// Ends matching and rejects the subject.
    EndFail,
    /// Sets ip to None.
    Fail,
    /// pops top entry from the stack and sets IP to None
    FailTwice,
    /// Sets ip Index to usize.
    Jump(usize),
    /// used to mark a location in the instruction code with an unique ID.
    /// Maybe we could get rid of that ?
    /// Likely to be deleted as Label will probably become a HashMap<Label, Program[index]
    Label(usize),
    /// No operation
    /// Likely to be deleted ?
    Nop,
    /// Commit and Choice in 1 instruction
    PartialCommit(usize),
    /// Pops a return address from the stack and jumps to it.
    Return,
    /// advances ip and consumes u8 from subject if contained in character set [`NormalSet`].
    /// Goes to [`Instr::Fail`] state otherwise.
    Set(NormalSet),
    /// [`Instr::Span`] equals to [`NormalSet`]*
    Span(NormalSet),
    /// checks if there is at least 1st arg characters remaining -> pushes backtrack
    /// entry and advances sp by 1st arg. if not jumps to 2nd arg.
    TestAny(usize, usize),
    /// checks if 1st arg matches at current sp -> pushes backtrack entry on stack
    /// and advances sp. if not, jumps to 2nd arg.
    TestChar(u8, usize),
    /// checks if 1st arg matches at sp -> advances sp. if not jumps to 2nd arg.
    TestCharNoChoice(u8, usize),
    /// checks if 1st arg (Set) matches at current sp -> pushes backtrack entry on stack
    /// and advances sp. if not, jumps to 2nd arg.
    TestSet(NormalSet, usize),
    /// Checks if NormalSet matches at sp -> advances sp. If not jumps to 2nd arg.
    TestSetNoChoice(NormalSet, usize),
}

/// A Program is a Vector of Instructions.
/// struct used here because type does not allow to use impl.
pub struct Program(Vec<Instr>);

/// Program is instanciated by default without arguments, so define
/// a Default implementation.
impl Default for Program {
    /// Clippy says so.
    fn default() -> Self {
        Program::new()
    }
}

/// Methods for a [`Program`]
impl Program {
    /// Instanciate a new [`Program`]
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::isa::Program;
    /// let p = Program::new();
    /// assert_eq!(p.size(), 0);
    /// ```
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Size of a Program (# of Instr minus Label and Nop)
    /// Likely to change as Label will probably become a HashMap<Label, Program[index]
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::isa::Program;
    /// let p = Program::new();
    /// assert_eq!(p.size(), 0);
    /// ```
    pub fn size(&self) -> usize {
        let mut s = 0;
        for i in &self.0 {
            match i {
                Instr::Label(_) | Instr::Nop => continue,
                _ => s += 1,
            }
        }
        s
    }
}

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

//! charset modules provides data types and methods for managing sets of characters.

use std::ops::Not;

use bitvec::prelude::*;

/// [`NormalSet`] structure represents a set of chars
///
/// 256 bits, one for each possible character value. Little endian (Lsb0)
pub struct NormalSet {
    /// 256 bits vector
    pub bits: BitVec<u8, Lsb0>,
}

/// [`SmallSet`] structure represents only the ASCII set of chars
///
/// 128 bits, one for each possible ASCII character value. Little endian (Lsb0)
pub struct SmallSet {
    /// 128 bits vector
    pub bits: BitVec<u8, Lsb0>,
}

/// [`Set`] trait defines common methods available for [`NormalSet`] and [`SmallSet`]
pub trait Set {
    /// Checks if a [`Set`] contains a character
    fn has(&self, r: u8) -> bool;
    /// Instanciates [`Set`] with a given charset
    fn new(chars: &[u8]) -> Self;
    /// Counts number of bits with value 1 in the bit vector which is the number of characters matched by [`Set`]
    fn size(&self) -> usize;
}

/// Common methods between [`SmallSet`] and [`NormalSet`]
impl Set for SmallSet {
    /// Checks if a [`SmallSet`] contains a character
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::SmallSet;
    /// use bitvec::prelude::*;
    /// let s = SmallSet::new(&[67u8, 68, 69]); // C, D, E ASCII decimal value
    /// assert_eq!(s.has(66), false);
    /// assert_eq!(s.has(67), true);
    /// assert_eq!(s.has(250), false);
    /// ```
    fn has(&self, r: u8) -> bool {
        if (r as usize) < self.bits.len() {
            self.bits[r as usize]
        } else {
            false
        }
    }

    /// Instanciate a [`SmallSet`] with a given charset
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::SmallSet;
    /// use bitvec::prelude::*;
    /// let s = SmallSet::new(&[67u8, 68, 69]); // C, D, E ASCII decimal value
    /// assert_eq!(s.has(66), false);
    /// assert_eq!(s.has(67), true);
    /// ```
    fn new(chars: &[u8]) -> Self {
        let mut s = Self {
            bits: bitvec![u8, Lsb0; 0; 128],
        };
        for i in chars {
            s.bits.set(*i as usize, true);
        }
        s
    }

    /// Count number of bits with value 1 in the bit vector which is the number of characters matched by a [`SmallSet`]
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::SmallSet;
    /// use bitvec::prelude::*;
    /// let s = SmallSet::new(&[67u8, 68, 69]); // C, D, E ASCII decimal value
    /// assert_eq!(s.size(), 3);
    /// ```
    fn size(&self) -> usize {
        self.bits.count_ones()
    }
}

/// Common methods between [`SmallSet`] and [`NormalSet`]
impl Set for NormalSet {
    /// Checks if a [`NormalSet`] contains a character
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::NormalSet;
    /// use bitvec::prelude::*;
    /// let s = NormalSet::new(&[67u8, 68, 69, 247]); // C, D, E, ÷ ASCII decimal value
    /// assert_eq!(s.has(66), false);
    /// assert_eq!(s.has(67), true);
    /// assert_eq!(s.has(246), false);
    /// assert_eq!(s.has(247), true);
    /// ```
    fn has(&self, r: u8) -> bool {
        self.bits[r as usize]
    }

    /// Instanciate [`NormalSet`] with a given charset
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::NormalSet;
    /// use bitvec::prelude::*;
    /// let s = NormalSet::new(&[67u8, 68, 69, 247]); // C, D, E, ÷ ASCII decimal value
    /// assert_eq!(s.has(66), false);
    /// assert_eq!(s.has(67), true);
    /// assert_eq!(s.has(246), false);
    /// assert_eq!(s.has(247), true);
    /// ```
    fn new(chars: &[u8]) -> Self {
        let mut s = Self {
            bits: bitvec![u8, Lsb0; 0; 256],
        };
        for i in chars {
            s.bits.set(*i as usize, true);
        }
        s
    }

    /// Count number of bits with value 1 in the bits vector which is the number of characters matched by [`NormalSet`]
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::NormalSet;
    /// use bitvec::prelude::*;
    /// let s = NormalSet::new(&[67u8, 68, 69, 247]); // C, D, E, ÷ ASCII decimal value
    /// assert_eq!(s.size(), 4);
    /// ```
    fn size(&self) -> usize {
        self.bits.count_ones()
    }
}

/// Implementations of functions reserved to [`NormalSet`]
impl NormalSet {
    /// Adds a [`NormalSet`] to the existing one (binary OR operation)
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::NormalSet;
    /// use bitvec::prelude::*;
    /// let mut s = NormalSet::new(&[67u8, 68, 69, 247]); // C, D, E, ÷ ASCII decimal value
    /// let s2 = NormalSet::new(&[65u8, 66, 247]); // A, B, ÷ ASCII decimal value
    /// s.add(s2);
    /// assert_eq!(s.size(), 6); // [65u8, 66, 67, 68, 69, 247]
    /// ```
    pub fn add(&mut self, s2: NormalSet) {
        self.bits |= s2.bits;
    }

    /// Returns all non-matched characters of a [`NormalSet`]
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::NormalSet;
    /// use bitvec::prelude::*;
    /// let s = NormalSet::new(&[67u8, 68, 69, 247]); // C, D, E, ÷ ASCII decimal value
    /// let s2 = s.complement();
    /// assert_eq!(s2.size(), 252); // 256-4
    /// ```
    pub fn complement(&self) -> NormalSet {
        let mut s = Self {
            bits: bitvec![u8, Lsb0; 0; 256],
        };
        //s.bits = !self.bits;
        for i in 0usize..256 {
            s.bits.set(i, self.bits.get(i).unwrap().not());
        }
        s
    }

    /// Returns true if [`NormalSet`] can be converted to a [`SmallSet`]
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::NormalSet;
    /// use bitvec::prelude::*;
    /// let s = NormalSet::new(&[67u8, 68, 69, 247]); // C, D, E, ÷ ASCII decimal value
    /// let s2 = NormalSet::new(&[65u8, 127]);
    /// assert_eq!(s.is_small(), false);
    /// assert_eq!(s2.is_small(), true);
    /// ```
    pub fn is_small(&self) -> bool {
        self.bits[128..256].count_ones() == 0
    }

    /// Returns a [`NormalSet`] matching all characters between low and high inclusive
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::NormalSet;
    /// use bitvec::prelude::*;
    /// let s = NormalSet::range(48, 57); // 0 to 9 in ASCII decimal value
    /// assert_eq!(s.size(), 10);
    /// assert_eq!(s.is_small(), true);
    /// assert_eq!(s.has(47), false);
    /// assert_eq!(s.has(48), true);
    /// assert_eq!(s.has(52), true);
    /// assert_eq!(s.has(57), true);
    /// assert_eq!(s.has(58), false);
    /// ```
    pub fn range(low: u8, high: u8) -> NormalSet {
        let mut s = Self {
            bits: bitvec![u8, Lsb0; 0; 256],
        };
        for i in low..=high {
            s.bits.set(i as usize, true);
        }
        s
    }

    /// [NormalSet::smallset()] method converts a [`NormalSet`] into a [`SmallSet`]
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::{NormalSet, SmallSet};
    /// use bitvec::prelude::*;
    /// let s = NormalSet::range(120, 130);
    /// let s2 = s.smallset();
    /// assert_eq!(s.size(), 11);
    /// assert_eq!(s.is_small(), false);
    /// assert_eq!(s.has(119), false);
    /// assert_eq!(s.has(120), true);
    /// assert_eq!(s.has(130), true);
    /// assert_eq!(s.has(131), false);
    /// assert_eq!(s2.size(), 8); // 120..128
    /// assert_eq!(s2.has(119), false);
    /// assert_eq!(s2.has(120), true);
    /// assert_eq!(s2.has(127), true);
    /// assert_eq!(s2.has(128), false);
    /// assert_eq!(s2.has(131), false);
    /// ```
    pub fn smallset(&self) -> SmallSet {
        // 0..128
        SmallSet {
            bits: self.bits[0..128].to_bitvec(),
        }
    }

    /// [NormalSet::string()] returns the string represention of the charset
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::{NormalSet, SmallSet};
    /// use bitvec::prelude::*;
    /// let mut s = NormalSet::range(120, 130);
    /// let s2 = NormalSet::range(110, 115);
    /// s.add(s2);
    /// let output = s.string();
    /// assert_eq!(output, "{110..116,120..131}");
    /// ```
    pub fn string(&self) -> String {
        let mut s = String::new();
        let mut inrange = false;
        for b in 0..=255 {
            if self.has(b) && b == 255 {
                s += "\u{00ff}"; // C1 command. Unicode internal stuff.
            } else if self.has(b) && !inrange {
                inrange = true;
                if self.has(b + 1) {
                    s += &b.to_string();
                    s += "..";
                }
            } else if !self.has(b) && inrange {
                inrange = false;
                s += &b.to_string();
                s += ",";
            }
        }
        if !s.is_empty() && s.ends_with(',') {
            s.pop();
        }
        s = "{".to_owned() + &s + "}";
        s
    }

    /// [`NormalSet::sub()`] method substracts a [`NormalSet`] to the existing one (not operation)
    ///
    /// # Examples
    /// ```
    /// use crate::ripeg::charset::Set;
    /// use crate::ripeg::charset::{NormalSet, SmallSet};
    /// use bitvec::prelude::*;
    /// let s = NormalSet::range(120, 130); // size 11
    /// let s2 = NormalSet::range(125, 127); // size 3
    /// let s3 = s.sub(s2); // 120..125, 128..131
    /// assert_eq!(s3.size(), 11-3);
    /// assert_eq!(s3.is_small(), false);
    /// assert_eq!(s3.has(119), false);
    /// assert_eq!(s3.has(120), true);
    /// assert_eq!(s3.has(124), true);
    /// assert_eq!(s3.has(125), false);
    /// assert_eq!(s3.has(127), false);
    /// assert_eq!(s3.has(128), true);
    /// assert_eq!(s3.has(130), true);
    /// assert_eq!(s3.has(131), false);
    /// let s4 = NormalSet::range(110, 119);
    /// let s5 = s.sub(s4);
    /// assert_eq!(s5.size(), 11); // s4 contains characters not in s so nothing should change
    /// assert_eq!(s5.is_small(), false);
    /// assert_eq!(s5.has(109), false);
    /// assert_eq!(s5.has(110), false);
    /// assert_eq!(s5.has(119), false);
    /// assert_eq!(s5.has(120), true);
    /// ```
    pub fn sub(&self, s2: NormalSet) -> NormalSet {
        let mut s = Self {
            bits: bitvec![u8, Lsb0; 0; 256],
        };
        s.bits = !s2.bits;
        s.bits &= self.bits.to_bitvec();
        s
    }
}

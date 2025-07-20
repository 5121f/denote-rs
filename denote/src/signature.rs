/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

use crate::format;

/// Represent signature in denote name scheme
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Signature(String);

impl Signature {
    /// ```
    /// use zeroten_denote::Signature;
    ///
    /// assert_eq!(Signature::parse("1b 2b=3c").unwrap().to_string(), "==1b=2b=3c");
    /// ```
    pub fn parse(string: &str) -> Option<Self> {
        let string = format::format(string, "=");
        (!string.is_empty()).then_some(string).map(Self)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=={}", self.0)
    }
}

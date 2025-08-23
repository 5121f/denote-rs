// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use std::fmt::{self, Display};

use crate::format;

const PREFIX: &str = "==";
const SEPARATOR: char = '=';

/// Represent signature in denote name scheme
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Signature(String);

impl Signature {
    /// ```
    /// use zeroten_denote::Signature;
    ///
    /// assert_eq!(Signature::parse("1b 2b=3c").unwrap().to_string(), "==1b=2b=3c");
    /// ```
    pub fn parse<S: AsRef<str>>(string: S) -> Option<Self> {
        let string = format::slugify(string, Some(SEPARATOR));
        (!string.is_empty()).then_some(string).map(Self)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", PREFIX, self.0)
    }
}

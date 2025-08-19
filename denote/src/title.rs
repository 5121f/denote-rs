/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

use crate::format;

const PREFIX: &str = "--";
const SEPARATOR: char = '-';

/// Represent title in denote name scheme
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Title(String);

impl Title {
    /// ```
    /// use zeroten_denote::Title;
    ///
    /// assert_eq!(Title::parse(",Some  title ").unwrap().to_string(), "--some-title");
    /// assert_eq!(Title::parse("some-title").unwrap().to_string(), "--some-title");
    /// ```
    pub fn parse<S: AsRef<str>>(string: S) -> Option<Self> {
        let string = format::slugify(string, Some(SEPARATOR));
        (!string.is_empty()).then_some(string).map(Self)
    }

    /// ```
    /// use zeroten_denote::Title;
    ///
    /// assert_eq!(Title::parse(" some Title").unwrap().desluggify(), "Some title");
    /// ```
    pub fn desluggify(&self) -> String {
        let deslugify = self.0.clone().replace(&SEPARATOR.to_string(), " ");
        format::first_letter_uppercase(&deslugify)
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", PREFIX, self.0)
    }
}

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

use crate::format;

/// Represent keyword in denote name scheme
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Keywords(Vec<String>);

impl Keywords {
    /// ```
    /// use zeroten_denote::Keywords;
    ///
    /// assert_eq!(
    ///     Keywords::parse_user_input("word1,word2, word 3 ").unwrap().to_string(),
    ///     "__word1_word2_word3"
    /// );
    /// ```
    pub fn parse_user_input(string: &str) -> Option<Self> {
        Self::parse(string, ",")
    }

    /// ```
    /// use zeroten_denote::Keywords;
    ///
    /// assert_eq!(
    ///     Keywords::parse_schemed_string("word1_wor d2_").unwrap().to_string(),
    ///     "__word1_word2"
    /// );
    /// ```
    pub fn parse_schemed_string(string: &str) -> Option<Self> {
        Self::parse(string, "_")
    }

    fn parse(string: &str, separator: &str) -> Option<Self> {
        let keywords: Vec<_> = string
            .split(separator)
            .map(|s| format::format(s, ""))
            .filter(|s| !s.is_empty())
            .collect();
        (!keywords.is_empty()).then_some(keywords).map(Self)
    }
}

impl Display for Keywords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "__{}", self.0.join("_"))
    }
}

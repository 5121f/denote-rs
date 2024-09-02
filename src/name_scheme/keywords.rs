/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

use crate::utils;

#[derive(Default)]
pub struct Keywords(Vec<String>);

impl Keywords {
    pub fn parse_user_input(string: &str) -> Self {
        Self::parse(string, ",")
    }

    pub fn parse_schemed_string(string: &str) -> Self {
        Self::parse(string, "_")
    }

    pub fn parse(string: &str, separator: &str) -> Self {
        let keywords: Vec<_> = string
            .split(separator)
            .map(|s| utils::format(s, ""))
            .collect();
        Self(keywords)
    }
}

impl Display for Keywords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            return fmt::Result::Ok(());
        }
        write!(f, "__{}", self.0.join("_"))
    }
}

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

use crate::utils;

#[derive(Default)]
pub(crate) struct Keywords(Vec<String>);

impl Keywords {
    pub(crate) fn parse_from_user_input(string: &str) -> Option<Self> {
        let keywords: Vec<_> = string
            .split(',')
            .filter_map(|s| utils::format(s, "-"))
            .collect();
        if keywords.is_empty() {
            return None;
        }
        Some(Self(keywords))
    }

    pub(crate) fn parse(string: &str) -> Option<Self> {
        let keywords: Vec<_> = string
            .split("_")
            .filter(|k| k.is_empty())
            .map(ToString::to_string)
            .collect();
        if keywords.is_empty() {
            return None;
        }
        Some(Self(keywords))
    }
}

impl Display for Keywords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "__{}", self.0.join("_"))
    }
}

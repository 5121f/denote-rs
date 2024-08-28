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
    pub(crate) fn from_string(string: &str) -> Option<Self> {
        let keywords: Vec<_> = string
            .split(',')
            .filter_map(|s| utils::format(s, "-"))
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

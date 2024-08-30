/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

#[derive(Clone, Default)]
pub(crate) struct Extention(String);

impl Extention {
    pub(crate) fn from_string(ext: String) -> Option<Self> {
        if ext.is_empty() {
            return None;
        }
        Some(Self(ext))
    }
}

impl Display for Extention {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".{}", self.0)
    }
}

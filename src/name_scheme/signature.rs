/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

use crate::utils;

#[derive(Clone)]
pub struct Signature(String);

impl Signature {
    pub fn from_string(string: String) -> Self {
        Self(string)
    }

    pub fn parse(string: &str) -> Self {
        Self(utils::format(string, "="))
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            return fmt::Result::Ok(());
        }
        write!(f, "=={}", self.0)
    }
}

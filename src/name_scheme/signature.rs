/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

use crate::utils;

#[derive(Clone)]
pub(crate) struct Signature(String);

impl Signature {
    pub(crate) fn from_string(string: String) -> Self {
        Self(string)
    }

    pub(crate) fn parse(string: &str) -> Option<Self> {
        utils::format(string, "=").map(Self)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=={}", self.0)
    }
}

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

use crate::utils;

#[derive(Clone)]
pub(crate) struct Title(String);

impl Title {
    pub(crate) fn from_string(string: String) -> Self {
        Self(string)
    }

    pub(crate) fn parse(string: &str) -> Option<Self> {
        utils::format(string, "-").map(Self)
    }

    pub(crate) fn desluggify(&self) -> String {
        let deslugify = self.0.clone().replace('-', " ");
        utils::first_letter_uppercase(&deslugify)
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "--{}", self.0)
    }
}

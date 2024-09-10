/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

use crate::utils;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Title(String);

impl Title {
    /// ```
    /// use denote::Title;
    ///
    /// assert_eq!(Title::parse(",Some  title ").to_string(), "--some-title");
    /// assert_eq!(Title::parse("some-title").to_string(), "--some-title");
    /// ```
    pub fn parse(string: &str) -> Self {
        Self(utils::format(string, "-"))
    }

    /// ```
    /// use denote::Title;
    ///
    /// assert_eq!(Title::parse(" some Title").desluggify(), "Some title");
    /// ```
    pub fn desluggify(&self) -> String {
        let deslugify = self.0.clone().replace('-', " ");
        utils::first_letter_uppercase(&deslugify)
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            return fmt::Result::Ok(());
        }
        write!(f, "--{}", self.0)
    }
}

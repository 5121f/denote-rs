/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

#[derive(Clone, Default)]
pub struct Extention(String);

impl Extention {
    /// ```
    /// assert_eq!(Extention::new(String::from("ext")).to_string(), ".ext");
    /// ```
    pub fn new(ext: String) -> Self {
        Self(ext.trim().to_string())
    }
}

impl Display for Extention {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            return fmt::Result::Ok(());
        }
        write!(f, ".{}", self.0)
    }
}

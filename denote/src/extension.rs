/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};
use std::path::Path;

/// Represent file extension
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Extension(String);

impl Extension {
    pub fn new(ext: &str) -> Option<Self> {
        let ext = ext.trim();
        let ext = ext.strip_prefix(".").unwrap_or(ext);
        if ext.is_empty() {
            return None;
        }
        Some(Self(ext.to_string()))
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path = path.as_ref();
        let ext = path.extension()?.to_str()?;
        Some(Self(ext.to_string()))
    }
}

impl Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".{}", self.0)
    }
}

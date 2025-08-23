// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use std::fmt::{self, Display};
use std::path::Path;

/// Represent file extension
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Extension(String);

impl Extension {
    pub fn new<S: AsRef<str>>(ext: S) -> Option<Self> {
        fn inner(ext: &str) -> Option<Extension> {
            let ext = ext.trim();
            let ext = ext.strip_prefix(".").unwrap_or(ext);
            if ext.is_empty() {
                return None;
            }
            Some(Extension(ext.to_string()))
        }
        inner(ext.as_ref())
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        fn inner(path: &Path) -> Option<Extension> {
            let ext = path.extension()?.to_str()?;
            Some(Extension(ext.to_string()))
        }

        inner(path.as_ref())
    }
}

impl Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".{}", self.0)
    }
}

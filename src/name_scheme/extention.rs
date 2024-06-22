/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[derive(Default)]
pub(crate) struct Extention(String);

impl Extention {
    pub(crate) fn new(ext: String) -> Option<Self> {
        if ext.is_empty() {
            return None;
        }
        Some(Self(ext))
    }

    pub(crate) fn to_string(&self) -> String {
        format!(".{}", self.0)
    }
}

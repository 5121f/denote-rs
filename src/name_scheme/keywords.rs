/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[derive(Default)]
pub(crate) struct Keywords(Vec<String>);

impl Keywords {
    pub(crate) fn from_string(string: &str) -> Option<Self> {
        let keywords: Vec<_> = string
            .trim()
            .to_lowercase()
            .split(',')
            .map(ToOwned::to_owned)
            .collect();
        let keywords = match keywords.first() {
            Some(first) if first.is_empty() => return None,
            _ => keywords,
        };
        Some(Self(keywords))
    }
}

impl ToString for Keywords {
    fn to_string(&self) -> String {
        format!("__{}", self.0.join("_"))
    }
}

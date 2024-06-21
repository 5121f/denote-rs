/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[derive(Default)]
pub(crate) struct Keywords(Vec<String>);

impl Keywords {
    pub(crate) fn from_string(string: &str) -> Self {
        let keywords: Vec<_> = string
            .trim()
            .to_lowercase()
            .split(',')
            .map(ToOwned::to_owned)
            .collect();
        let keywords = match keywords.first() {
            Some(first) if first.is_empty() => Vec::new(),
            _ => keywords,
        };
        Self(keywords)
    }

    pub(crate) fn into_string(self) -> Option<String> {
        if self.0.is_empty() {
            return None;
        }
        Some(format!("__{}", self.0.join("_")))
    }
}

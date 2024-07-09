/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use regex::Regex;

const TITLE_REGEXP: &str = r"--([\p{Alphabetic}\pN-]*)";
const PUNCTUATION: &str = r"\p{P}";

pub(crate) struct Title(String);

impl Title {
    pub(crate) fn parse(string: &str) -> Result<Option<Self>> {
        let punctuation = Regex::new(PUNCTUATION)?;
        let string = punctuation.replace_all(string, "");
        let string = string.trim().to_lowercase().replace(' ', "-");
        if string.is_empty() {
            return Ok(None);
        }
        Ok(Some(Self(string)))
    }

    pub(crate) fn find_in_string(string: &str) -> Result<Option<Self>> {
        let capture = Regex::new(TITLE_REGEXP)?.captures(string);
        let Some(capture) = capture else {
            return Ok(None);
        };
        let title = Self(capture[1].to_owned());
        Ok(Some(title))
    }

    pub(crate) fn desluggify(&self) -> String {
        let deslugify = self.0.clone().replace('-', " ");
        first_letter_uppercase(&deslugify)
    }

    pub(crate) fn into_string(self) -> String {
        format!("--{}", self.0)
    }
}

fn first_letter_uppercase(string: &str) -> String {
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Regex: {0}")]
    Regex(#[from] regex::Error),
}

type Result<T> = std::result::Result<T, Error>;

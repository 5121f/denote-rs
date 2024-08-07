/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use regex::Regex;

use crate::utils;

const SIGNATURE_REGEXP: &str = r"==([\p{Alphabetic}\pN=]*)";

pub(crate) struct Signature(String);

impl Signature {
    pub(crate) fn parse(string: &str) -> Result<Option<Self>> {
        let string = utils::remove_punctuation(string)?;
        let string = string.trim().to_lowercase().replace(' ', "=");
        if string.is_empty() {
            return Ok(None);
        }
        Ok(Some(Self(string)))
    }

    pub(crate) fn find_in_string(string: &str) -> Result<Option<Self>> {
        let capture = Regex::new(SIGNATURE_REGEXP)?.captures(string);
        let Some(capture) = capture else {
            return Ok(None);
        };
        let title = Self(capture[1].to_owned());
        Ok(Some(title))
    }

    pub(crate) fn into_string(&self) -> String {
        format!("=={}", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Regex(#[from] regex::Error),
}

type Result<T> = std::result::Result<T, Error>;

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fmt::{self, Display},
    path::Path,
};

use regex::Regex;

use crate::utils;

const TITLE_REGEXP: &str = r"--([\p{Alphabetic}\pN-]*)";

pub(crate) struct Title(String);

impl Title {
    pub(crate) fn parse(string: &str) -> Result<Option<Self>> {
        let string = utils::remove_punctuation(string)?;
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

    pub(crate) fn from_file_name(path: &Path) -> Result<Option<Self>> {
        let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) else {
            return Ok(None);
        };
        let title = Title::find_in_string(file_stem)?;
        if title.is_some() {
            return Ok(title);
        }
        Title::parse(&file_stem)
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Regex")]
    Regex(#[from] regex::Error),
}

type Result<T> = std::result::Result<T, Error>;

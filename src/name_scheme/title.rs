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
    pub(crate) fn parse(string: &str) -> Option<Self> {
        utils::format(string, "-").map(Self)
    }

    pub(crate) fn find_in_string(string: &str) -> Option<Self> {
        let regex = Regex::new(TITLE_REGEXP).unwrap();
        let capture = regex.captures(string)?;
        Some(Self(capture[1].to_owned()))
    }

    pub(crate) fn from_file_name(path: &Path) -> Option<Self> {
        let file_stem = path.file_stem().and_then(|s| s.to_str())?;
        if let Some(title) = Title::find_in_string(file_stem) {
            return Some(title);
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

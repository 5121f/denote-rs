/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};

use regex::Regex;

use crate::utils;

const SIGNATURE_REGEXP: &str = r"==([\p{Alphabetic}\pN=]*)";

pub(crate) struct Signature(String);

impl Signature {
    pub(crate) fn parse(string: &str) -> Option<Self> {
        utils::format(string, "=").map(Self)
    }

    pub(crate) fn find_in_string(string: &str) -> Option<Self> {
        let regex = Regex::new(SIGNATURE_REGEXP).unwrap();
        let capture = regex.captures(string)?;
        let title = Self(capture[1].to_owned());
        Some(title)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=={}", self.0)
    }
}

#[test]
fn signature_regexp() {
    Regex::new(SIGNATURE_REGEXP).unwrap();
}

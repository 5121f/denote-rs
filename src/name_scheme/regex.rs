/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use regex::Regex;

const ACCEPTABLE_CHAR: &str = r"[\d\p{Alphabetic}]";
const IDENTIFIER: &str = r"(?<id>\d{8}T\d{8})";

pub(crate) fn signature() -> String {
    format!(r"(?<signature>{ACCEPTABLE_CHAR}[{ACCEPTABLE_CHAR}=]*)")
}

pub(crate) fn title() -> String {
    format!("(?<title>{ACCEPTABLE_CHAR}[{ACCEPTABLE_CHAR}-]*)")
}

pub(crate) fn keywords() -> String {
    format!("(?<keywords>{ACCEPTABLE_CHAR}[{ACCEPTABLE_CHAR}_]*)")
}

pub(crate) fn extention() -> String {
    format!("(?<ext>{ACCEPTABLE_CHAR}+)")
}

fn _name_scheme() -> String {
    format!(
        "^{id}(=={signature})?(--{title})?(__{keywords})?(.{ext})?$",
        id = IDENTIFIER,
        signature = signature(),
        title = title(),
        keywords = keywords(),
        ext = extention()
    )
}

pub(crate) fn name_scheme() -> Regex {
    Regex::new(&_name_scheme()).unwrap()
}

#[test]
fn regexp() {
    Regex::new(&_name_scheme()).unwrap();
}

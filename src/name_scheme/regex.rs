/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use regex::Regex;

const ACCEPTABLE_CHARS: &str = r"[\d\p{Alphabetic}]";
const IDENTIFIER: &str = r"(?<id>\d{8}T\d{8})";

pub(crate) fn identifier() -> Regex {
    Regex::new(IDENTIFIER).unwrap()
}

pub(crate) fn signature() -> String {
    format!(r"(?<signature>{ACCEPTABLE_CHARS}[{ACCEPTABLE_CHARS}=]*)")
}

pub(crate) fn title() -> String {
    format!("(?<title>{ACCEPTABLE_CHARS}[{ACCEPTABLE_CHARS}-]*)")
}

pub(crate) fn keywords() -> String {
    format!("(?<keywords>{ACCEPTABLE_CHARS}[{ACCEPTABLE_CHARS}_]*)")
}

pub(crate) fn extention() -> String {
    format!("(?<ext>{ACCEPTABLE_CHARS}+)")
}

pub(crate) fn name_scheme() -> Regex {
    let regex = format!(
        "^{id}(=={signature})?(--{title})?(__{keywords})?(.{ext})?$",
        id = IDENTIFIER,
        signature = signature(),
        title = title(),
        keywords = keywords(),
        ext = extention()
    );
    Regex::new(&regex).unwrap()
}

#[test]
fn regexp() {
    name_scheme();
}

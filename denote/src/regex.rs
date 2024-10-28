/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use once_cell::sync::Lazy;
use regex::Regex;

const ACCEPTABLE_CHARS: &str = r"[\d\p{Alphabetic}]";
const _IDENTIFIER: &str = r"(?<id>\d{8}T\d{8})";

pub static IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(_IDENTIFIER).unwrap());

pub fn signature() -> String {
    format!(r"(?<signature>{ACCEPTABLE_CHARS}[{ACCEPTABLE_CHARS}=]*)")
}

pub fn title() -> String {
    format!("(?<title>{ACCEPTABLE_CHARS}[{ACCEPTABLE_CHARS}-]*)")
}

pub fn keywords() -> String {
    format!("(?<keywords>{ACCEPTABLE_CHARS}[{ACCEPTABLE_CHARS}_]*)")
}

pub fn extension() -> String {
    format!("(?<ext>{ACCEPTABLE_CHARS}+)")
}

pub static NAME_SCHEME: Lazy<Regex> = Lazy::new(|| {
    let regex = format!(
        "^{id}(=={signature})?(--{title})?(__{keywords})?(.{ext})?$",
        id = _IDENTIFIER,
        signature = signature(),
        title = title(),
        keywords = keywords(),
        ext = extension()
    );
    Regex::new(&regex).unwrap()
});

#[test]
fn regexp() {
    NAME_SCHEME.captures("test");
}

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use once_cell::sync::Lazy;
use regex::Regex;

const ACCEPTABLE_CHARS: &str = r"[\d\p{Alphabetic}]";
const _IDENTIFIER: &str = r"(?<id>\d{8}T\d{8})";

pub(crate) const IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(_IDENTIFIER).unwrap());

pub(crate) fn signature() -> String {
    format!(r"(?<signature>{ACCEPTABLE_CHARS}[{ACCEPTABLE_CHARS}=]*)")
}

pub(crate) fn title() -> String {
    format!("(?<title>{ACCEPTABLE_CHARS}[{ACCEPTABLE_CHARS}-]*)")
}

pub(crate) fn keywords() -> String {
    format!("(?<keywords>{ACCEPTABLE_CHARS}[{ACCEPTABLE_CHARS}_]*)")
}

pub(crate) fn extension() -> String {
    format!("(?<ext>{ACCEPTABLE_CHARS}+)")
}

pub(crate) const NAME_SCHEME: Lazy<Regex> = Lazy::new(|| {
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
    static REGEX: Lazy<Regex> = NAME_SCHEME;
    REGEX.captures("test");
}

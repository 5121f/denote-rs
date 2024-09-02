/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use regex::Regex;

const PUNCTUATION: &str = r"\p{P}";
const SPACE_CHARACTERS: &str = r"\s+";

pub(crate) fn remove_punctuation(string: &str) -> String {
    let punctuation = Regex::new(PUNCTUATION).unwrap();
    punctuation.replace_all(string, "").to_string()
}

pub(crate) fn first_letter_uppercase(string: &str) -> String {
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub(crate) fn replace_spaces(string: &str, rep: &str) -> String {
    let regex = Regex::new(SPACE_CHARACTERS).unwrap();
    regex.replace_all(string, rep).to_string()
}

pub(crate) fn format(string: &str, separator: &str) -> Option<String> {
    let string = remove_punctuation(string);
    let string = string.trim().to_lowercase();
    (!string.is_empty()).then(|| replace_spaces(&string, separator))
}

#[test]
fn regexp() {
    Regex::new(PUNCTUATION).unwrap();
    Regex::new(SPACE_CHARACTERS).unwrap();
}

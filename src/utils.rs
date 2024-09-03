/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use regex::Regex;

const PUNCTUATION: &str = r"\p{P}";

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

pub(crate) fn format(string: &str, separator: &str) -> String {
    let string = only_one_letter(string, separator);
    let string = remove_punctuation(&string);
    let string = string.trim().to_lowercase();
    let string = only_one_letter(&string, " ");
    string.replace(" ", separator)
}

fn only_one_letter(string: &str, letter: &str) -> String {
    string.chars().fold(String::new(), |acc, x| {
        let x = x.to_string();
        if acc.ends_with(letter) && letter == x {
            acc
        } else {
            acc + &x
        }
    })
}

#[test]
fn regexp() {
    Regex::new(PUNCTUATION).unwrap();
}

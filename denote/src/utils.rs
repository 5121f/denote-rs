/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

const PUNCTUATION: &str = r"-=_.,;*()";

/// Remove symbols from string which contains in `PUNCTUATION` except separator
pub(crate) fn remove_punctuation(string: &str, separator: &str) -> String {
    string.chars().fold(String::new(), |acc, x| {
        let x = x.to_string();
        if x != separator && PUNCTUATION.contains(&x) {
            return acc;
        }
        acc + &x
    })
}

/// Makes first letter in string uppercase
pub(crate) fn first_letter_uppercase(string: &str) -> String {
    let mut chars = string.chars();
    let first_letter = chars.next();
    match first_letter {
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

pub(crate) fn format(string: &str, separator: &str) -> String {
    let string = leave_only_one_letter(string, separator);
    let string = remove_punctuation(&string, separator);
    let string = string.trim().to_lowercase();
    let string = leave_only_one_letter(&string, " ");
    string.replace(" ", separator)
}

/// Leave only one `letter` per in `string` in places where it is repeated
fn leave_only_one_letter(string: &str, letter: &str) -> String {
    string.chars().fold(String::new(), |acc, x| {
        let x = x.to_string();
        if acc.ends_with(letter) && letter == x {
            return acc;
        }
        acc + &x
    })
}

#[cfg(test)]
mod tests {
    use super::leave_only_one_letter;

    #[test]
    fn only_one_letter_test() {
        assert_eq!(leave_only_one_letter("some---title", "-"), "some-title");
    }
}

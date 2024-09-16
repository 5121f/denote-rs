/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use slug::slugify;

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
    let string = slugify(string);
    if separator == "-" {
        string
    } else {
        string.replace("-", separator)
    }
}

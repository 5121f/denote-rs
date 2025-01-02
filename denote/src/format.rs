/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Makes first letter in string uppercase
pub fn first_letter_uppercase(string: &str) -> String {
    let mut chars = string.chars();
    let first_letter = chars.next();
    first_letter.map_or_else(String::new, |c| {
        format!(
            "{first_letter}{other_letters}",
            first_letter = c.to_uppercase(),
            other_letters = chars.as_str()
        )
    })
}

pub fn format(string: &str, separator: &str) -> String {
    let string = slugify(string);
    if separator == "-" {
        string
    } else {
        string.replace("-", separator)
    }
}

pub fn slugify<S: AsRef<str>>(s: S) -> String {
    _slugify(s.as_ref())
}

fn _slugify(s: &str) -> String {
    let mut slug = String::with_capacity(s.len());
    // Starts with true to avoid leading -
    let mut prev_is_dash = true;

    for x in s.chars() {
        if !x.is_alphabetic() {
            if !prev_is_dash {
                slug.push('-');
                prev_is_dash = true;
            }
            continue;
        }
        slug = format!("{slug}{}", x.to_lowercase());
        prev_is_dash = false;
    }

    if slug.ends_with('-') {
        slug.pop();
        slug.shrink_to_fit();
    }

    slug
}

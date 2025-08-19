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

pub fn slugify<S: AsRef<str>>(s: S, separator: Option<char>) -> String {
    fn inner(s: &str, separator: Option<char>) -> String {
        let mut slug = String::with_capacity(s.len());
        let separator = separator.map(|c| c.to_string()).unwrap_or_default();
        // Starts with true to avoid leading separator
        let mut prev_is_dash = true;

        for x in s.chars() {
            if x.is_alphabetic() || x.is_numeric() {
                slug.push_str(&x.to_lowercase().to_string());
                prev_is_dash = false;
                continue;
            }
            if !prev_is_dash {
                slug.push_str(&separator);
                prev_is_dash = true;
            }
        }

        if !separator.is_empty() && slug.ends_with(&separator) {
            slug.pop();
            slug.shrink_to_fit();
        }

        slug
    }

    inner(s.as_ref(), separator)
}

#[cfg(test)]
mod test {
    use super::*;
    // use Separator::*;

    #[test]
    fn test() {
        assert_eq!(slugify("Some title ", Some('-')), "some-title");
        assert_eq!(
            slugify("Some,keywords asd ", Some(',')),
            "some,keywords,asd"
        );
        assert_eq!(slugify("empTy  separator", None), "emptyseparator");
        assert_eq!(slugify("ddDDDD  ,  lll", Some('=')), "dddddd=lll");
    }
}

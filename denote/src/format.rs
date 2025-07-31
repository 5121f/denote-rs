/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt;

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

pub fn slugify<S: AsRef<str>>(s: S, separator: &Separator) -> String {
    fn inner(s: &str, separator: &Separator) -> String {
        let mut slug = String::with_capacity(s.len());
        let sep_str = separator.to_string();
        // Starts with true to avoid leading separator
        let mut prev_is_dash = true;

        for x in s.chars() {
            if x.is_alphabetic() || x.is_numeric() {
                slug.push_str(&x.to_lowercase().to_string());
                prev_is_dash = false;
                continue;
            }
            if !prev_is_dash {
                slug.push_str(&sep_str);
                prev_is_dash = true;
            }
        }

        if !separator.is_none() && slug.ends_with(&sep_str) {
            slug.pop();
        }

        slug.shrink_to_fit();

        slug
    }

    inner(s.as_ref(), separator)
}

pub enum Separator {
    Char(char),
    None,
}

impl Separator {
    const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl fmt::Display for Separator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Char(ch) => ch.fmt(f),
            Self::None => "".fmt(f),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Separator::*;

    #[test]
    fn test() {
        assert_eq!(slugify("Some title ", &Char('-')), "some-title");
        assert_eq!(
            slugify("Some,keywords asd ", &Char(',')),
            "some,keywords,asd"
        );
        assert_eq!(slugify("empTy  separator", &None), "emptyseparator");
        assert_eq!(slugify("ddDDDD  ,  lll", &Char('=')), "dddddd=lll");
    }
}

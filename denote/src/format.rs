// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use std::borrow::Cow;

/// Makes first letter in string uppercase
pub fn first_letter_uppercase<'a, S>(string: S) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>,
{
    fn inner(string: Cow<'_, str>) -> Cow<'_, str> {
        let mut chars = string.chars();
        let first_letter = chars.next();
        first_letter
            .map(|c| {
                format!(
                    "{first_letter}{other_letters}",
                    first_letter = c.to_uppercase(),
                    other_letters = chars.as_str()
                )
                .into()
            })
            .unwrap_or(string)
    }

    inner(string.into())
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

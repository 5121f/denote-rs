/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};
use std::fs;
use std::path::Path;
use std::time::SystemTime;

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};

use super::regex;

/// Identifier is a date and time formatted as "20240912T13015412"
/// and represent unic identifier for file
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl Identifier {
    /// Use current system time for create Identifier
    pub fn now() -> Self {
        Local::now().naive_local().into()
    }

    /// Try parse identifier from given string.
    pub fn parse<S: AsRef<str>>(string: S) -> Option<Self> {
        fn inner(string: &str) -> Option<Identifier> {
            if string == "now" {
                return Some(Identifier::now());
            }

            Identifier::find_in_string(string).or_else(|| Identifier::from_string(string))
        }

        inner(string.as_ref())
    }

    /// Just call a `from_string_date` and `parse_from_xml` functions.
    pub fn from_string<S: AsRef<str>>(string: S) -> Option<Self> {
        fn inner(string: &str) -> Option<Identifier> {
            let current_time = Local::now().naive_local().time();

            Identifier::from_string_date(string, current_time)
                .or_else(|| Identifier::parse_from_xml_date(string, current_time))
        }

        inner(string.as_ref())
    }

    /// Parse date from xml date format. Takes time from given `time`.
    pub fn parse_from_xml_date<S: AsRef<str>>(string: S, time: NaiveTime) -> Option<Self> {
        fn inner(string: &str, time: NaiveTime) -> Option<Identifier> {
            NaiveDate::parse_from_str(string, "%Y-%m-%d")
                .ok()
                .map(|d| d.and_time(time))
                .map(Into::into)
        }

        inner(string.as_ref(), time)
    }

    /// Parse string for date and time formatted  as follows: `%Y-%m-%d %H:%M`.
    /// Takes milliseconds from given `time`.
    pub fn from_string_date<S: AsRef<str>>(string: S, time: NaiveTime) -> Option<Self> {
        fn inner(string: &str, time: NaiveTime) -> Option<Identifier> {
            NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M")
                .ok()
                .and_then(|d| {
                    d.checked_add_signed(Duration::milliseconds(
                        time.format("%S%3f").to_string().parse().ok()?,
                    ))
                })
                .map(Into::into)
        }

        inner(string.as_ref(), time)
    }

    /// Take date of file creation from file metadata and format it in denote identifier format
    ///
    /// # Errors
    ///
    /// Returns Err if metadata request fails.
    pub fn from_file_metadata<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let metadata = fs::metadata(path)?;
        let created = metadata.created()?;
        Ok(created.into())
    }

    /// Find identifier in string
    ///
    /// # Examples
    ///
    /// ```
    /// use zeroten_denote::Identifier;
    ///
    /// let string = "some random data 20240912T13015412 asdfsas";
    /// let id = Identifier::find_in_string(string).unwrap();
    /// assert_eq!(id.to_string(), "20240912T13015412");
    /// ```
    pub fn find_in_string<S: AsRef<str>>(string: S) -> Option<Self> {
        fn inner(string: &str) -> Option<Identifier> {
            let captures = regex::IDENTIFIER.captures(string)?;

            // We have test in `regex` module to ensure regex is contains `id` group
            let id = captures
                .name("id")
                .expect("Regex: \"id\" name group didn't found");

            Some(Identifier(id.as_str().to_string()))
        }

        inner(string.as_ref())
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::now()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<NaiveDateTime> for Identifier {
    fn from(value: NaiveDateTime) -> Self {
        let date = value.date().format("%Y%m%d");
        let time = value.time();
        let milliseconds = &time.format("%3f").to_string()[..2];
        let time = time.format("%H%M%S");
        Self(format!("{date}T{time}{milliseconds}"))
    }
}

impl From<DateTime<Local>> for Identifier {
    fn from(value: DateTime<Local>) -> Self {
        value.naive_local().into()
    }
}

impl From<SystemTime> for Identifier {
    fn from(value: SystemTime) -> Self {
        let date_time: DateTime<Local> = value.into();
        date_time.into()
    }
}

#[cfg(test)]
mod test {
    use super::Identifier;

    #[test]
    fn find() {
        let text = "some random text 20240908T19063022 asd";
        let identifier = Identifier::parse(text).unwrap();
        assert_eq!(identifier.to_string(), "20240908T19063022");
    }
}

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};
use std::fs;
use std::path::Path;

use chrono::{DateTime, Duration, Local, NaiveDateTime, NaiveTime};

use super::regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl Identifier {
    fn from_date_time(date_time: NaiveDateTime) -> Self {
        let date = date_time.date().format("%Y%m%d").to_string();
        let time = date_time.time();
        let milliseconds = time.format("%3f").to_string()[..2].to_owned();
        let time = time.format("%H%M%S").to_string();
        Self(format!("{date}T{time}{milliseconds}"))
    }

    /// Use current system time for create Identifier
    pub fn now() -> Self {
        let now = chrono::offset::Local::now().naive_local();
        Self::from_date_time(now)
    }

    /// Try parse identifier from given string.
    pub fn parse(string: &str) -> Result<Self> {
        if string == "now" {
            return Ok(Self::now());
        }

        if let Some(id) = Self::find_in_string(string) {
            return Ok(id);
        }

        Self::from_string(string)
    }

    /// Just call a `from_string_date` and `parse_from_xml` functions.
    pub fn from_string(string: &str) -> Result<Self> {
        let current_time = chrono::offset::Local::now().naive_local().time();

        Self::from_string_date(string, current_time)
            .or_else(|_| Self::parse_from_xml_date(string, current_time))
    }

    /// Parse date from xml date format. Takes time from given `time`.
    pub fn parse_from_xml_date(string: &str, time: NaiveTime) -> Result<Self> {
        let date_time = chrono::NaiveDate::parse_from_str(string, "%Y-%m-%d")
            .ok()
            .map(|d| d.and_time(time))
            .ok_or(Error::ParseDate)?;

        Ok(Self::from_date_time(date_time))
    }

    /// Parse string for date and time formatted  as follows: `%Y-%m-%d %H:%M`.
    /// Takes milliseconds from given `time`.
    pub fn from_string_date(string: &str, time: NaiveTime) -> Result<Self> {
        let date_time = chrono::NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M")
            .ok()
            .and_then(|d| {
                d.checked_add_signed(Duration::milliseconds(
                    time.format("%S%3f").to_string().parse().ok()?,
                ))
            })
            .ok_or(Error::ParseDate)?;

        Ok(Self::from_date_time(date_time))
    }

    pub fn from_file_metadata(path: impl AsRef<Path>) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let created: DateTime<Local> = metadata.created()?.into();
        Ok(Self::from_date_time(created.naive_local()))
    }

    pub fn find_in_string(string: &str) -> Option<Self> {
        let id = regex::IDENTIFIER
            .captures(string)?
            .name("id")
            .expect("Regex: \"id\" name group didn't found");
        Some(Self(id.as_str().to_string()))
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::now()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to papse date")]
    ParseDate,
    #[error("Failed to take file metadata: {}", 0)]
    IO(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

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

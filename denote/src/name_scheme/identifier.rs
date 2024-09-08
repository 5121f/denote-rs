/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{self, Display};
use std::fs;
use std::path::Path;

use chrono::{DateTime, Duration, Local, NaiveDateTime};

use super::regex;

#[derive(Clone)]
pub struct Identifier(String);

impl Identifier {
    fn from_date_time(date_time: NaiveDateTime) -> Self {
        let date = date_time.date().format("%Y%m%d").to_string();
        let time = date_time.time();
        let milliseconds = time.format("%3f").to_string()[..2].to_owned();
        let time = time.format("%H%M%S").to_string();
        Self(format!("{date}T{time}{milliseconds}"))
    }

    pub fn now() -> Self {
        let now = chrono::offset::Local::now().naive_local();
        Self::from_date_time(now)
    }

    pub fn parse(string: &str) -> Result<Self> {
        if string == "now" {
            return Ok(Self::now());
        }
        if let Some(id) = Self::find_in_string(string) {
            return Ok(id);
        }
        let current_time = chrono::offset::Local::now().naive_local().time();
        let date_time = chrono::NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M")
            .ok()
            .and_then(|d| {
                d.checked_add_signed(Duration::milliseconds(
                    current_time.format("%S%3f").to_string().parse().ok()?,
                ))
            })
            .or_else(|| {
                chrono::NaiveDate::parse_from_str(string, "%Y-%m-%d")
                    .ok()
                    .map(|d| d.and_time(current_time))
            })
            .ok_or(Error::ConvertDate)?;
        Ok(Self::from_date_time(date_time))
    }

    pub fn from_file_metadata(path: impl AsRef<Path>) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let created: DateTime<Local> = metadata.created()?.into();
        Ok(Self::from_date_time(created.naive_local()))
    }

    fn find_in_string(string: &str) -> Option<Self> {
        let id = regex::IDENTIFIER.captures(string)?.name("id")?;
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
    #[error("Failed to convert date")]
    ConvertDate,
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

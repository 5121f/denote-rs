/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{fs, path::Path};

use chrono::{DateTime, Duration, Local, NaiveDateTime};
use regex::Regex;

const ID_REGEXP: &str = r"\d{8}T\d{8}";

pub(crate) struct Identifier(String);

impl Identifier {
    fn from_date_time(date_time: NaiveDateTime) -> Self {
        let date = date_time.date().format("%Y%m%d").to_string();
        let time = date_time.time();
        let milliseconds = time.format("%3f").to_string()[..2].to_owned();
        let time = time.format("%H%M%S").to_string();
        Self(format!("{date}T{time}{milliseconds}"))
    }

    pub(crate) fn now() -> Self {
        let now = chrono::offset::Local::now().naive_local();
        Self::from_date_time(now)
    }

    pub(crate) fn from_string(string: &str) -> Result<Self> {
        if string == "now" {
            return Ok(Self::now());
        }
        let currnet_time = chrono::offset::Local::now().naive_local().time();
        let first_try = chrono::NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M")
            .map(|d| {
                d.checked_add_signed(Duration::milliseconds(
                    currnet_time.format("%S%3f").to_string().parse().ok()?,
                ))
            })
            .ok()
            .flatten();
        let date_time = match first_try {
            Some(date) => date,
            None => chrono::NaiveDate::parse_from_str(string, "%Y-%m-%d")
                .map(|d| d.and_time(currnet_time))?,
        };
        Ok(Self::from_date_time(date_time))
    }

    pub(crate) fn extract_from_string(string: &str) -> Result<Self> {
        let id = Regex::new(ID_REGEXP)?
            .find(string)
            .ok_or(Error::ExtractIdentifier)?;
        Ok(Self(id.as_str().to_owned()))
    }

    pub(crate) fn from_file_metadata(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let created: DateTime<Local> = metadata.created()?.into();
        Ok(Self::from_date_time(created.naive_local()))
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Regex")]
    Regex(#[from] regex::Error),
    #[error("Failed to extract edentifier")]
    ExtractIdentifier,
    #[error("Failed to convert date")]
    ConvertDate(#[from] chrono::ParseError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

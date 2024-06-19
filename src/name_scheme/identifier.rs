use std::{fs, path::Path};

use anyhow::{Context, Result};
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
                .map(|d| d.and_time(currnet_time))
                .context("Failed to convert date")?,
        };
        Ok(Self::from_date_time(date_time))
    }

    pub(crate) fn extract_from_string(string: &str) -> Result<Self> {
        let id = Regex::new(ID_REGEXP)
            .context("Failed to regex compile")?
            .find(string)
            .context("Failed to extract identifire")?;
        Ok(Self(id.as_str().to_owned()))
    }

    pub(crate) fn from_file_metadata(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let created: DateTime<Local> = metadata.created()?.into();
        Ok(Self::from_date_time(created.naive_local()))
    }

    pub(crate) fn into_string(self) -> String {
        self.0
    }
}

use anyhow::{Context, Result};
use chrono::{Duration, NaiveDateTime};
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

    pub(crate) fn current_time() -> Self {
        Self::from_date_time(chrono::offset::Local::now().naive_local())
    }

    pub(crate) fn from_string(string: &str) -> Result<Self> {
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
                .context("Не удалось конвертировать дату.")?,
        };
        Ok(Self::from_date_time(date_time))
    }

    pub(crate) fn extract_from_string(string: &str) -> Option<Self> {
        Regex::new(ID_REGEXP)
            .ok()?
            .find(string)
            .map(|f| Self(f.as_str().to_owned()))
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

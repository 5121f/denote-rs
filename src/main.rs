mod io;

use crate::io::Io;
use anyhow::{bail, Context, Result};
use chrono::{Duration, NaiveDateTime};
use clap::Parser;
use regex::Regex;
use std::{fs, path::PathBuf};

const ID_REGEXP: &str = r"\d{8}T\d{8}";

struct Identifier(String);

impl Identifier {
    fn from_date_time(date_time: NaiveDateTime) -> Self {
        let date = date_time.date().format("%Y%m%d").to_string();
        let time = date_time.time();
        let milliseconds = time.format("%3f").to_string()[..2].to_owned();
        let time = time.format("%H%M%S").to_string();
        Self(format!("{date}T{time}{milliseconds}"))
    }

    fn current_time() -> Self {
        Self::from_date_time(chrono::offset::Local::now().naive_local())
    }

    fn from_string(string: &str) -> Option<Self> {
        let currnet_time = chrono::offset::Local::now().naive_local().time();
        let date_time = chrono::NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M")
            .map(|d| {
                d.checked_add_signed(Duration::milliseconds(
                    currnet_time.format("%S%3f").to_string().parse().ok()?,
                ))
            })
            .ok()
            .flatten()
            .or_else(|| {
                chrono::NaiveDate::parse_from_str(string, "%Y-%m-%d")
                    .map(|d| d.and_time(currnet_time))
                    .ok()
            })?;
        Some(Self::from_date_time(date_time))
    }

    fn extract_from_string(string: &str) -> Option<Self> {
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

static TITLE_REGEXP: &str = r"--([\p{Alphabetic}\pN-]*)";

#[derive(Default)]
struct Title(String);

impl Title {
    fn from_string(string: &str) -> Self {
        Self(string.trim().to_lowercase().replace(' ', "-"))
    }

    fn extract_from_string(string: &str) -> Option<Self> {
        Regex::new(TITLE_REGEXP)
            .ok()?
            .captures(string)
            .map(|m| Self(m[1].to_owned()))
    }

    fn desluggify(&self) -> String {
        let deslugify = self.0.clone().replace('-', " ");
        let mut chars = deslugify.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

impl ToString for Title {
    fn to_string(&self) -> String {
        format!("--{}", self.0)
    }
}

#[derive(Default)]
struct Keywords(Vec<String>);

impl Keywords {
    fn from_string(string: &str) -> Self {
        let keywords: Vec<_> = string
            .trim()
            .to_lowercase()
            .split(',')
            .map(ToOwned::to_owned)
            .collect();
        let keywords = match keywords.first() {
            Some(first) if first.is_empty() => Vec::new(),
            _ => keywords,
        };
        Self(keywords)
    }
}

impl ToString for Keywords {
    fn to_string(&self) -> String {
        if self.0.is_empty() {
            return String::new();
        }
        format!("__{}", self.0.join("_"))
    }
}

#[derive(Parser)]
struct Cli {
    file_name: String,
    date: Option<String>,
}

struct NameScheme {
    title: Title,
    keywords: Keywords,
    identifier: Identifier,
}

impl ToString for NameScheme {
    fn to_string(&self) -> String {
        [
            self.identifier.to_string(),
            self.title.to_string(),
            self.keywords.to_string(),
        ]
        .concat()
    }
}

#[derive(Default)]
struct NameSchemeBuilder {
    title: Option<Title>,
    keywords: Option<Keywords>,
    identifier: Option<Identifier>,
}

impl NameSchemeBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn take_title_from_user_with_old_title(mut self, io: &mut Io, old_title: &str) -> Result<Self> {
        io.print(&format!("Заголовок [{}]: ", &old_title))?;
        let title = Title::from_string(
            &Some(io.read_line()?)
                .filter(|f| !f.trim().is_empty())
                .unwrap_or(old_title.to_owned()),
        );
        self.title = Some(title);
        Ok(self)
    }

    fn take_title_from_user(mut self, io: &mut Io) -> Result<Self> {
        io.print("Заголовок: ")?;
        let title = Title::from_string(&io.read_line()?);
        self.title = Some(title);
        Ok(self)
    }

    fn take_keywords_from_user(mut self, io: &mut Io) -> Result<Self> {
        io.print("Ключевые слова: ")?;
        let keywords = Keywords::from_string(&io.read_line()?);
        self.keywords = Some(keywords);
        Ok(self)
    }

    fn identifier(mut self, identifier: Identifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    fn build(self) -> NameScheme {
        NameScheme {
            title: self.title.unwrap_or_default(),
            keywords: self.keywords.unwrap_or_default(),
            identifier: self.identifier.unwrap_or_else(Identifier::current_time),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = PathBuf::from(&cli.file_name);

    if !path.exists() {
        bail!("Указаного файла не существует.");
    }
    if !path.is_file() {
        bail!("Указан не файл.");
    }

    let extension = path.extension().and_then(|s| s.to_str()).map(String::from);
    let file_title = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let title = Title::extract_from_string(&file_title)
        .map(|f| f.desluggify())
        .unwrap_or(file_title.to_owned());
    let identifier = if let Some(date) = cli.date {
        Identifier::from_string(&date).context("Не удалось конвертировать дату.")?
    } else {
        Identifier::extract_from_string(&file_title).unwrap_or_else(Identifier::current_time)
    };

    let mut io = Io::new();

    let namescheme_builder = NameSchemeBuilder::new()
        .identifier(identifier)
        .take_title_from_user_with_old_title(&mut io, &title)?
        .take_keywords_from_user(&mut io)?;

    let new_file_title = namescheme_builder.build().to_string();

    let new_file_name = if let Some(extention) = &extension {
        format!("{new_file_title}.{extention}")
    } else {
        new_file_title
    };

    if cli.file_name == new_file_name {
        println!("Действие не требуется.");
    } else {
        println!(
            "Переименовать \"{}\" в \"{}\"",
            &cli.file_name, new_file_name
        );
        if io.question("Подтвердить переименование?", true)? {
            fs::rename(&path, new_file_name)
                .with_context(|| format!("Не удалсоь переименовать файл {path:?}"))?;
        }
    }

    Ok(())
}

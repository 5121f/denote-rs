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
            .flatten();
        let date_time = match date_time {
            Some(date_time) => date_time,
            None => chrono::NaiveDate::parse_from_str(string, "%Y-%m-%d")
                .ok()?
                .and_time(currnet_time),
        };
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

struct Title(String);

impl Title {
    fn from_string(string: &str) -> Self {
        Self(string.trim().to_lowercase().replace(' ', "-"))
    }

    fn extract_from_string(strnig: &str) -> Option<Self> {
        Regex::new(TITLE_REGEXP)
            .ok()?
            .captures(strnig)
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
        (!self.0.is_empty())
            .then_some(format!("__{}", self.0.join("_")))
            .unwrap_or_default()
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Rename file
    Rename {
        file_name: String,
        date: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Rename { file_name, date } => {
            let path = PathBuf::from(&file_name);

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
            let identifier = if let Some(date) = date {
                Identifier::from_string(&date).context("Не удалось конвертировать дату.")?
            } else {
                Identifier::extract_from_string(&file_title)
                    .unwrap_or_else(Identifier::current_time)
            };

            let mut io = Io::new();

            let title = Title::extract_from_string(&file_title)
                .map(|f| f.desluggify())
                .unwrap_or(file_title.clone());
            io.print(&format!("Заголовок [{}]: ", &title))?;
            let new_title = Title::from_string(
                &Some(io.read_line()?)
                    .filter(|f| !f.trim().is_empty())
                    .unwrap_or(title),
            );

            io.print("Ключевые слова: ")?;
            let keywords = Keywords::from_string(&io.read_line()?);

            let new_file_title = [
                identifier.to_string(),
                new_title.to_string(),
                keywords.to_string(),
            ]
            .concat();
            let new_file_name = if let Some(extention) = &extension {
                format!("{new_file_title}.{extention}")
            } else {
                new_file_title
            };

            if file_name == new_file_name {
                println!("Действие не требуется.");
            } else {
                println!("Переименовать \"{}\" в \"{}\"", &file_name, new_file_name);
                if io.question("Подтвердить переименование?", true)? {
                    fs::rename(&path, new_file_name)
                        .with_context(|| format!("Не удалсоь переименовать файл {path:?}"))?;
                }
            }
        }
    }
    Ok(())
}

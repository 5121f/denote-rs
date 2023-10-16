mod io;

use crate::io::{Stdin, Stdout};
use anyhow::{bail, Context, Ok, Result};
use chrono::NaiveDateTime;
use clap::Parser;
use regex::Regex;
use std::{env, fs, path::PathBuf};

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

    fn from_string(string: &str) -> Result<Self> {
        let currnet_time = chrono::offset::Local::now().naive_local().time();
        let seconds = currnet_time.format("%S:%.f").to_string();
        let date_time = chrono::NaiveDateTime::parse_from_str(
            &format!("{string}:{seconds}"),
            "%Y-%m-%d %H:%M:%S:%.f",
        )
        .or_else(|_| {
            let time = currnet_time.format("%H:%M:%S:%.f").to_string();
            chrono::NaiveDateTime::parse_from_str(
                &format!("{string} {time}"),
                "%Y-%m-%d %H:%M:%S:%.f",
            )
        })?;
        Ok(Self::from_date_time(date_time))
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
        if self.0.is_empty() {
            String::new()
        } else {
            let keywords = self.0.join("_");
            format!("__{keywords}")
        }
    }
}

struct NameScheme {
    date: Identifier,
    title: Title,
    keywords: Keywords,
    extention: Option<String>,
}

impl NameScheme {
    fn new(date: Identifier, title: Title, keywords: Keywords, extention: Option<String>) -> Self {
        Self {
            date,
            title,
            keywords,
            extention,
        }
    }
}

impl ToString for NameScheme {
    fn to_string(&self) -> String {
        let title = format!(
            "{}{}{}",
            self.date.to_string(),
            self.title.to_string(),
            self.keywords.to_string()
        );
        if let Some(extention) = &self.extention {
            format!("{title}.{extention}")
        } else {
            title
        }
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
            let current_dir =
                env::current_dir().context("Не удалось получить рабочую директорию")?;
            let path = current_dir.join(&file_name);

            if !path.exists() {
                bail!("Указаного файла не существует.");
            }
            if !path.is_file() {
                bail!("Указан не файл.");
            }

            let extension = path.extension().and_then(|s| s.to_str()).map(String::from);
            let file_title = PathBuf::from(file_name.clone())
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(String::new);

            let mut stdout = Stdout::new();
            let mut stdin = Stdin::new();

            let title = Title::extract_from_string(&file_title)
                .map(|f| f.desluggify())
                .unwrap_or(file_title.clone());
            stdout.print(&format!("Заголовок [{}]: ", &title))?;
            let new_title = Title::from_string(
                &Some(stdin.read_line()?)
                    .filter(|f| !f.trim().is_empty())
                    .unwrap_or(title),
            );

            stdout.print("Ключевые слова: ")?;
            let keywords = Keywords::from_string(&stdin.read_line()?);

            let identifier = if let Some(date) = date {
                Identifier::from_string(&date).context("Не удалось конвертировать дату.")?
            } else {
                Identifier::extract_from_string(&file_title)
                    .unwrap_or_else(Identifier::current_time)
            };

            let name_scheme =
                NameScheme::new(identifier, new_title, keywords, extension).to_string();

            if file_name == name_scheme {
                println!("Действие не требуется.");
            } else {
                println!("Переименовать \"{}\" в \"{}\"", &file_name, name_scheme);
                stdout.print("Подтвердить переименование? [Y/n] ")?;
                if stdin.take_confirmation()? {
                    fs::rename(&path, current_dir.join(name_scheme))
                        .with_context(|| format!("Не удалсоь переименовать файл {path:?}"))?;
                }
            }
        }
    }
    Ok(())
}

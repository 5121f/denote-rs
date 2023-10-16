use anyhow::{bail, Context, Result};
use clap::Parser;
use regex::Regex;
use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

const ID_REGEXP: &str = r"\d{8}T\d{8}";

struct Date(String);

impl Date {
    fn current_time() -> Self {
        let now = chrono::offset::Local::now();
        let date = now.date_naive().format("%Y%m%d").to_string();
        let time = now.time();
        let milliseconds = time.format("%3f").to_string()[..2].to_owned();
        let time = time.format("%H%M%S").to_string();
        Self(format!("{date}T{time}{milliseconds}"))
    }

    fn retrive_from_string(string: &str) -> Option<Self> {
        Regex::new(ID_REGEXP)
            .ok()?
            .find(string)
            .map(|f| Self(f.as_str().to_owned()))
    }
}

impl ToString for Date {
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

    fn retrive_from_string(strnig: &str) -> Option<Self> {
        Regex::new(TITLE_REGEXP)
            .ok()?
            .captures(strnig)
            .map(|m| Self(m[1].to_owned()))
    }

    fn desluggify(&self) -> String {
        self.0
            .clone()
            .replace('-', " ")
            .chars()
            .enumerate()
            .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
            .collect()
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
    date: Date,
    title: Title,
    keywords: Keywords,
    extention: Option<String>,
}

impl NameScheme {
    fn new(date: Date, title: Title, keywords: Keywords, extention: Option<String>) -> Self {
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

struct Stdout(io::Stdout);

impl Stdout {
    fn new() -> Self {
        Self(io::stdout())
    }

    fn print(&mut self, value: &str) -> Result<()> {
        print!("{}", value);
        Ok(self.0.flush()?)
    }
}

struct Stdin {
    stdin: io::Stdin,
    buf: String,
}

impl Stdin {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buf: String::new(),
        }
    }

    fn read_line(&mut self) -> Result<String> {
        self.stdin
            .read_line(&mut self.buf)
            .context("Не удалось прочитать пользовательский ввод")?;
        let res = self.buf.clone();
        self.buf.clear();
        Ok(res)
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
    Rename { file_name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Rename { file_name } => {
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

            let title = Title::retrive_from_string(&file_title)
                .map(|f| f.desluggify())
                .unwrap_or(file_title.clone());
            stdout.print(&format!("Заголовок [{}]: ", &title))?;
            let title = {
                let title = Some(stdin.read_line()?)
                    .filter(|f| !f.trim().is_empty())
                    .unwrap_or(title);
                Title::from_string(&title)
            };

            stdout.print("Ключевые слова: ")?;
            let keywords = {
                let keywords = stdin.read_line()?;
                Keywords::from_string(&keywords)
            };

            let date = Date::retrive_from_string(&file_title).unwrap_or_else(Date::current_time);

            let name_scheme = NameScheme::new(date, title, keywords, extension);
            let name_scheme = name_scheme.to_string();

            if file_title == name_scheme {
                println!("Действие не требуется.");
            } else {
                println!("Переименовать \"{}\" в \"{}\"", &file_name, name_scheme);
                stdout.print("Подтвердить переименование? [Y/n] ")?;
                let response = stdin.read_line()?;
                let response = response.trim().to_lowercase();
                if response == "y" || response.is_empty() {
                    fs::rename(&path, current_dir.join(name_scheme))
                        .with_context(|| format!("Не удалсоь переименовать файл {path:?}"))?;
                }
            }
        }
    }
    Ok(())
}

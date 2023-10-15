use anyhow::{bail, Context, Ok, Result};
use clap::Parser;
use regex::{Match, Regex};
use std::{
    env, fs,
    io::{self, Write},
};

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
}

impl ToString for Date {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

static FILENAME_REGEXP: &str = r"--([\p{Alphabetic}\pN]*)";

struct Filename(String);

impl Filename {
    fn from_string(string: String) -> Self {
        Self(string.trim().to_lowercase().replace(' ', "-"))
    }

    fn retrive_from_string(strnig: &str) -> Option<Self> {
        Regex::new(FILENAME_REGEXP)
            .ok()?
            .captures(strnig)
            .map(|m| Self(m[1].to_owned()))
    }

    fn desluggify(&self) -> Option<String> {
        let mut desluggify = self.0.clone().replace('-', " ");
        let firs_letter = desluggify.chars().next()?.to_uppercase().to_string();
        desluggify.replace_range(0..1, &firs_letter);
        Some(desluggify)
    }
}

impl ToString for Filename {
    fn to_string(&self) -> String {
        format!("--{}", self.0)
    }
}

struct Keywords(Vec<String>);

impl Keywords {
    fn from_string(string: String) -> Self {
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
    name: Filename,
    keywords: Keywords,
}

impl NameScheme {
    fn new(date: Date, name: Filename, keywords: Keywords) -> Self {
        Self {
            date,
            name,
            keywords,
        }
    }
}

impl ToString for NameScheme {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.date.to_string(),
            self.name.to_string(),
            self.keywords.to_string()
        )
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
    #[clap(long)]
    rename: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(file_name) = cli.rename {
        let current_dir = env::current_dir().context("Не удалось получить рабочую директорию")?;
        let path = current_dir.join(&file_name);

        if !path.exists() {
            bail!("Указаного файла не существует.");
        }
        if !path.is_file() {
            bail!("Указан не файл.");
        }

        let mut stdout = Stdout::new();
        let mut stdin = Stdin::new();

        let title = Filename::retrive_from_string(&file_name)
            .and_then(|f| f.desluggify())
            .unwrap_or(file_name.clone());
        stdout.print(&format!("Имя файла [{}]: ", &title))?;
        let new_file_name = {
            let new_file_name = Some(stdin.read_line()?)
                .filter(|f| !f.trim().is_empty())
                .unwrap_or(title);
            Filename::from_string(new_file_name)
        };

        stdout.print("Ключевые слова: ")?;
        let keywords = {
            let keywords = stdin.read_line()?;
            Keywords::from_string(keywords)
        };

        let name_scheme = NameScheme::new(Date::current_time(), new_file_name, keywords);
        let name_scheme = name_scheme.to_string();

        println!("Переименовать \"{}\" в \"{}\"", &file_name, name_scheme);
        stdout.print("Подтвердить переименование? [Y/n] ")?;
        let response = stdin.read_line()?;
        let response = response.trim().to_lowercase();
        if response == "y" || response.is_empty() {
            fs::rename(&path, current_dir.join(name_scheme))
                .with_context(|| format!("Не удалсоь переименовать файл {path:?}"))?;
        }
    }
    Ok(())
}

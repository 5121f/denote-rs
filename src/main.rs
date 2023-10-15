use clap::Parser;
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

struct Filename(String);

impl Filename {
    fn from_string(string: String) -> Self {
        let filename = string.to_lowercase().replace(' ', "-");
        Self(format!("--{filename}"))
    }
}

impl ToString for Filename {
    fn to_string(&self) -> String {
        self.0.clone()
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

    fn print(&mut self, value: &str) {
        print!("{}", value);
        self.0.flush().unwrap();
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

    fn read_line(&mut self) -> String {
        self.stdin.read_line(&mut self.buf).unwrap();
        self.buf.clone()
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(long)]
    rename: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(file_name) = cli.rename {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join(&file_name);

        let mut stdout = Stdout::new();
        let mut stdin = Stdin::new();

        stdout.print(&format!("Имя файла [{}]: ", &file_name));
        let new_file_name = {
            let new_file_name = stdin.read_line();
            let new_file_name = if new_file_name.trim().is_empty() {
                file_name.clone()
            } else {
                new_file_name
            };
            Filename::from_string(new_file_name)
        };

        stdout.print("Ключевые слова: ");
        let keywords = {
            let keywords = stdin.read_line();
            Keywords::from_string(keywords)
        };

        let name_scheme = NameScheme::new(Date::current_time(), new_file_name, keywords);
        let name_scheme = name_scheme.to_string();

        println!("Переименовать \"{}\" в \"{}\"", &file_name, name_scheme);
        stdout.print("Подтвердить переименование? [Y/n] ");
        let response = stdin.read_line();
        let response = response.trim().to_lowercase();
        if response == "y" || response.is_empty() {
            fs::rename(path, current_dir.join(name_scheme)).unwrap();
        }
    }
}

use clap::Parser;
use std::{
    env, fs,
    io::{self, Stdout, Write},
    path::PathBuf,
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
        Self(
            string
                .to_lowercase()
                .split(',')
                .map(ToOwned::to_owned)
                .collect(),
        )
    }
}

impl ToString for Keywords {
    fn to_string(&self) -> String {
        let keywords = self.0.join("_");
        format!("__{keywords}")
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

#[derive(Parser)]
struct Cli {
    #[clap(long)]
    rename: Option<PathBuf>,
}

fn print(value: &str, stdout: &mut Stdout) {
    print!("{}", value);
    stdout.flush().unwrap();
}

fn main() {
    let cli = Cli::parse();
    if let Some(path) = cli.rename {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join(path);

        let mut stdout = io::stdout();
        let stdin = io::stdin();

        print("Имя файла: ", &mut stdout);
        let file_name = {
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            Filename::from_string(buf)
        };

        print("Ключевые слова: ", &mut stdout);
        let keywords = {
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            Keywords::from_string(buf)
        };

        let name_scheme = NameScheme::new(Date::current_time(), file_name, keywords);
        fs::rename(path, current_dir.join(name_scheme.to_string())).unwrap();
    }
}

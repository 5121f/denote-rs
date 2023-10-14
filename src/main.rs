use clap::Parser;
use std::{env, fs, io, path::PathBuf};

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

struct NameScheme {
    date: Date,
    name: Filename,
}

impl NameScheme {
    fn new(date: Date, name: Filename) -> Self {
        Self { date, name }
    }
}

impl ToString for NameScheme {
    fn to_string(&self) -> String {
        format!("{}{}", self.date.to_string(), self.name.to_string())
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(long)]
    rename: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(path) = cli.rename {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join(path);
        println!("Имя файла: ");
        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name).unwrap();
        let file_name = Filename::from_string(file_name);
        let name_scheme = NameScheme::new(Date::current_time(), file_name);
        fs::rename(path, current_dir.join(name_scheme.to_string())).unwrap();
    }
}

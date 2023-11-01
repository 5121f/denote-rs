use anyhow::{Context, Result};
use std::io::Write;

pub struct Io {
    stdout: std::io::Stdout,
    stdin: std::io::Stdin,
}

impl Io {
    pub fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut buf = String::new();
        self.stdin
            .read_line(&mut buf)
            .context("Не удалось прочитать пользовательский ввод")?;
        Ok(buf)
    }

    pub fn print(&mut self, value: &str) -> Result<()> {
        print!("{}", value);
        self.stdout.flush()?;
        Ok(())
    }

    pub fn question(&mut self, text: &str, default_ansfer: bool) -> Result<bool> {
        print!("{}", text);
        let prompt = if default_ansfer { " [Y/n] " } else { " [y/N] " };
        print!("{}", prompt);
        self.stdout.flush()?;
        let response = self.read_line()?;
        let response = if response == "\n" {
            default_ansfer
        } else {
            response.to_lowercase() == "y\n"
        };
        Ok(response)
    }
}

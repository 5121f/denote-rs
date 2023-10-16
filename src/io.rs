use anyhow::{Context, Result};
use std::io::Write;

pub(crate) struct Stdout(std::io::Stdout);

impl Stdout {
    pub(crate) fn new() -> Self {
        Self(std::io::stdout())
    }

    pub(crate) fn print(&mut self, value: &str) -> Result<()> {
        print!("{}", value);
        self.0.flush()?;
        Ok(())
    }
}

pub(crate) struct Stdin {
    stdin: std::io::Stdin,
    buf: String,
}

impl Stdin {
    pub(crate) fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
            buf: String::new(),
        }
    }

    pub(crate) fn read_line(&mut self) -> Result<String> {
        self.stdin
            .read_line(&mut self.buf)
            .context("Не удалось прочитать пользовательский ввод")?;
        let res = self.buf.clone();
        self.buf.clear();
        Ok(res)
    }

    pub(crate) fn take_confirmation(&mut self) -> Result<bool> {
        let response = self.read_line()?;
        Ok(response == "\n" || response.to_lowercase() == "y\n")
    }
}

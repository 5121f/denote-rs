/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

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
            .context("Failed to read user input")?;
        Ok(buf.trim().to_owned())
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
        let response = if response.is_empty() {
            default_ansfer
        } else {
            response.to_lowercase() == "y"
        };
        Ok(response)
    }
}

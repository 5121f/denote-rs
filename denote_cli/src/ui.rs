/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt;
use std::io::{self, Write};

use Answer::{No, Yes};
use anyhow::{Context, Result};
use denote::{Extension, Keywords, Title};

/// User Interface
pub struct UI {
    stdout: io::Stdout,
    stdin: io::Stdin,
}

impl UI {
    pub(crate) fn new() -> Self {
        Self {
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }

    pub(crate) fn confirm(
        &mut self,
        question: impl fmt::Display,
        default_ansfer: Answer,
    ) -> Result<Answer> {
        let prompt = match default_ansfer {
            Yes => "[Y/n]",
            No => "[y/N]",
        };
        self.print(format!("{question} {prompt} "))?;
        let mut response = self.read_line()?;
        response.make_ascii_lowercase();
        Ok(match response.trim() {
            "y" | "yes" => Yes,
            "" => default_ansfer,
            _ => No,
        })
    }

    pub fn rename_confirm(
        &mut self,
        old_file_name: impl fmt::Display,
        new_file_name: impl fmt::Display,
    ) -> Result<Answer> {
        println!(
            "Old name \"{old_file_name}\"\n\
            New name \"{new_file_name}\""
        );
        self.confirm("Accept?", Yes)
    }

    pub fn create_file_confirm(&mut self, file_name: impl fmt::Display) -> Result<Answer> {
        self.confirm(format!("Create file \"{file_name}\"?"), Yes)
    }

    pub(crate) fn title_with_old_title<S>(&mut self, old_title: S) -> Result<Option<Title>>
    where
        S: AsRef<str>,
    {
        let old_title = old_title.as_ref();
        self.print(format!("Title [{old_title}]: "))?;
        let input = self.read_line()?;
        let title = if input.trim().is_empty() {
            old_title
        } else {
            &input
        };
        Ok(Title::parse(title))
    }

    pub(crate) fn no_action_needed() {
        println!("No action needed");
    }

    pub(crate) fn take_title(&mut self) -> Result<Option<Title>> {
        self.print("Title: ")?;
        let input = self.read_line()?;
        Ok(Title::parse(&input))
    }

    pub(crate) fn take_keywords(&mut self) -> Result<Option<Keywords>> {
        self.print("Keywords: ")?;
        let input = self.read_line()?;
        Ok(Keywords::parse_user_input(&input))
    }

    pub(crate) fn take_extension(&mut self) -> Result<Option<Extension>> {
        self.print("Extension: ")?;
        let input = self.read_line()?;
        Ok(Extension::new(&input))
    }

    fn read_line(&self) -> Result<String> {
        let mut buf = String::new();
        self.stdin
            .read_line(&mut buf)
            .context("failed to read user input")?;
        Ok(buf)
    }

    fn print(&mut self, value: impl fmt::Display) -> Result<()> {
        print!("{value}");
        self.stdout.flush().context("failed to flush stdout")?;
        Ok(())
    }
}

pub enum Answer {
    Yes,
    No,
}

impl Answer {
    pub const fn as_bool(&self) -> bool {
        match self {
            Yes => true,
            No => false,
        }
    }
}

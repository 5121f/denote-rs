/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fmt,
    io::{self, Write},
    ops::Deref,
};

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
        let response = self.read_line()?;
        let response = if response.is_empty() {
            default_ansfer
        } else {
            match response.to_lowercase().trim() {
                "y" | "yes" => Yes,
                _ => No,
            }
        };
        Ok(response)
    }

    pub fn rename(
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

    pub fn create_file_p(&mut self, file_name: impl fmt::Display) -> Result<Answer> {
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

impl Deref for Answer {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        match self {
            Yes => &true,
            No => &false,
        }
    }
}

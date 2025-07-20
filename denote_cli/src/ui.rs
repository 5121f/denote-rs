/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::io::{self, Write};

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

    pub(crate) fn question(&mut self, question: &str, default_ansfer: bool) -> Result<bool> {
        let prompt = if default_ansfer { "[Y/n]" } else { "[y/N]" };
        self.print(&format!("{question} {prompt} "))?;
        let response = self.read_line()?;
        let response = if response.is_empty() {
            default_ansfer
        } else {
            let response = response.to_lowercase();
            response == "y" || response == "yes"
        };
        Ok(response)
    }

    pub fn rename(&mut self, old_file_name: &str, new_file_name: &str) -> Result<bool> {
        println!(
            "Old name \"{old_file_name}\"\n\
            New name \"{new_file_name}\""
        );
        self.question("Accept?", true)
    }

    pub fn create_file_p(&mut self, file_name: &str) -> Result<bool> {
        self.question(&format!("Create file \"{file_name}\"?"), true)
    }

    pub(crate) fn title_with_old_title(&mut self, old_title: &str) -> Result<Option<Title>> {
        self.print(&format!("Title [{old_title}]: "))?;
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
        self.stdin.read_line(&mut buf)?;
        Ok(buf.trim().to_owned())
    }

    fn print(&mut self, value: &str) -> Result<()> {
        print!("{value}");
        self.stdout.flush()?;
        Ok(())
    }
}

type Result<T> = std::result::Result<T, io::Error>;

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod cli_args;
mod io;
mod name_scheme;

use anyhow::{bail, Context, Result};
use clap::Parser;
use cli_args::Cli;
use io::Io;
use name_scheme::{extention::Extention, identifier::Identifier, name_scheme, title::Title};
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut io = Io::new();

    match cli {
        Cli::Rename {
            file_names,
            date,
            date_from_metadata,
            accept,
            no_keywords,
        } => {
            for file_name in file_names {
                rename_file(
                    file_name,
                    date.as_ref().map(|d| d.as_str()),
                    date_from_metadata,
                    accept,
                    no_keywords,
                    &mut io,
                )?;
            }
        }
        Cli::Touch { date } => touch(date.as_deref(), &mut io)?,
    }

    Ok(())
}

fn touch(date: Option<&str>, io: &mut Io) -> Result<()> {
    let identifier = match date {
        Some(date) => Identifier::from_string(date)?,
        None => Identifier::now(),
    };

    let file_name = name_scheme(identifier, io.title()?, io.keywords()?, io.extention()?);

    let accepted = io.question(&format!("Create file \"{file_name}\"?"), true)?;
    if accepted {
        fs::File::create(file_name).context("Failed to file creation")?;
    }
    Ok(())
}

fn rename_file(
    file_name: String,
    date: Option<&str>,
    date_from_metadata: bool,
    accept: bool,
    no_keywords: bool,
    io: &mut Io,
) -> Result<()> {
    let path = PathBuf::from(&file_name);

    if !path.exists() {
        bail!("File dosen't exists");
    }
    if path.is_dir() {
        bail!("Renaming directories are not supported");
    }

    let extention = path
        .extension()
        .and_then(|s| s.to_str())
        .map(String::from)
        .map(Extention::new)
        .flatten();

    let file_title = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let identifier = if date_from_metadata {
        Identifier::from_file_metadata(&path)?
    } else if let Some(date) = date {
        Identifier::from_string(&date)?
    } else {
        Identifier::extract_from_string(&file_title).unwrap_or_else(|_| Identifier::now())
    };

    let title = if accept {
        Title::find_in_string(&file_title).or_else(|_| Title::parse(&file_title))?
    } else {
        let old_title = Title::find_in_string(&file_title)?
            .map(|title| title.desluggify())
            .unwrap_or(file_title);
        io.title_with_old_title(&old_title)?
    };

    let keywords = if !no_keywords { io.keywords()? } else { None };

    let new_file_name = name_scheme(identifier, title, keywords, extention);

    if file_name == new_file_name {
        println!("No action required");
        return Ok(());
    }

    if !accept {
        println!("Rename \"{}\" в \"{}\"", &file_name, new_file_name);
        let accepted = io.question("Accept?", true)?;
        if !accepted {
            return Ok(());
        }
    }

    fs::rename(&path, new_file_name).with_context(|| format!("Failed to rename file {path:?}"))?;

    Ok(())
}

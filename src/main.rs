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
use name_scheme::{
    extention::Extention, identifier::Identifier, keywords::Keywords, name_scheme, title::Title,
};
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Rename {
            file_names,
            date,
            date_from_metadata,
            title,
            keywords,
            extention,
            default,
            accept,
        } => {
            for file_name in file_names {
                let date = date.as_ref().map(|d| d.as_str());
                let keywords = keywords.as_deref();
                let title = title.as_deref();
                rename_file(
                    file_name,
                    date,
                    date_from_metadata,
                    title,
                    keywords,
                    extention.as_deref(),
                    default,
                    accept,
                )?;
            }
        }
        Cli::Touch {
            title,
            date,
            keywords,
            extention,
            default,
            accept,
        } => touch(
            title.as_deref(),
            date.as_deref(),
            keywords.as_deref(),
            extention.as_deref(),
            default,
            accept,
        )?,
    }

    Ok(())
}

fn touch(
    title: Option<&str>,
    date: Option<&str>,
    keywords: Option<&str>,
    extention: Option<&str>,
    default: bool,
    accept: bool,
) -> Result<()> {
    let mut io = Io::new();

    let identifier = match date {
        Some(date) => Identifier::from_string(date)?,
        None => Identifier::now(),
    };

    let title = if let Some(title) = title {
        Title::parse(title)?
    } else if default {
        None
    } else {
        io.title()?
    };

    let keywords = if let Some(keywords) = keywords {
        Keywords::from_string(keywords)
    } else if default {
        None
    } else {
        io.keywords()?
    };

    let extention = if let Some(extention) = extention {
        Extention::new(extention.to_string())
    } else if default {
        None
    } else {
        io.extention()?
    };

    let file_name = name_scheme(identifier, title, keywords, extention);

    if !accept {
        let accepted = io.question(&format!("Create file \"{file_name}\"?"), true)?;
        if !accepted {
            return Ok(());
        }
    }

    fs::File::create(file_name).context("Failed to file creation")?;

    Ok(())
}

fn rename_file(
    file_name: String,
    date: Option<&str>,
    date_from_metadata: bool,
    title: Option<&str>,
    keywords: Option<&str>,
    extention: Option<&str>,
    default: bool,
    accept: bool,
) -> Result<()> {
    let mut io = Io::new();

    let path = PathBuf::from(&file_name);

    if !path.exists() {
        bail!("File dosen't exists");
    }
    if path.is_dir() {
        bail!("Renaming directories are not supported");
    }

    let extention = if let Some(extention) = extention {
        Extention::new(extention.to_string())
    } else {
        path.extension()
            .and_then(|s| s.to_str())
            .map(String::from)
            .map(Extention::new)
            .flatten()
    };

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

    let title = if let Some(title) = title {
        Title::parse(title)?
    } else if default {
        Title::find_in_string(&file_title).or_else(|_| Title::parse(&file_title))?
    } else {
        let old_title = Title::find_in_string(&file_title)?
            .map(|title| title.desluggify())
            .unwrap_or(file_title);
        io.title_with_old_title(&old_title)?
    };

    let keywords = if let Some(keywords) = keywords {
        Keywords::from_string(&keywords)
    } else if default {
        None
    } else {
        io.keywords()?
    };

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

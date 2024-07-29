/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod cli_args;
mod io;
mod name_scheme;
mod utils;

use anyhow::{bail, Context, Result};
use clap::Parser;
use cli_args::Cli;
use io::Io;
use name_scheme::{
    extention::Extention, identifier::Identifier, keywords::Keywords, signature::Signature,
    title::Title, NameScheme,
};
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Rename {
            file_names,
            date,
            date_from_metadata,
            signature,
            title,
            keywords,
            extention,
            non_interactive,
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
                    signature.as_deref(),
                    title,
                    keywords,
                    extention.as_deref(),
                    non_interactive,
                    accept,
                )?;
            }
        }
        Cli::Touch {
            title,
            date,
            signature,
            keywords,
            extention,
            non_interactive,
            accept,
        } => touch(
            title.as_deref(),
            date.as_str(),
            signature.as_deref(),
            keywords.as_deref(),
            extention.as_deref(),
            non_interactive,
            accept,
        )?,
    }

    Ok(())
}

fn touch(
    title: Option<&str>,
    date: &str,
    signature: Option<&str>,
    keywords: Option<&str>,
    extention: Option<&str>,
    non_interactive: bool,
    accept: bool,
) -> Result<()> {
    let mut io = Io::new();

    let identifier = Identifier::from_string(date)?;

    let mut name_scheme = NameScheme::new(identifier);

    if let Some(signature) = signature {
        let signature = Signature::parse(signature)?;
        name_scheme.signature = signature;
    };

    name_scheme.title = if let Some(title) = title {
        Title::parse(title)?
    } else if non_interactive {
        None
    } else {
        io.title()?
    };

    name_scheme.keywords = if let Some(keywords) = keywords {
        Keywords::from_string(keywords)
    } else if non_interactive {
        None
    } else {
        io.keywords()?
    };

    name_scheme.extention = if let Some(extention) = extention {
        Extention::new(extention.to_string())
    } else if non_interactive {
        None
    } else {
        io.extention()?
    };

    let file_name = name_scheme.to_string();

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
    signature: Option<&str>,
    title: Option<&str>,
    keywords: Option<&str>,
    extention: Option<&str>,
    non_interactive: bool,
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

    let mut name_scheme = NameScheme::new(identifier);

    name_scheme.signature = if let Some(signature) = signature {
        Signature::parse(signature)?
    } else if non_interactive {
        Signature::find_in_string(&file_title)?
    } else {
        None
    };

    name_scheme.title = if let Some(title) = title {
        Title::parse(title)?
    } else if non_interactive {
        Title::find_in_string(&file_title).or_else(|_| Title::parse(&file_title))?
    } else {
        let old_title = Title::find_in_string(&file_title)?
            .map(|title| title.desluggify())
            .unwrap_or(file_title);
        io.title_with_old_title(&old_title)?
    };

    name_scheme.keywords = if let Some(keywords) = keywords {
        Keywords::from_string(&keywords)
    } else if non_interactive {
        None
    } else {
        io.keywords()?
    };

    name_scheme.extention = if let Some(extention) = extention {
        Extention::new(extention.to_string())
    } else {
        path.extension()
            .and_then(|s| s.to_str())
            .map(String::from)
            .map(Extention::new)
            .flatten()
    };

    let new_file_name = name_scheme.to_string();

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

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod cli_args;
mod ui;

use std::{fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;

use cli_args::Cli;
use denote::{Extension, Identifier, Keywords, NameScheme, Signature, Title};
use ui::UI;

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
            extension,
            non_interactive,
            accept,
        } => {
            for file_name in file_names {
                rename_file(
                    file_name,
                    date.as_deref(),
                    date_from_metadata,
                    signature.as_deref(),
                    title.as_deref(),
                    keywords.as_deref(),
                    extension.as_deref(),
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
            extension,
            non_interactive,
            accept,
        } => touch(
            title,
            date,
            signature,
            keywords,
            extension,
            non_interactive,
            accept,
        )?,
    }

    Ok(())
}

fn touch(
    title: Option<String>,
    date: String,
    signature: Option<String>,
    keywords: Option<String>,
    extension: Option<String>,
    non_interactive: bool,
    accept: bool,
) -> Result<()> {
    let mut ui = UI::new();

    let identifier = Identifier::parse(&date)?;

    let interactive = !non_interactive;

    let mut name_scheme = NameScheme::new(identifier);

    if let Some(signature) = signature {
        let signature = Signature::parse(&signature);
        name_scheme.signature(signature);
    }

    if let Some(title) = title {
        let title = Title::parse(&title);
        name_scheme.title(title);
    } else if interactive {
        let title = ui.take_title()?;
        name_scheme.title(title);
    }

    if let Some(keywords) = keywords {
        let keywords = Keywords::parse_user_input(&keywords);
        name_scheme.keywords(keywords);
    } else if interactive {
        let keywords = ui.take_keywords()?;
        name_scheme.keywords(keywords);
    }

    if let Some(extension) = extension {
        let extension = Extension::new(extension);
        name_scheme.extension(extension);
    } else if interactive {
        let extension = ui.take_extension()?;
        name_scheme.extension(extension);
    }

    let file_name = name_scheme.to_string();

    if !accept {
        let accepted = ui.question(&format!("Create file \"{file_name}\"?"), true)?;
        if !accepted {
            UI::no_action_needed();
            return Ok(());
        }
    }

    fs::File::create(file_name).context("Failed to create file")?;

    Ok(())
}

fn rename_file(
    file_name: String,
    date: Option<&str>,
    date_from_metadata: bool,
    signature: Option<&str>,
    title: Option<&str>,
    keywords: Option<&str>,
    extension: Option<&str>,
    non_interactive: bool,
    accept: bool,
) -> Result<()> {
    let mut io = UI::new();

    let path = PathBuf::from(&file_name);

    if !path.exists() {
        bail!("File doesn't exists");
    }
    if path.is_dir() {
        bail!("Renaming directories are not supported");
    }

    let file_title = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let current_name_scheme = NameScheme::from_path(&path);

    let identifier = if date_from_metadata {
        Identifier::from_file_metadata(&path)?
    } else if let Some(date) = date {
        Identifier::parse(&date)?
    } else {
        current_name_scheme
            .as_ref()
            .map(|ns| ns.identifier.clone())
            .unwrap_or_default()
    };

    let interactive = !non_interactive;

    let mut name_scheme = NameScheme::new(identifier);

    if let Some(signature) = signature {
        let signature = Signature::parse(signature);
        name_scheme.signature(signature);
    } else if !interactive {
        name_scheme.signature = current_name_scheme
            .as_ref()
            .and_then(|ns| ns.signature.clone());
    }

    if let Some(title) = title {
        let title = Title::parse(title);
        name_scheme.title(title);
    } else if interactive {
        let old_title = current_name_scheme
            .as_ref()
            .and_then(|ns| ns.title.clone())
            .map(|title| title.desluggify())
            .unwrap_or(file_title);
        let title = io.title_with_old_title(&old_title)?;
        name_scheme.title(title);
    } else {
        name_scheme.title = current_name_scheme.as_ref().and_then(|ns| ns.title.clone())
    };

    if let Some(keywords) = keywords {
        let keywords = Keywords::parse_user_input(&keywords);
        name_scheme.keywords(keywords);
    } else if interactive {
        let keywords = io.take_keywords()?;
        name_scheme.keywords(keywords);
    };

    if let Some(extension) = extension {
        let extension = Extension::new(extension.to_string());
        name_scheme.extension(extension);
    } else {
        name_scheme.extension = current_name_scheme
            .as_ref()
            .and_then(|ns| ns.extension.clone());
    };

    let new_file_name = name_scheme.to_string();

    if file_name == new_file_name {
        UI::no_action_needed();
        return Ok(());
    }

    if !accept {
        println!("Old name \"{file_name}\"\nNew name \"{new_file_name}\"");
        let accepted = io.question("Accept?", true)?;
        if !accepted {
            UI::no_action_needed();
            return Ok(());
        }
    }

    fs::rename(&path, new_file_name).with_context(|| format!("Failed to rename file {path:?}"))?;

    Ok(())
}

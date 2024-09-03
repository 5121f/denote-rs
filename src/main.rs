/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod cli_args;
mod name_scheme;
mod ui;
mod utils;

use std::{fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;

use crate::{
    cli_args::Cli,
    name_scheme::{
        extention::Extention, identifier::Identifier, keywords::Keywords, signature::Signature,
        title::Title, NameScheme,
    },
    ui::UI,
};

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
                rename_file(
                    file_name,
                    date.as_deref(),
                    date_from_metadata,
                    signature.as_deref(),
                    title.as_deref(),
                    keywords.as_deref(),
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
            title,
            date,
            signature,
            keywords,
            extention,
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
    extention: Option<String>,
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
        name_scheme.title(Title::parse(&title));
    } else if interactive {
        let title = ui.take_title()?;
        name_scheme.title(title);
    }

    if let Some(keywords) = keywords {
        name_scheme.keywords(Keywords::parse_user_input(&keywords));
    } else if interactive {
        let keywords = ui.take_keywords()?;
        name_scheme.keywords(keywords);
    }

    if let Some(extention) = extention {
        name_scheme.extention(Extention::new(extention));
    } else if interactive {
        let extention = ui.take_extention()?;
        name_scheme.extention(extention);
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
    extention: Option<&str>,
    non_interactive: bool,
    accept: bool,
) -> Result<()> {
    let mut io = UI::new();

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
        name_scheme.signature(Signature::parse(signature));
    } else if !interactive {
        name_scheme.signature = current_name_scheme
            .as_ref()
            .and_then(|ns| ns.signature.clone());
    }

    if let Some(title) = title {
        name_scheme.title(Title::parse(title));
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

    if let Some(extention) = extention {
        let extention = Extention::new(extention.to_string());
        name_scheme.extention(extention);
    } else {
        name_scheme.extention = current_name_scheme
            .as_ref()
            .and_then(|ns| ns.extention.clone());
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

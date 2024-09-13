/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{fs, path::Path, process::Stdio};

use anyhow::{Context, Result};
use denote::{Denote, Extension, Identifier, Keywords, Signature, Title};

use crate::ui::UI;

pub fn build_denote(
    ui: &mut UI,
    title: Option<String>,
    date: String,
    signature: Option<String>,
    keywords: Option<String>,
    extension: Option<String>,
    non_interactive: bool,
) -> Result<Denote> {
    let identifier = Identifier::parse(&date).context("Failed to parse identifier")?;

    let interactive = !non_interactive;

    let mut name_scheme = Denote::new(identifier);

    if let Some(signature) = signature {
        name_scheme.signature = Signature::parse(&signature)
    }

    if let Some(title) = title {
        name_scheme.title = Title::parse(&title);
    } else if interactive {
        name_scheme.title = ui.take_title()?;
    }

    if let Some(keywords) = keywords {
        name_scheme.keywords = Keywords::parse_user_input(&keywords);
    } else if interactive {
        name_scheme.keywords = ui.take_keywords()?;
    }

    if let Some(extension) = &extension {
        name_scheme.extension = Extension::new(extension);
    } else if interactive {
        name_scheme.extension = ui.take_extension()?;
    }

    Ok(name_scheme)
}

pub fn create_file(file_name: impl AsRef<Path>) -> Result<()> {
    fs::File::create(&file_name).context("Failed to create file")?;
    Ok(())
}

pub fn open_file(file_name: impl AsRef<Path>) -> Result<()> {
    let editor = std::env::var("EDITOR").context("EDITOR environment variable don't set")?;
    let mut cmd = std::process::Command::new(editor);
    cmd.arg(file_name.as_ref()).stdout(Stdio::inherit());
    cmd.output()?;
    Ok(())
}

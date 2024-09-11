/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fs;

use anyhow::{Context, Result};
use denote::{Extension, Identifier, Keywords, NameScheme, Signature, Title};

use crate::ui::UI;

pub fn touch(
    title: Option<String>,
    date: String,
    signature: Option<String>,
    keywords: Option<String>,
    extension: Option<String>,
    non_interactive: bool,
    accept: bool,
) -> Result<()> {
    let mut ui = UI::new();

    let identifier = Identifier::parse(&date).context("Failed to parse identifier")?;

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

    if let Some(extension) = &extension {
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

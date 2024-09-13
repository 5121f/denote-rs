/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fs;

use anyhow::{Context, Result};
use denote::{Denote, Extension, Identifier, Keywords, Signature, Title};

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

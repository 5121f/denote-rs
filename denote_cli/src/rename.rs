/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{fs, path::Path};

use anyhow::{bail, Context, Result};
use denote::{Denote, Extension, Identifier, Keywords, Signature, Title};

use crate::ui::UI;

#[allow(clippy::too_many_arguments)]
pub fn rename(
    ui: &mut UI,
    path: &Path,
    date: Option<&str>,
    date_from_metadata: bool,
    signature: Option<&str>,
    title: Option<&str>,
    keywords: Option<&str>,
    extension: Option<&str>,
    non_interactive: bool,
    accept: bool,
) -> Result<()> {
    if !path.exists() {
        bail!("File doesn't exists");
    }
    if path.is_dir() {
        bail!("Renaming directories is not supported");
    }

    let file_title = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let current_name_scheme = Denote::from_path(path);

    let identifier = if date_from_metadata {
        Identifier::from_file_metadata(path)?
    } else if let Some(date) = date {
        Identifier::parse(date).unwrap_or_default()
    } else {
        current_name_scheme
            .as_ref()
            .map(|cns| cns.identifier.clone())
            .unwrap_or_default()
    };

    let interactive = !non_interactive;

    let mut name_scheme = Denote::new(identifier);

    if let Some(signature) = signature {
        name_scheme.signature = Signature::parse(signature);
    } else if !interactive {
        if let Some(cns) = &current_name_scheme {
            name_scheme.signature = cns.signature.clone();
        }
    }

    if let Some(title) = title {
        name_scheme.title = Title::parse(title);
    } else if interactive {
        let old_title = current_name_scheme
            .as_ref()
            .and_then(|ns| ns.title.clone())
            .map(|title| title.desluggify())
            .unwrap_or(file_title.clone());
        name_scheme.title = ui.title_with_old_title(&old_title)?;
    } else if let Some(cns) = &current_name_scheme {
        name_scheme.title = cns.title.clone();
    };

    if let Some(keywords) = keywords {
        name_scheme.keywords = Keywords::parse_user_input(keywords);
    } else if interactive {
        name_scheme.keywords = ui.take_keywords()?;
    };

    if let Some(extension) = extension {
        name_scheme.extension = Extension::new(extension);
    } else if let Some(cns) = &current_name_scheme {
        name_scheme.extension = cns.extension.clone()
    };

    let new_file_name = name_scheme.to_string();

    if file_title == new_file_name {
        UI::no_action_needed();
        return Ok(());
    }

    if !accept && !ui.rename(&file_title, &new_file_name)? {
        UI::no_action_needed();
        return Ok(());
    }

    fs::rename(path, new_file_name).with_context(|| format!("Failed to rename file {path:?}"))?;

    Ok(())
}

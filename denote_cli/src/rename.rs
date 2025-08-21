/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use anyhow::{Context, bail};
use denote::{Denote, Extension, Identifier, Keywords, Signature, Title};
use fs_err as fs;

use crate::args;
use crate::ui::{Answer, UI};

pub fn rename(args: &args::Rename, ui: &mut UI) -> anyhow::Result<()> {
    if args.paths.len() > 1 && unic_id(args.date.as_deref()) {
        let accept = ui.confirm(
            "It is not recommended to use one unique identifier for several files\nContinue?",
            Answer::No,
        )?;
        if !accept.as_bool() {
            UI::no_action_needed();
            return Ok(());
        }
    }

    for path in &args.paths {
        if !path.exists() {
            bail!("File doesn't exists");
        }
        if path.is_dir() {
            bail!("Renaming directories is not supported");
        }

        let current_name_scheme = Denote::from_path(path);

        let identifier = if args.date_from_metadata {
            Identifier::from_file_metadata(path)?
        } else if let Some(date) = &args.date {
            Identifier::parse(date).unwrap_or_default()
        } else {
            current_name_scheme
                .as_ref()
                .map(|cns| cns.identifier.clone())
                .unwrap_or_default()
        };

        let interactive = !args.non_interactive;

        let mut name_scheme = Denote::new(identifier);

        if let Some(signature) = &args.signature {
            name_scheme.signature = Signature::parse(signature);
        } else if !interactive && let Some(cns) = &current_name_scheme {
            name_scheme.signature.clone_from(&cns.signature);
        }

        if let Some(title) = &args.title {
            name_scheme.title = Title::parse(title);
        } else if interactive {
            let file_title = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            let old_title = current_name_scheme
                .as_ref()
                .and_then(|ns| ns.title.clone())
                .map_or(file_title, |title| title.desluggify());
            name_scheme.title = ui.title_with_old_title(&old_title)?;
        } else if let Some(cns) = &current_name_scheme {
            name_scheme.title.clone_from(&cns.title);
        }

        if let Some(keywords) = &args.keywords {
            name_scheme.keywords = Keywords::parse_user_input(keywords);
        } else if interactive {
            name_scheme.keywords = ui.take_keywords()?;
        }

        if let Some(extension) = &args.extension {
            name_scheme.extension = Extension::new(extension);
        } else if let Some(cns) = &current_name_scheme {
            name_scheme.extension.clone_from(&cns.extension);
        } else if let Some(ext) = Extension::from_path(path) {
            name_scheme.extension(ext);
        }

        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        let new_file_name = name_scheme.to_string();

        if file_name == new_file_name {
            UI::no_action_needed();
            return Ok(());
        }

        if !args.accept && !ui.rename(&file_name, &new_file_name)?.as_bool() {
            UI::no_action_needed();
            return Ok(());
        }

        let parent = path.parent().with_context(|| {
            format!("Failed to find parent of dir '{}'", path.to_string_lossy())
        })?;
        let new_path = parent.join(&new_file_name);

        fs::rename(path, new_path)?;
    }

    Ok(())
}

fn unic_id(date: Option<&str>) -> bool {
    let Some(date) = &date else {
        return false;
    };
    let identifier = Identifier::find_in_string(date);
    identifier.is_some()
}

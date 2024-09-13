/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod args;
mod rename;
mod touch;
mod ui;

use anyhow::{bail, Result};
use clap::Parser;
use denote::{Identifier, Keywords, Title};

use args::Args;
use rename::rename;
use touch::{build_denote, create_file, open_file};
use ui::UI;

fn main() -> Result<()> {
    let cli = Args::parse();

    let mut ui = UI::new();

    match cli {
        Args::Rename {
            paths,
            date,
            date_from_metadata,
            signature,
            title,
            keywords,
            extension,
            non_interactive,
            accept,
        } => {
            if paths.len() > 1 && unic_id(date.as_deref()) {
                let accept = ui.question(
                    "It is not recommended to use one unique identifier for several files\nContinue?",
                    false,
                )?;
                if !accept {
                    UI::no_action_needed();
                    return Ok(());
                }
            }

            for path in &paths {
                if !path.exists() {
                    bail!("File doesn't exists");
                }
                if path.is_dir() {
                    bail!("Renaming directories is not supported");
                }

                let denote = rename::build_denote(
                    &mut ui,
                    path,
                    date.as_deref(),
                    date_from_metadata,
                    signature.as_deref(),
                    title.as_deref(),
                    keywords.as_deref(),
                    extension.as_deref(),
                    non_interactive,
                )?;

                let file_name = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();

                let new_file_name = denote.to_string();

                if file_name == new_file_name {
                    UI::no_action_needed();
                    return Ok(());
                }

                if !accept && !ui.rename(&file_name, &new_file_name)? {
                    UI::no_action_needed();
                    return Ok(());
                }

                rename(path, &new_file_name)?;
            }
        }
        Args::Touch {
            title,
            date,
            signature,
            keywords,
            extension,
            non_interactive,
            accept,
            open,
        } => {
            let denote = build_denote(
                &mut ui,
                title,
                date,
                signature,
                keywords,
                extension,
                non_interactive,
            )?;

            let file_name = denote.to_string();

            if !accept && !ui.create_file_p(&file_name)? {
                UI::no_action_needed();
                return Ok(());
            }

            create_file(&file_name)?;

            if open {
                open_file(&file_name)?;
            }
        }
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

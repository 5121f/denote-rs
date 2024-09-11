/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod args;
mod rename;
mod touch;
mod ui;

use anyhow::Result;
use clap::Parser;
use denote::{Identifier, Keywords, Title};

use args::Args;
use rename::rename;
use touch::touch;
use ui::UI;

fn main() -> Result<()> {
    let cli = Args::parse();

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
            let mut ui = UI::new();

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
                rename(
                    path,
                    date.as_deref(),
                    date_from_metadata,
                    signature.as_deref(),
                    title.as_deref(),
                    keywords.as_deref(),
                    extension.as_deref(),
                    non_interactive,
                    accept,
                    &mut ui,
                )?;
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

fn unic_id(date: Option<&str>) -> bool {
    let Some(date) = &date else {
        return false;
    };
    let identifier = Identifier::find_in_string(date);
    identifier.is_some()
}

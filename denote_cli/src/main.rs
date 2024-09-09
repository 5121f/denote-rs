/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod cli_args;
mod rename;
mod touch;
mod ui;

use anyhow::Result;
use clap::Parser;
use denote::{Extension, Identifier, Keywords, Title};

use cli_args::Cli;
use rename::rename;
use touch::touch;
use ui::UI;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Rename {
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

            if paths.len() > 1 && !sisngle_unic_id(date.as_deref(), &mut ui)? {
                UI::no_action_needed();
                return Ok(());
            }

            for path in paths {
                rename(
                    &path,
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

fn sisngle_unic_id(date: Option<&str>, ui: &mut UI) -> Result<bool> {
    let Some(date) = &date else {
        return Ok(false);
    };
    if Identifier::find_in_string(date).is_none() {
        return Ok(false);
    };
    let accept = ui.question(
        "It is not recommended to use one unique identifier for several files\nContinue?",
        false,
    )?;
    Ok(accept)
}

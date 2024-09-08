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
use denote::{Extension, Keywords, Title};

use cli_args::Cli;
use rename::rename;
use touch::touch;

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

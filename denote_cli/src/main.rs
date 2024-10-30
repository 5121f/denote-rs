/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![warn(clippy::nursery)]

mod args;
mod rename;
mod touch;
mod ui;

use anyhow::Result;
use clap::Parser;

use args::Args;
use rename::rename;
use touch::touch;
use ui::UI;

fn main() -> Result<()> {
    let cli = Args::parse();
    let mut ui = UI::new();

    match cli {
        Args::Rename(args) => rename(args, &mut ui)?,
        Args::Touch(args) => touch(args, &mut ui)?,
    }

    Ok(())
}

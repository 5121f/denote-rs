// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

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
        Args::Rename(args) => rename(&args, &mut ui)?,
        Args::Touch(args) => touch(args, &mut ui)?,
    }

    Ok(())
}

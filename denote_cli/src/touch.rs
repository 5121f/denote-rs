/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{path::Path, process::Stdio};

use anyhow::{Context, Result};
use denote::{Denote, Extension, Identifier, Keywords, Signature, Title};
use fs_err as fs;

use crate::{args, ui::UI};

pub fn touch(args: args::Touch, ui: &mut UI) -> anyhow::Result<()> {
    let identifier = Identifier::parse(&args.date).context("Failed to parse identifier")?;

    let interactive = !args.non_interactive;

    let mut name_scheme = Denote::new(identifier);

    if let Some(signature) = args.signature {
        name_scheme.signature = Signature::parse(&signature);
    }

    if let Some(title) = args.title {
        name_scheme.title = Title::parse(&title);
    } else if interactive {
        name_scheme.title = ui.take_title()?;
    }

    if let Some(keywords) = args.keywords {
        name_scheme.keywords = Keywords::parse_user_input(&keywords);
    } else if interactive {
        name_scheme.keywords = ui.take_keywords()?;
    }

    if let Some(extension) = &args.extension {
        name_scheme.extension = Extension::new(extension);
    } else if interactive {
        name_scheme.extension = ui.take_extension()?;
    }

    let file_name = name_scheme.to_string();

    if !args.accept && !*ui.create_file_p(&file_name)? {
        UI::no_action_needed();
        return Ok(());
    }

    fs::File::create(&file_name)?;

    if args.open {
        open_file(&file_name)?;
    }

    Ok(())
}

pub fn open_file(file_name: impl AsRef<Path>) -> Result<()> {
    let editor = std::env::var("EDITOR").context("EDITOR environment variable don't set")?;
    let mut cmd = std::process::Command::new(editor);
    cmd.arg(file_name.as_ref()).stdout(Stdio::inherit());
    cmd.output()?;
    Ok(())
}

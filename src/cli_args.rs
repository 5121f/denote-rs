/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use clap::Parser;

#[derive(Parser)]
#[command(version = clap::crate_version!())]
pub(crate) enum Cli {
    Rename {
        #[clap(required = true)]
        file_names: Vec<String>,
        /// Date of file creating. You can use "now" to use current system time
        #[clap(long, short, conflicts_with("date_from_metadata"))]
        date: Option<String>,
        /// Use date from file metadata
        #[clap(long, short = 'm', conflicts_with("date"))]
        date_from_metadata: bool,
        /// Accept all questions
        #[clap(long, short)]
        accept: bool,
        /// Don't use keywords
        #[clap(long, short = 'k')]
        no_keywords: bool,
    },
    Touch {
        /// Date of file creating. You can use "now" to use current system time
        #[clap(long, short)]
        date: Option<String>,
    },
}

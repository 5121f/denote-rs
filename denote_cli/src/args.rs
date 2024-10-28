/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version = clap::crate_version!(), about = clap::crate_description!())]
pub enum Args {
    /// Rename existing file
    Rename {
        #[clap(required = true)]
        paths: Vec<PathBuf>,
        /// The date to be used as the identifier.
        /// Can be used a date like 2022-06-30 or a date and time like 2022-06-16 14:30.
        /// Can be used "now" to use current system date and time.
        /// Can be used schemed format like 20240903T21133121.
        #[clap(long, short, conflicts_with("date_from_metadata"))]
        date: Option<String>,
        /// Use creation file date from the file metadata
        #[clap(long, short = 'm', conflicts_with("date"))]
        date_from_metadata: bool,
        /// Specify the signature
        #[clap(long, short)]
        signature: Option<String>,
        /// Specify the title
        #[clap(long, short)]
        title: Option<String>,
        /// Specify the keywords
        #[clap(long, short)]
        keywords: Option<String>,
        /// Specify the v
        #[clap(long, short)]
        extension: Option<String>,
        /// Don't ask anything. Use default values if not specified
        #[clap(long, short)]
        non_interactive: bool,
        /// Don't ask confirmation to rename file
        #[clap(long, short)]
        accept: bool,
    },
    /// Create new file
    Touch {
        /// Specify the title
        title: Option<String>,
        /// The date to be used as the identifier.
        /// Can be used a date like 2022-06-30 or a date and time like 2022-06-16 14:30.
        /// Can be used "now" to use current system date and time.
        /// Can be used schemed format like 20240903T21133121.
        #[clap(long, short, default_value = "now")]
        date: String,
        /// Specify the signature
        #[clap(long, short)]
        signature: Option<String>,
        /// Specify the keywords
        #[clap(long, short)]
        keywords: Option<String>,
        /// Specify the file extension
        #[clap(long, short)]
        extension: Option<String>,
        /// Don't ask anything. Use default values if not specified
        #[clap(long, short)]
        non_interactive: bool,
        /// Don't ask confirmation to create file
        #[clap(long, short)]
        accept: bool,
        /// Open created file in default text editor
        #[clap(long, short)]
        open: bool,
    },
}

// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use clap::Parser;

use std::path::PathBuf;

/// Rename existing file
#[derive(Parser)]
pub struct Rename {
    #[clap(required = true)]
    pub paths: Vec<PathBuf>,
    /// The date to be used as the identifier.
    /// Can be used a date like 2022-06-30 or a date and time like 2022-06-16 14:30.
    /// Can be used "now" to use current system date and time.
    /// Can be used schemed format like 20240903T21133121.
    #[clap(long, short, conflicts_with("date_from_metadata"))]
    pub date: Option<String>,
    /// Use creation file date from the file metadata
    #[clap(long, short = 'm', conflicts_with("date"))]
    pub date_from_metadata: bool,
    /// Specify the signature
    #[clap(long, short)]
    pub signature: Option<String>,
    /// Specify the title
    #[clap(long, short)]
    pub title: Option<String>,
    /// Specify the keywords
    #[clap(long, short)]
    pub keywords: Option<String>,
    /// Specify the v
    #[clap(long, short)]
    pub extension: Option<String>,
    /// Don't ask anything. Use default values if not specified
    #[clap(long, short)]
    pub non_interactive: bool,
    /// Don't ask confirmation to rename file
    #[clap(long, short)]
    pub accept: bool,
}

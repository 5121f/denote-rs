/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use clap::Parser;

/// Create new file
#[derive(Parser)]
pub struct Touch {
    /// Specify the title
    pub title: Option<String>,
    /// The date to be used as the identifier.
    /// Can be used a date like 2022-06-30 or a date and time like 2022-06-16 14:30.
    /// Can be used "now" to use current system date and time.
    /// Can be used schemed format like 20240903T21133121.
    #[clap(long, short, default_value = "now")]
    pub date: String,
    /// Specify the signature
    #[clap(long, short)]
    pub signature: Option<String>,
    /// Specify the keywords
    #[clap(long, short)]
    pub keywords: Option<String>,
    /// Specify the file extension
    #[clap(long, short)]
    pub extension: Option<String>,
    /// Don't ask anything. Use default values if not specified
    #[clap(long, short)]
    pub non_interactive: bool,
    /// Don't ask confirmation to create file
    #[clap(long, short)]
    pub accept: bool,
    /// Open created file in default text editor
    #[clap(long, short)]
    pub open: bool,
}

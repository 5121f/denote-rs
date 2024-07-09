/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use clap::Parser;

#[derive(Parser)]
#[command(version = clap::crate_version!())]
pub(crate) enum Cli {
    #[clap(about = "Rename existing file")]
    Rename {
        #[clap(required = true)]
        file_names: Vec<String>,
        /// The date to be used as the identifier.
        /// You can use "now" to explicitly specify to use current system time
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
        /// Specify the extention
        #[clap(long, short)]
        extention: Option<String>,
        /// Use default values if not specified
        #[clap(long, short = 'f')]
        default: bool,
        /// Don't ask confirmation to rename file
        #[clap(long, short)]
        accept: bool,
    },
    #[clap(about = "Create new file")]
    Touch {
        /// Specify the title
        title: Option<String>,
        /// The date to be used as the identifier.
        /// By default used curent system time.
        /// Also you can use "now" to explicitly specify to use current system time
        #[clap(long, short)]
        date: Option<String>,
        /// Specify the signature
        #[clap(long, short)]
        signature: Option<String>,
        /// Specify the keywords
        #[clap(long, short)]
        keywords: Option<String>,
        /// Specify the extention
        #[clap(long, short)]
        extention: Option<String>,
        /// Use default values if not specified
        #[clap(long, short = 'f')]
        default: bool,
        /// Don't ask confirmation to create file
        #[clap(long, short)]
        accept: bool,
    },
}

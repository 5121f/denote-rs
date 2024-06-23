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
        /// Date of file creating. You can use "now" to use current system time
        #[clap(long, short, conflicts_with("date_from_metadata"))]
        date: Option<String>,
        /// Use date from file metadata
        #[clap(long, short = 'm', conflicts_with("date"))]
        date_from_metadata: bool,
        /// Specified title
        #[clap(long, short)]
        title: Option<String>,
        /// Specified keywords
        #[clap(long, short)]
        keywords: Option<String>,
        /// Use default value if not specified
        #[clap(long, short = 'f')]
        default: bool,
    },
    #[clap(about = "Create new file")]
    Touch {
        ///Specified title
        title: Option<String>,
        /// Date of file creating. By default used curent system time.
        /// Also you can use "now" to use current system time
        #[clap(long, short)]
        date: Option<String>,
    },
}

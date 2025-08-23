// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

mod rename;
mod touch;

pub use rename::Rename;
pub use touch::Touch;

use clap::Parser;

#[derive(Parser)]
#[command(version = clap::crate_version!(), about = clap::crate_description!())]
pub enum Args {
    Rename(Rename),
    Touch(Touch),
}

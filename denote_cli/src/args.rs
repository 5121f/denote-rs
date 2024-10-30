/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

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

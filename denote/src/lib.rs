/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod name_scheme;
#[cfg(test)]
mod tests;
mod utils;

pub use name_scheme::{Extension, Identifier, Keywords, NameScheme, Signature, Title};

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Handle denote name scheme.
//!
//! Core of this crate is the `Denote` struct. Create it, fill in the fields as an option and
//! convert it in to string to get formatted string in denote name scheme.
//!
//! # Example
//!
//! ```
//! use denote::{Denote, Identifier, Signature, Extension, Title};
//!
//! // You can use something like `Identifier::now()` but for example, we will take an already
//! // formatted identifier
//! let identifier = Identifier::parse("20240912T13015412").unwrap();
//! let denote = Denote::new(identifier)
//!     .title(Title::parse("Some title").unwrap())
//!     .signature(Signature::parse("1b").unwrap())
//!     .extension(Extension::new("txt"))
//!     .to_string();
//! assert_eq!(denote.to_string(), "20240912T13015412==1b--some-title.txt");
//! ```

mod denote;
mod extension;
mod format;
mod identifier;
mod keywords;
mod regex;
mod signature;
mod title;

pub use denote::Denote;
pub use extension::Extension;
pub use identifier::Identifier;
pub use keywords::Keywords;
pub use signature::Signature;
pub use title::Title;

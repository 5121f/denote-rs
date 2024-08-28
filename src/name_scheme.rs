/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub(crate) mod extention;
pub(crate) mod identifier;
pub(crate) mod keywords;
pub(crate) mod signature;
pub(crate) mod title;

use signature::Signature;

use self::{extention::Extention, identifier::Identifier, keywords::Keywords, title::Title};

#[derive(Default)]
pub struct NameScheme {
    pub identifier: Identifier,
    pub signature: Option<Signature>,
    pub title: Option<Title>,
    pub keywords: Option<Keywords>,
    pub extention: Option<Extention>,
}

impl NameScheme {
    pub fn new(identifier: Identifier) -> Self {
        Self {
            identifier,
            ..Default::default()
        }
    }
}

impl ToString for NameScheme {
    fn to_string(&self) -> String {
        let mut name_scheme = self.identifier.to_string();

        if let Some(signature) = &self.signature {
            name_scheme.push_str(&signature.to_string());
        }

        if let Some(title) = &self.title {
            name_scheme.push_str(&title.to_string());
        }

        if let Some(keywords) = &self.keywords {
            name_scheme.push_str(&keywords.to_string());
        }

        if let Some(extention) = &self.extention {
            name_scheme.push_str(&extention.to_string());
        }

        name_scheme
    }
}

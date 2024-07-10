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

        let signature = self
            .signature
            .as_ref()
            .map(|signature| signature.into_string());
        name_scheme = maybe_add(name_scheme, signature.as_deref());

        let title = self.title.as_ref().map(|title| title.to_string());
        name_scheme = maybe_add(name_scheme, title.as_deref());

        let keywords = self.keywords.as_ref().map(|keywrds| keywrds.to_string());
        name_scheme = maybe_add(name_scheme, keywords.as_deref());

        let extention = self.extention.as_ref().map(|ext| ext.to_string());
        name_scheme = maybe_add(name_scheme, extention.as_deref());

        name_scheme
    }
}

impl Default for NameScheme {
    fn default() -> Self {
        Self {
            identifier: Identifier::now(),
            signature: Default::default(),
            title: Default::default(),
            keywords: Default::default(),
            extention: Default::default(),
        }
    }
}

fn maybe_add<'a, 'b>(base: String, added: Option<&str>) -> String {
    if let Some(added) = added {
        return format!("{}{}", base, added);
    }
    base
}

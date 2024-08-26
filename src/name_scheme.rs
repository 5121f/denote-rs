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

        let signature = self
            .signature
            .as_ref()
            .map(|signature| signature.into_string());
        maybe_add(&mut name_scheme, signature.as_deref());

        let title = self.title.as_ref().map(|title| title.to_string());
        maybe_add(&mut name_scheme, title.as_deref());

        let keywords = self.keywords.as_ref().map(|keywrds| keywrds.to_string());
        maybe_add(&mut name_scheme, keywords.as_deref());

        let extention = self.extention.as_ref().map(|ext| ext.to_string());
        maybe_add(&mut name_scheme, extention.as_deref());

        name_scheme
    }
}

fn maybe_add<'a, 'b>(base: &mut String, added: Option<&str>) {
    if let Some(added) = added {
        base.push_str(added);
    }
}

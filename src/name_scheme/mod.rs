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

pub(crate) fn name_scheme(
    identifier: Identifier,
    signature: Option<Signature>,
    title: Option<Title>,
    keywords: Option<Keywords>,
    extention: Option<Extention>,
) -> String {
    let mut name_scheme = identifier.into_string();

    let signature = signature.map(|signature| signature.into_string());
    name_scheme = maybe_add(name_scheme, signature.as_deref());

    let title = title.map(|title| title.into_string());
    name_scheme = maybe_add(name_scheme, title.as_deref());

    let keywords = keywords.map(|keywrds| keywrds.to_string());
    name_scheme = maybe_add(name_scheme, keywords.as_deref());

    let extention = extention.map(|ext| ext.to_string());
    name_scheme = maybe_add(name_scheme, extention.as_deref());

    name_scheme
}

fn maybe_add<'a, 'b>(base: String, added: Option<&str>) -> String {
    if let Some(added) = added {
        return format!("{}{}", base, added);
    }
    base
}

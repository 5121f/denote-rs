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

use pomsky_macro::pomsky;
use regex::Regex;
use signature::Signature;

use std::{
    fmt::{self, Display},
    path::Path,
};

use self::{extention::Extention, identifier::Identifier, keywords::Keywords, title::Title};

const NAME_SHCHEME_REGEXP: &str = pomsky!(
    ^
    :id([digit]{8} 'T' [digit]{8})
    ("==" :signature([Alphabetic] ([Alphabetic '='])*))?
    ("--" :title([Alphabetic] ([Alphabetic '-']*)))?
    ("__" :keywords([Alphabetic] ([Alphabetic '_']*)))?
    ('.' :ext([Alphabetic]+))?
    $
);

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

    pub fn from_path(path: &Path) -> Option<Self> {
        let regex = Regex::new(NAME_SHCHEME_REGEXP).unwrap();
        let file_name = path.file_name()?.to_str()?;
        let captures = regex.captures(file_name)?;

        let id = {
            let capture = captures.name("id")?;
            Identifier::new(capture.as_str().to_string())
        };

        let mut name_scheme = Self::new(id);

        name_scheme.signature = captures
            .name("signature")
            .map(|c| c.as_str())
            .and_then(Signature::parse);

        name_scheme.title = captures
            .name("title")
            .map(|c| c.as_str())
            .and_then(Title::parse);

        name_scheme.keywords = captures
            .name("keywords")
            .map(|c| c.as_str())
            .and_then(Keywords::from_string);

        name_scheme.extention = captures
            .name("ext")
            .map(|c| c.as_str().to_string())
            .and_then(Extention::new);

        Some(name_scheme)
    }
}

impl Display for NameScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)?;

        if let Some(signature) = &self.signature {
            write!(f, "{signature}")?;
        }

        if let Some(title) = &self.title {
            write!(f, "{title}")?;
        }

        if let Some(keywords) = &self.keywords {
            write!(f, "{keywords}")?;
        }

        if let Some(extention) = &self.extention {
            write!(f, "{extention}")?;
        }

        fmt::Result::Ok(())
    }
}

#[test]
fn name_scheme_regexp() {
    Regex::new(NAME_SHCHEME_REGEXP).unwrap();
}

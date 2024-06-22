/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub(crate) mod extention;
pub(crate) mod identifier;
pub(crate) mod keywords;
pub(crate) mod title;

use crate::io::Io;

use self::{extention::Extention, identifier::Identifier, keywords::Keywords, title::Title};
use anyhow::Result;

pub(crate) struct NameScheme {
    title: Title,
    keywords: Option<Keywords>,
    identifier: Identifier,
    extention: Option<Extention>,
}

impl NameScheme {
    pub(crate) fn into_string(self) -> String {
        let mut name_scheme = format!(
            "{}{}",
            self.identifier.into_string(),
            self.title.into_string(),
        );

        let keywords = self.keywords.map(|keywrds| keywrds.to_string());
        name_scheme = maybe_add(name_scheme, keywords.as_deref());

        let extention = self.extention.map(|ext| ext.to_string());
        name_scheme = maybe_add(name_scheme, extention.as_deref());

        name_scheme
    }
}

#[derive(Default)]
pub(crate) struct NameSchemeBuilder {
    title: Option<Title>,
    keywords: Option<Keywords>,
    identifier: Option<Identifier>,
    extention: Option<Extention>,
}

impl NameSchemeBuilder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn take_title_from_user_with_old_title(
        &mut self,
        io: &mut Io,
        old_title: &str,
    ) -> Result<&mut Self> {
        io.print(&format!("Title [{}]: ", &old_title))?;
        let user_input = io.read_line()?;
        let title = if user_input.trim().is_empty() {
            old_title.to_owned()
        } else {
            user_input
        };
        let title = Title::parse(&title)?;
        self.title = Some(title);
        Ok(self)
    }

    pub(crate) fn take_title_from_user(&mut self, io: &mut Io) -> Result<&mut Self> {
        io.print("Title: ")?;
        let title = Title::parse(&io.read_line()?)?;
        self.title = Some(title);
        Ok(self)
    }

    pub(crate) fn take_keywords_from_user(&mut self, io: &mut Io) -> Result<&mut Self> {
        io.print("Keywords: ")?;
        self.keywords = Keywords::from_string(&io.read_line()?);
        Ok(self)
    }

    pub(crate) fn title(&mut self, title: Title) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub(crate) fn identifier(&mut self, identifier: Identifier) -> &mut Self {
        self.identifier = Some(identifier);
        self
    }

    pub(crate) fn extention(&mut self, extention: String) -> &mut Self {
        self.extention = Extention::new(extention);
        self
    }

    pub(crate) fn take_extention_from_user(&mut self, io: &mut Io) -> Result<&mut Self> {
        io.print("Extention: ")?;
        let extention = io.read_line()?;
        self.extention = Extention::new(extention);
        Ok(self)
    }

    pub(crate) fn build(self) -> NameScheme {
        NameScheme {
            title: self.title.unwrap_or_default(),
            keywords: self.keywords,
            identifier: self.identifier.unwrap_or_else(Identifier::now),
            extention: self.extention,
        }
    }
}

fn maybe_add<'a, 'b>(base: String, added: Option<&str>) -> String {
    if let Some(added) = added {
        return format!("{}{}", base, added);
    }
    base
}

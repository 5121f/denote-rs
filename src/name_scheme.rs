pub(crate) mod identifier;
pub(crate) mod keywords;
pub(crate) mod title;

use crate::io::Io;

use self::{identifier::Identifier, keywords::Keywords, title::Title};
use anyhow::Result;

pub(crate) struct NameScheme {
    title: Title,
    keywords: Keywords,
    identifier: Identifier,
    extention: Option<String>,
}

impl NameScheme {
    pub(crate) fn into_string(self) -> String {
        if let Some(extention) = &self.extention {
            return format!(
                "{}{}{}.{}",
                self.identifier.into_string(),
                self.title.into_string(),
                self.keywords.into_string(),
                extention
            );
        }
        format!(
            "{}{}{}",
            self.identifier.into_string(),
            self.title.into_string(),
            self.keywords.into_string(),
        )
    }
}

#[derive(Default)]
pub(crate) struct NameSchemeBuilder {
    title: Option<Title>,
    keywords: Option<Keywords>,
    identifier: Option<Identifier>,
    extention: Option<String>,
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
        let keywords = Keywords::from_string(&io.read_line()?);
        self.keywords = Some(keywords);
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
        self.extention = Some(extention);
        self
    }

    pub(crate) fn take_extention_from_user(&mut self, io: &mut Io) -> Result<&mut Self> {
        io.print("Extention: ")?;
        let extention = io.read_line()?;
        if !extention.is_empty() {
            self.extention = Some(extention);
        }
        Ok(self)
    }

    pub(crate) fn build(self) -> NameScheme {
        NameScheme {
            title: self.title.unwrap_or_default(),
            keywords: self.keywords.unwrap_or_default(),
            identifier: self.identifier.unwrap_or_else(Identifier::now),
            extention: self.extention,
        }
    }
}

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

impl ToString for NameScheme {
    fn to_string(&self) -> String {
        if let Some(extention) = &self.extention {
            format!(
                "{}{}{}.{}",
                self.identifier.to_string(),
                self.title.to_string(),
                self.keywords.to_string(),
                extention
            )
        } else {
            [
                self.identifier.to_string(),
                self.title.to_string(),
                self.keywords.to_string(),
            ]
            .concat()
        }
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
        mut self,
        io: &mut Io,
        old_title: &str,
    ) -> Result<Self> {
        io.print(&format!("Заголовок [{}]: ", &old_title))?;
        let title = Title::from_string(
            &Some(io.read_line()?)
                .filter(|f| !f.trim().is_empty())
                .unwrap_or(old_title.to_owned()),
        );
        self.title = Some(title);
        Ok(self)
    }

    pub(crate) fn take_title_from_user(mut self, io: &mut Io) -> Result<Self> {
        io.print("Заголовок: ")?;
        let title = Title::from_string(&io.read_line()?);
        self.title = Some(title);
        Ok(self)
    }

    pub(crate) fn take_keywords_from_user(mut self, io: &mut Io) -> Result<Self> {
        io.print("Ключевые слова: ")?;
        let keywords = Keywords::from_string(&io.read_line()?);
        self.keywords = Some(keywords);
        Ok(self)
    }

    pub(crate) fn identifier(mut self, identifier: Identifier) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub(crate) fn extention(mut self, extention: String) -> Self {
        self.extention = Some(extention);
        self
    }

    pub(crate) fn take_extention_from_user(mut self, io: &mut Io) -> Result<Self> {
        io.print("Расширение: ")?;
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
            identifier: self.identifier.unwrap_or_else(Identifier::current_time),
            extention: self.extention,
        }
    }
}

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod identifier;
mod keywords;
mod regex;
mod signature;
mod title;

use std::fmt::{self, Display};
use std::path::{Path, PathBuf};

pub use identifier::{Error as IdentifierError, Identifier};
pub use keywords::Keywords;
pub use signature::Signature;
pub use title::Title;

use crate::utils::{self, FileNameError};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct NameScheme {
    pub identifier: Identifier,
    pub signature: Option<Signature>,
    pub title: Option<Title>,
    pub keywords: Option<Keywords>,
    pub extension: Option<String>,
}

impl NameScheme {
    pub fn new(identifier: Identifier) -> Self {
        Self {
            identifier,
            ..Default::default()
        }
    }

    /// Trying find denote name scheme in file name. Returns `None` if name scheme didn't found.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use denote::{NameScheme, Title};
    ///
    /// let path = "20240903T13173023--title__keyword.txt";
    /// let mut name_scheme = NameScheme::from_path(path).unwrap();
    /// name_scheme.title(Title::parse("Another title"));
    /// assert_eq!(name_scheme.to_string(), "20240903T13173023--another-title__keyword.txt");
    /// ```
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let file_name = utils::take_file_name(&path)?;

        let captures = regex::NAME_SCHEME
            .captures(&file_name)
            .ok_or(Error::find(&path))?;

        let id = {
            let capture = captures.name("id").unwrap();
            Identifier::parse(capture.as_str())?
        };

        let mut name_scheme = Self::new(id);

        name_scheme.signature = captures
            .name("signature")
            .map(|c| c.as_str())
            .map(Signature::parse);

        name_scheme.title = captures.name("title").map(|c| c.as_str()).map(Title::parse);

        name_scheme.keywords = captures
            .name("keywords")
            .map(|c| c.as_str())
            .map(Keywords::parse_schemed_string);

        name_scheme.extension = captures.name("ext").map(|c| c.as_str().to_string());

        Ok(name_scheme)
    }

    pub fn signature(&mut self, signature: Signature) -> &mut Self {
        self.signature = Some(signature);
        self
    }

    pub fn title(&mut self, title: Title) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn keywords(&mut self, keywords: Keywords) -> &mut Self {
        self.keywords = Some(keywords);
        self
    }

    pub fn extension(&mut self, extension: String) -> &mut Self {
        self.extension = Some(extension);
        self
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

        if let Some(extension) = &self.extension {
            write!(f, ".{extension}")?;
        }

        fmt::Result::Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Name scheme didn't found in path: {path}")]
    Find { path: PathBuf },
    #[error("Identifier error: {0}")]
    Identifier(#[from] IdentifierError),
    #[error(transparent)]
    FileName(#[from] FileNameError),
}

impl Error {
    fn find(path: impl AsRef<Path>) -> Self {
        Self::Find {
            path: path.as_ref().to_path_buf(),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

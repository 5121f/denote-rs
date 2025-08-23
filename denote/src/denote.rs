// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use std::fmt::{self, Display};
use std::path::Path;

use crate::{Extension, Identifier, Keywords, Signature, Title, regex};

/// Handle denote name scheme
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Denote {
    pub identifier: Identifier,
    pub signature: Option<Signature>,
    pub title: Option<Title>,
    pub keywords: Option<Keywords>,
    pub extension: Option<Extension>,
}

impl Denote {
    /// Create Denote with Identifier and empty other fields
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
    /// use zeroten_denote::{Denote, Title};
    ///
    /// let path = "20240903T13173023--title__keyword.txt";
    /// let mut name_scheme = Denote::from_path(path).unwrap();
    /// name_scheme.title = Title::parse("Another title");
    /// assert_eq!(name_scheme.to_string(), "20240903T13173023--another-title__keyword.txt");
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        fn inner(path: &Path) -> Option<Denote> {
            let file_name = path.file_name()?.to_str()?.to_string();

            let captures = regex::NAME_SCHEME.captures(&file_name)?;

            let id = {
                let capture = captures.name("id").unwrap();
                Identifier::parse(capture.as_str())?
            };

            let mut name_scheme = Denote::new(id);

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
                .and_then(Keywords::parse_schemed_string);

            name_scheme.extension = captures
                .name("ext")
                .map(|c| c.as_str())
                .and_then(Extension::new);

            Some(name_scheme)
        }

        inner(path.as_ref())
    }

    /// Set signature
    pub fn signature(&mut self, signature: Signature) -> &mut Self {
        self.signature = Some(signature);
        self
    }

    /// Set title
    pub fn title(&mut self, title: Title) -> &mut Self {
        self.title = Some(title);
        self
    }

    /// Set keywords
    pub fn keywords(&mut self, keywords: Keywords) -> &mut Self {
        self.keywords = Some(keywords);
        self
    }

    /// Set extension
    pub fn extension(&mut self, extension: Extension) -> &mut Self {
        self.extension = Some(extension);
        self
    }
}

impl Display for Denote {
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
            write!(f, "{extension}")?;
        }

        fmt::Result::Ok(())
    }
}

use anyhow::{anyhow, Context, Result};
use regex::Regex;

const TITLE_REGEXP: &str = r"--([\p{Alphabetic}\pN-]*)";
const PUNCTUATION: &str = r"\p{P}";

#[derive(Default)]
pub(crate) struct Title(String);

impl Title {
    pub(crate) fn parse(string: &str) -> Result<Self> {
        let punctuation = Regex::new(PUNCTUATION).context("Failed to regex compile")?;
        let string = punctuation.replace_all(string, "");
        let string = string.trim().to_lowercase().replace(' ', "-");
        Ok(Self(string))
    }

    pub(crate) fn find_in_string(string: &str) -> Result<Self> {
        let capture = Regex::new(TITLE_REGEXP)
            .context("Failed to regex compile")?
            .captures(string)
            .ok_or_else(|| anyhow!("Extract title failed"))?;
        Ok(Self(capture[1].to_owned()))
    }

    pub(crate) fn desluggify(&self) -> String {
        let deslugify = self.0.clone().replace('-', " ");
        let mut chars = deslugify.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    pub(crate) fn into_string(self) -> String {
        format!("--{}", self.0)
    }
}

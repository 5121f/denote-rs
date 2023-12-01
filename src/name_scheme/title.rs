use anyhow::{anyhow, Context, Result};
use regex::Regex;

static TITLE_REGEXP: &str = r"--([\p{Alphabetic}\pN-]*)";

#[derive(Default)]
pub(crate) struct Title(String);

impl Title {
    pub(crate) fn from_string(string: &str) -> Self {
        Self(string.trim().to_lowercase().replace(' ', "-"))
    }

    pub(crate) fn extract_from_string(string: &str) -> Result<Self> {
        let capture = Regex::new(TITLE_REGEXP)
            .context("Произошла ошибка при компиляции регулярного выражения")?
            .captures(string)
            .ok_or_else(|| anyhow!("Не удалось извечь заголовок из строки"))?;
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
}

impl ToString for Title {
    fn to_string(&self) -> String {
        format!("--{}", self.0)
    }
}

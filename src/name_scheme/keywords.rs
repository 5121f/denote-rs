#[derive(Default)]
pub(crate) struct Keywords(Vec<String>);

impl Keywords {
    pub(crate) fn from_string(string: &str) -> Self {
        let keywords: Vec<_> = string
            .trim()
            .to_lowercase()
            .split(',')
            .map(ToOwned::to_owned)
            .collect();
        let keywords = match keywords.first() {
            Some(first) if first.is_empty() => Vec::new(),
            _ => keywords,
        };
        Self(keywords)
    }

    pub(crate) fn into_string(self) -> String {
        if self.0.is_empty() {
            return String::new();
        }
        format!("__{}", self.0.join("_"))
    }
}

#[derive(Default)]
pub(crate) struct Extention(Option<String>);

impl Extention {
    pub(crate) fn new(ext: String) -> Self {
        if ext.is_empty() {
            return Self(None);
        }
        Self(Some(ext))
    }

    pub(crate) fn to_string(self) -> Option<String> {
        self.0.map(|ext| format!(".{}", ext))
    }
}

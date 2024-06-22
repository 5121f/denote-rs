#[derive(Default)]
pub(crate) struct Extention(String);

impl Extention {
    pub(crate) fn new(ext: String) -> Option<Self> {
        if ext.is_empty() {
            return None;
        }
        Some(Self(ext))
    }

    pub(crate) fn to_string(&self) -> String {
        format!(".{}", self.0)
    }
}

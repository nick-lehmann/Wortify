pub struct Word(pub String);

impl Word {
    pub(crate) fn from(line: &str) -> Self {
        Self(line.to_lowercase().to_owned())
    }
}

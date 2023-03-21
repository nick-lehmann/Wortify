/// A word is simply a string that was converted to lowercase.
/// We use this type to make sure this invariant is always true.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word(String);

impl Word {
    pub(crate) fn from(line: &str) -> Self {
        Self(line.to_lowercase().to_owned())
    }
}

impl std::ops::Deref for Word {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl PartialEq<Word> for &Word {
    fn eq(&self, other: &Word) -> bool {
        self.0 == **other
    }
}

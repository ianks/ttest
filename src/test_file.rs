use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
};

/// A wrapper around a [`PathBuf`] that has some additional functionality.
#[derive(Debug, PartialEq, Clone)]
pub struct TestFile(PathBuf);

impl TestFile {
    /// Creates a new [`TestPath`] from a [`PathBuf`].
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    /// Tests if the path matches a pattern.
    pub fn matches(&self, pattern: &str) -> bool {
        self.0
            .to_str()
            .map(|path| path.contains(pattern))
            .unwrap_or(false)
    }

    /// Returns the path as a string.
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.0.to_string_lossy()
    }
}

impl AsRef<Path> for TestFile {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl From<PathBuf> for TestFile {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl From<&str> for TestFile {
    fn from(path: &str) -> Self {
        Self::new(path.into())
    }
}

impl Display for TestFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

use std::{
    error::Error,
    path::{Path, PathBuf},
};

use serde::Deserialize;

lazy_static::lazy_static! {
    static ref DEFAULT_CONFIG: Config = {
        Config::from_toml(include_str!("../ttest.toml")).unwrap()
    };
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct AdapterConfig {
    pub file_patterns: Vec<String>,
    pub test_patterns: Vec<String>,
    pub namespace_patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Config {
    ruby_rspec: AdapterConfig,
    ruby_minitest: AdapterConfig,
    cargo_test: AdapterConfig,
}

impl Config {
    /// Load the config from the given folder.
    #[allow(dead_code)]
    pub fn from_dir(cwd: &std::path::Path) -> Result<Option<Self>, Box<dyn Error>> {
        let config = if let Some(path) =
            find_project_file(cwd, &["ttest.toml", "_ttest.toml", ".ttest.toml"])
        {
            Some(Self::from_file(&path)?)
        } else {
            None
        };
        Ok(config)
    }

    /// Load the config from the given file.
    pub fn from_file(path: &std::path::Path) -> Result<Self, Box<dyn Error>> {
        let s = std::fs::read_to_string(path)?;
        Self::from_toml(&s)
    }

    /// Load the config from the given TOML string.
    pub fn from_toml(data: &str) -> Result<Self, Box<dyn Error>> {
        let content: Config = toml_edit::de::from_str(data)?;
        Ok(content)
    }

    /// Get a reference to the default config.
    #[allow(dead_code)]
    pub fn default() -> &'static Self {
        &*DEFAULT_CONFIG
    }
}

fn find_project_file(dir: impl AsRef<Path>, names: &[&str]) -> Option<PathBuf> {
    let mut file_path = dir.as_ref().join("placeholder");
    for name in names {
        file_path.set_file_name(name);
        if file_path.exists() {
            return Some(file_path);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_find_project_file() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let path = find_project_file(dir, &["ttest.toml"]);
        assert!(path.is_some());
        assert_eq!(path.unwrap().file_name().unwrap(), "ttest.toml");
    }

    #[test]
    fn test_find_project_file_not_found() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let path = find_project_file(dir, &["foo.toml"]);
        assert!(path.is_none());
    }

    #[test]
    fn test_from_dir() {
        let config = Config::default();

        assert_eq!(
            config.ruby_minitest.file_patterns,
            vec![r"(.*(^|/)(spec|test)/(test_.+|.+_test|.+_spec))\.rb$"]
        );

        assert_eq!(
            config.ruby_minitest.test_patterns[0],
            "\\s*(test|it)(\\(| )(\"|')(?P<name>.*)(\"|')"
        );

        assert_eq!(config.cargo_test.file_patterns, Vec::<String>::new());
    }
}

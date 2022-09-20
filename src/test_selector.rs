use std::{
    error::Error,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::test_file::TestFile;

#[derive(Debug, PartialEq, Clone)]
pub enum TestSelector {
    PathWithLineNumber { path: TestFile, line: u32 },
    PathOnly { path: TestFile },
    NameOnly { name: String },
}

impl FromStr for TestSelector {
    type Err = Box<dyn Error>;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Some((path, line)) = raw.split_once(':') {
            if !Path::new(&path).exists() {
                return Err(format!("File does not exist: {}", path).into());
            }

            let path: TestFile = path.into();
            let line = line.parse::<u32>()?;

            Ok(TestSelector::PathWithLineNumber { path, line })
        } else if PathBuf::from(&raw).exists() {
            Ok(TestSelector::PathOnly { path: raw.into() })
        } else {
            Ok(TestSelector::NameOnly { name: raw.into() })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_env, TestSelector};

    #[test]
    fn matches_path_with_line_number() {
        test_env::with(|env| {
            env.write_file("foo.rb", "puts 'hello'");

            let selector = "foo.rb:1".parse::<TestSelector>().unwrap();

            assert_eq!(
                selector,
                TestSelector::PathWithLineNumber {
                    path: "foo.rb".into(),
                    line: 1
                }
            );
        });
    }

    #[test]
    fn matches_path_only() {
        test_env::with(|env| {
            env.write_file("foo.rb", "puts 'hello'");

            let selector = "foo.rb".parse::<TestSelector>().unwrap();

            assert_eq!(
                selector,
                TestSelector::PathOnly {
                    path: "foo.rb".into()
                }
            );
        });
    }

    #[test]
    fn matches_name_only() {
        let name = "some_test_name".to_string();
        let selector = name.parse::<TestSelector>().unwrap();

        assert_eq!(selector, TestSelector::NameOnly { name });
    }
}

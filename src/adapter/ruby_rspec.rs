use std::borrow::Cow;

use crate::{
    memoize_string,
    selector_match::{exclusive_or_none, shared_or_none, SelectorMatch},
    utils::{file_exists, is_in_file},
    TestSelector,
};

use super::TestAdapter;

#[derive(Debug, PartialEq, Eq)]
pub struct RubyRspec {}

impl RubyRspec {
    /// Creates a new RubyMinitest adapter.
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self {}
    }
}

impl TestAdapter for RubyRspec {
    fn selector_matches(&self, selector: &TestSelector) -> SelectorMatch {
        match selector {
            TestSelector::PathWithLineNumber { path, .. } => {
                exclusive_or_none(path.matches("_spec.rb"))
            }
            TestSelector::PathOnly { path } => exclusive_or_none(path.matches("_spec.rb")),
            TestSelector::NameOnly { .. } => shared_or_none(is_in_file("Gemfile", "rspec")),
        }
    }

    fn collect_commands(&self, selectors: &[&TestSelector]) -> Option<Vec<Cow<'_, str>>> {
        let mut args = vec![];
        let mut commands: Vec<Cow<str>> = vec![];

        for selector in selectors {
            match selector {
                TestSelector::PathWithLineNumber { path, line } => {
                    args.push(format!("{}:{}", path.to_string_lossy(), line))
                }
                TestSelector::PathOnly { path } => args.push(path.to_string_lossy().into()),
                TestSelector::NameOnly { name } => {
                    commands.push(format!("{} --example {}", rspec_command(), name).into());
                }
            };
        }

        if !args.is_empty() {
            commands.push(format!("{} {}", rspec_command(), shell_words::join(args)).into());
        }

        if commands.is_empty() {
            None
        } else {
            Some(commands)
        }
    }
}

fn rspec_command() -> Cow<'static, str> {
    memoize_string!({
        if file_exists("bin/rspec") {
            "bin/rspec".to_string()
        } else if is_in_file("Gemfile", "rspec") {
            "bundle exec rspec".to_string()
        } else {
            "rspec".to_string()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_env;

    #[test]
    fn rspec_command_detects_bin_rspec() {
        test_env::with(|test_env| {
            test_env.write_file("bin/rspec", "");
            assert_eq!("bin/rspec", rspec_command());
        })
    }

    #[test]
    fn rspec_command_detects_gemfile() {
        test_env::with(|test_env| {
            test_env.write_file("Gemfile", "gem 'rspec'");
            assert_eq!("bundle exec rspec", rspec_command());
        })
    }

    #[test]
    fn rspec_command_detects_default() {
        test_env::with(|_| {
            assert_eq!("rspec", rspec_command());
        })
    }

    #[test]
    fn selector_matches_path_with_line_number() {
        test_env::with(|_| {
            let adapter = RubyRspec::new();
            let selector = TestSelector::PathWithLineNumber {
                path: "foo_spec.rb".into(),
                line: 1,
            };
            assert_eq!(
                SelectorMatch::Exclusive,
                adapter.selector_matches(&selector)
            );
        })
    }

    #[test]
    fn selector_matches_path_only() {
        test_env::with(|_| {
            let adapter = RubyRspec::new();
            let selector = TestSelector::PathOnly {
                path: "foo_spec.rb".into(),
            };
            assert_eq!(
                SelectorMatch::Exclusive,
                adapter.selector_matches(&selector)
            );
        })
    }

    #[test]
    fn selector_matches_name_only() {
        test_env::with(|test_env| {
            test_env.write_file("Gemfile", "gem 'rspec'");
            let adapter = RubyRspec::new();
            let selector = TestSelector::NameOnly { name: "foo".into() };
            assert_eq!(SelectorMatch::Shared, adapter.selector_matches(&selector));
        })
    }

    #[test]
    fn collect_commands_path_with_line_number() {
        test_env::with(|_| {
            let adapter = RubyRspec::new();
            let selector = TestSelector::PathWithLineNumber {
                path: "foo_spec.rb".into(),
                line: 1,
            };
            let commands = adapter.collect_commands(&[&selector]).unwrap();
            assert_eq!(commands, vec!["rspec foo_spec.rb:1"]);
        })
    }

    #[test]
    fn collect_commands_path_only() {
        test_env::with(|_| {
            let adapter = RubyRspec::new();
            let selector = TestSelector::PathOnly {
                path: "foo_spec.rb".into(),
            };
            let commands = adapter.collect_commands(&[&selector]).unwrap();
            assert_eq!(commands, vec!["rspec foo_spec.rb"]);
        })
    }

    #[test]
    fn collect_commands_name_only() {
        test_env::with(|test_env| {
            test_env.write_file("Gemfile", "gem 'rspec'");
            let adapter = RubyRspec::new();
            let selector = TestSelector::NameOnly { name: "foo".into() };
            let commands = adapter.collect_commands(&[&selector]).unwrap();
            assert_eq!(commands, vec!["bundle exec rspec --example foo"]);
        })
    }

    #[test]
    fn collect_commands_name_only_and_path_with_line_number() {
        test_env::with(|test_env| {
            test_env.write_file("Gemfile", "gem 'rspec'");
            let adapter = RubyRspec::new();
            let selector1 = TestSelector::NameOnly { name: "foo".into() };
            let selector2 = TestSelector::PathWithLineNumber {
                path: "foo_spec.rb".into(),
                line: 1,
            };
            let commands = adapter.collect_commands(&[&selector1, &selector2]).unwrap();

            assert_eq!(
                commands,
                vec![
                    "bundle exec rspec --example foo",
                    "bundle exec rspec foo_spec.rb:1"
                ]
            );
        })
    }
}

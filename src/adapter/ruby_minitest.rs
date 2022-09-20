use super::{SelectorMatch, TestAdapter};
use crate::{
    memoize_string, none_if_empty,
    selector_match::{exclusive_or_none, shared_or_none},
    test_file::TestFile,
    test_index::TestIndex,
    utils::is_in_file,
    TestSelector,
};
use std::borrow::Cow;
use TestSelector::*;

const TEST_PATTERNS: &[&str] = &["  def test_", "  it \"", "  it '", "  test \"", "  test '"];

#[derive(Debug, PartialEq, Eq)]
pub struct RubyMinitest {}

impl RubyMinitest {
    /// Creates a new RubyMinitest adapter.
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self {}
    }
}

impl TestAdapter for RubyMinitest {
    fn selector_matches(&self, selector: &TestSelector) -> SelectorMatch {
        match selector {
            PathWithLineNumber { path, .. } => {
                exclusive_or_none(is_in_file("Gemfile", "minitest") && path.matches(".rb"))
            }
            PathOnly { path } => {
                exclusive_or_none(is_in_file("Gemfile", "minitest") && path.matches(".rb"))
            }
            NameOnly { .. } => shared_or_none(is_in_file("Gemfile", "minitest")),
        }
    }

    fn collect_commands(&self, selectors: &[&TestSelector]) -> Option<Vec<Cow<'_, str>>> {
        let mut commands: Vec<Cow<str>> = vec![];
        let cmd = minitest_command();

        for selector in selectors {
            match selector {
                PathWithLineNumber { path, line } => {
                    if let Some(pattern) = find_test_pattern(path, *line) {
                        let path = path.to_string_lossy();

                        commands.push(format!("{cmd} {path} --name='{pattern}'").into());
                    }
                }
                PathOnly { path } => commands.push(format!("{cmd} {path}").into()),
                NameOnly { name } => commands.push(format!("{cmd} --name=/{}/", name).into()),
            };
        }

        none_if_empty!(commands)
    }
}

fn find_test_pattern(path: &TestFile, input_line_no: u32) -> Option<String> {
    let test_index = TestIndex::build(path, TEST_PATTERNS).ok()?;
    let closest_match = test_index.closest_to_line_number(input_line_no)?;

    Some(format_line_match(closest_match.content()))
}

fn format_line_match(line: &str) -> String {
    let line = line.trim();

    if line.starts_with("def test_") {
        line.trim_start_matches("def ").to_string()
    } else {
        let line = TEST_PATTERNS
            .iter()
            .fold(line, |acc, patt| acc.trim_start_matches(patt.trim()));

        format!("/_{}$/", line.trim_end_matches("\" do"))
    }
}

fn minitest_command() -> Cow<'static, str> {
    memoize_string!({
        if is_in_file("Gemfile", "minitest") {
            "bundle exec ruby -rminitest/autorun -Ilib:test"
        } else {
            "ruby -rminitest/autorun -Ilib:test"
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{adapter::TestAdapter, test_env};

    use super::RubyMinitest;

    #[test]
    fn generates_commands_for_path_with_line_number_using_def_syntax() {
        test_env::with(|env| {
            let test_content = r#"
              class MyTest < Minitest::Test
                def test_something
                  assert true
                end

                def test_something_else
                  assert true
                end
              end
            "#;

            env.write_file("my_test.rb", test_content);
            let selectors = [&env.selector("my_test.rb:4")];
            let adapter = RubyMinitest::new();
            let commands = adapter.collect_commands(selectors.as_slice()).unwrap();

            assert_eq!(
                commands,
                vec!["ruby -rminitest/autorun -Ilib:test my_test.rb --name='test_something_else'"]
            );
        });
    }

    #[test]
    fn generates_commands_for_path_with_line_number_using_spec_syntax() {
        test_env::with(|env| {
            let test_content = r#"
              describe "MyTest" do
                it "does something" do
                  assert true
                end

                it "does something else" do
                  assert true
                end
              end
            "#;

            env.write_file("my_test.rb", test_content);
            let selectors = [&env.selector("my_test.rb:4")];
            let adapter = RubyMinitest::new();
            let commands = adapter.collect_commands(selectors.as_slice()).unwrap();

            assert_eq!(
                commands,
                vec!["ruby -rminitest/autorun -Ilib:test my_test.rb --name='/_does something else$/'"]
            );
        });
    }
}

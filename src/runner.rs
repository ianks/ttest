use std::{
    borrow::Cow,
    error::Error,
    process::{Command, Stdio},
};

use crate::{
    adapter::{self, TestAdapter},
    selector_match::SelectorMatch,
    test_selector::TestSelector,
};

/// Filters the selectors which are supported by the given adapter, taking care
/// to remove any exclusive selectors from the list.
fn take_selectors(
    selectors: &mut Vec<TestSelector>,
    adapter: &impl TestAdapter,
) -> Vec<TestSelector> {
    let result = selectors.clone();
    let result = result.iter().enumerate();

    result
        .filter(|(i, selector)| match adapter.selector_matches(selector) {
            SelectorMatch::None => false,
            SelectorMatch::Shared => true,
            SelectorMatch::Exclusive => {
                selectors.remove(*i);
                true
            }
        })
        .map(|(_, selector)| selector.clone())
        .collect()
}

/// Collect all of the shell commands that should be run for the given selectors.
pub fn collect_commands(selectors: &[TestSelector]) -> Vec<Cow<'_, str>> {
    let mut selectors = selectors.to_vec();

    let commands = adapter::all().iter().flat_map(|adapter| {
        let matched = take_selectors(&mut selectors, adapter);
        let matched = matched.iter().collect::<Vec<&TestSelector>>();

        adapter
            .collect_commands(matched.as_slice()).unwrap_or_default()
    });

    commands.collect::<Vec<_>>()
}

/// Run all the shell commands for the given selectors.
pub fn run_all(selectors: &[TestSelector]) -> Result<(), Box<dyn Error>> {
    for command in collect_commands(selectors) {
        eprintln!("Running command: {}", command);

        let split_command = shell_words::split(&command)?;
        let mut split_command = split_command.iter();

        if let Some(executable) = split_command.next() {
            Command::new(executable)
                .args(split_command)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_env;

    #[test]
    fn collect_commands_generates_correct_commands() {
        test_env::with(|env| {
            env.write_file("Gemfile", "gem 'rspec'");
            let selectors = vec![
                TestSelector::PathWithLineNumber {
                    path: "spec/foo_spec.rb".into(),
                    line: 1,
                },
                TestSelector::NameOnly { name: "foo".into() },
            ];
            let commands = collect_commands(&selectors);

            assert_eq!(commands.len(), 2);
            assert_eq!(commands[0], "bundle exec rspec --example foo");
            assert_eq!(commands[1], "bundle exec rspec spec/foo_spec.rb:1");
        });
    }

    #[test]
    fn collect_commands_has_exclusive_matching() {
        test_env::with(|env| {
            env.write_file("Gemfile", "gem 'rspec'\ngem 'minitest'");

            let selectors = vec![TestSelector::PathWithLineNumber {
                path: "spec/foo_spec.rb".into(),
                line: 1,
            }];
            let commands = collect_commands(&selectors);

            assert_eq!(commands.len(), 1);
            assert_eq!(commands[0], "bundle exec rspec spec/foo_spec.rb:1");
        });
    }
}

mod ruby_minitest;
mod ruby_rspec;

pub use self::ruby_minitest::RubyMinitest;
pub use self::ruby_rspec::RubyRspec;

use crate::selector_match::SelectorMatch;
use crate::{define_adapters, TestSelector};
use enum_dispatch::enum_dispatch;
use std::borrow::Cow;

define_adapters! {
    "ruby_rspec" => RubyRspec,
    "ruby_minitest" => RubyMinitest
}

/// The interface a test adapter must implement to be used by the test runner.
#[enum_dispatch(Adapter)]
pub trait TestAdapter {
    /// Returns true if the adapter can handle the given selector.
    fn selector_matches(&self, selector: &TestSelector) -> SelectorMatch;
    /// Collect all of the shell commands that should be run for the given selectors.
    fn collect_commands(&self, selector: &[&TestSelector]) -> Option<Vec<Cow<'_, str>>>;
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::adapter::{Adapter, RubyMinitest, RubyRspec};

    #[test]
    fn from_str_works() {
        assert_eq!(
            Adapter::RubyRspec(RubyRspec {}),
            "ruby_rspec".try_into().unwrap(),
        );
        assert_eq!(
            Adapter::RubyMinitest(RubyMinitest {}),
            "ruby_minitest".try_into().unwrap(),
        );

        assert!(Adapter::try_from("does_not_exist").is_err());
    }
}

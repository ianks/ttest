/// The type of match that was made for a given selector.
#[derive(Debug, PartialEq, Eq)]
pub enum SelectorMatch {
    /// The selector matches, and no further adapters should be checked.
    Exclusive,
    /// The selector matches, but other adapters should be checked.
    Shared,
    /// The selector does not match.
    None,
}

/// Returns a [`SelectorMatch::Exclusive`], or [`SelectorMatch::None`],
/// depending on the bool.
pub fn exclusive_or_none(test: bool) -> SelectorMatch {
    if test {
        SelectorMatch::Exclusive
    } else {
        SelectorMatch::None
    }
}

/// Returns a [`SelectorMatch::Shared`], or [`SelectorMatch::None`], depending
/// on the bool.
pub fn shared_or_none(test: bool) -> SelectorMatch {
    if test {
        SelectorMatch::Shared
    } else {
        SelectorMatch::None
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorMatch::*;
    use super::*;

    #[test]
    fn exclusive_or_none_returns_exclusive_when_true() {
        assert_eq!(Exclusive, exclusive_or_none(true));
    }

    #[test]
    fn exclusive_or_none_returns_none_when_false() {
        assert_eq!(None, exclusive_or_none(false));
    }

    #[test]
    fn shared_or_none_returns_shared_when_true() {
        assert_eq!(Shared, shared_or_none(true));
    }

    #[test]
    fn shared_or_none_returns_none_when_false() {
        assert_eq!(None, shared_or_none(false));
    }
}

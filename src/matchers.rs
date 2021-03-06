pub mod collection;
pub mod option;
pub mod path;
pub mod result;
pub mod string;

use crate::Matcher;

/// Matches if `expected` is equal to the actual value.
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::equal};
/// expect(&"foo").to(equal("foo"));
/// expect(&"foo").not_to(equal("bar"));
/// ```
pub fn equal<T>(expected: T) -> EqualMatcher<T> {
    EqualMatcher { expected }
}

pub struct EqualMatcher<T> {
    expected: T,
}

impl<E: std::fmt::Debug, A: PartialEq<E> + std::fmt::Debug> Matcher<A> for EqualMatcher<E> {
    fn match_value(&self, actual: &A) -> bool {
        actual == &self.expected
    }

    fn failure_message(&self, actual: &A) -> String {
        format!(
            "\tExpected:\n\t\t{:?}\n\tto equal:\n\t\t{:?}",
            actual, self.expected,
        )
    }

    fn negated_failure_message(&self, actual: &A) -> String {
        format!(
            "\tExpected:\n\t\t{:?}\n\tnot to equal:\n\t\t{:?}",
            actual, self.expected
        )
    }
}

#[cfg(test)]
mod tests {
    use super::equal;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_equals_expected() {
        assert!(equal("foo").match_value(&"foo"))
    }

    #[test]
    fn should_not_match_if_actual_does_not_equal_expected() {
        assert!(!equal("foo").match_value(&"bar"))
    }

    #[test]
    fn should_allow_comparisons_between_partialeq_values() {
        assert!(equal("foo").match_value(&String::from("foo")));
    }

    #[test]
    fn failure_messages() {
        assert_eq!(
            equal("foo").failure_message(&"bar"),
            String::from("\tExpected:\n\t\t\"bar\"\n\tto equal:\n\t\t\"foo\"")
        );
        assert_eq!(
            equal("foo").negated_failure_message(&"foo"),
            String::from("\tExpected:\n\t\t\"foo\"\n\tnot to equal:\n\t\t\"foo\"")
        );
    }
}

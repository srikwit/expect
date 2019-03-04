use crate::Match;
use crate::Matcher;

pub fn be_some() -> SomeMatcher {
    SomeMatcher {}
}

pub struct SomeMatcher {}

impl<T: std::fmt::Debug> Matcher<Option<T>> for SomeMatcher {
    fn match_value(&self, actual: &Option<T>) -> Match {
        if actual.is_some() {
            Match::Matched(format!("expected {:?} not to be a Some", actual))
        } else {
            Match::NotMatched(String::from("expected None to be a Some"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::be_some;
    use super::SomeMatcher;
    use crate::expect;
    use crate::Match;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_is_some() {
        let actual = Some("foo");
        assert_eq!(
            SomeMatcher {}.match_value(&actual),
            Match::Matched(String::from("expected Some(\"foo\") not to be a Some"))
        )
    }

    #[test]
    fn should_not_match_if_actual_is_none() {
        let actual: Option<String> = None;
        assert_eq!(
            SomeMatcher {}.match_value(&actual),
            Match::NotMatched(String::from("expected None to be a Some"))
        )
    }

    #[test]
    fn be_some_should_contruct_a_some_matcher() {
        expect(&Some("thing")).to(be_some())
    }
}
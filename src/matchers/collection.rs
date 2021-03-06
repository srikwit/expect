use crate::Matcher;

use std::marker::PhantomData;

/// Matches if `actual` contains `element`.
///
/// Supports [arrays] of up to 256 elements, [`Vec`]s, [`VecDeque`]s, [`LinkedList`]s, [`HashSet`]s
/// and [`BTreeSet`]s.
///
/// [array]: https://doc.rust-lang.org/std/primitive.array.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
/// [`LinkedList`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
/// [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
/// [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
///
/// Examples
///
/// ```
/// # use expect::{expect, matchers::collection::contain};
/// expect(&[1, 2, 3]).to(contain(2));
/// expect(&vec![1, 2, 3]).not_to(contain(4));
/// ```
pub fn contain<T>(element: T) -> ContainMatcher<T> {
    ContainMatcher { element }
}

pub struct ContainMatcher<T> {
    element: T,
}

impl<T: std::cmp::PartialEq + std::fmt::Debug, V: Collection<T> + std::fmt::Debug> Matcher<V>
    for ContainMatcher<T>
{
    fn match_value(&self, collection: &V) -> bool {
        collection.contains_element(&self.element)
    }

    fn failure_message(&self, collection: &V) -> String {
        format!(
            "\tExpected:\n\t\t{:?}\n\tto contain:\n\t\t{:?}",
            collection, self.element
        )
    }

    fn negated_failure_message(&self, collection: &V) -> String {
        format!(
            "\tExpected:\n\t\t{:?}\n\tnot to contain:\n\t\t{:?}",
            collection, self.element
        )
    }
}

/// Matches if `actual` is empty.
///
/// Supports [arrays] of up to 256 elements, [`Vec`]s, [`VecDeque`]s, [`LinkedList`]s, [`HashSet`]s
/// and [`BTreeSet`]s.
///
/// [array]: https://doc.rust-lang.org/std/primitive.array.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
/// [`LinkedList`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
/// [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
/// [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
///
/// Examples
///
/// ```
/// # use expect::{expect, matchers::collection::be_empty};
/// expect(&Vec::<i32>::new()).to(be_empty());
/// ```
pub fn be_empty<T>() -> BeEmptyMatcher<T> {
    BeEmptyMatcher::<T> {
        phantom: PhantomData,
    }
}

pub struct BeEmptyMatcher<T> {
    phantom: PhantomData<T>,
}

impl<T, V: Collection<T> + std::fmt::Debug> Matcher<V> for BeEmptyMatcher<T> {
    fn match_value(&self, collection: &V) -> bool {
        collection.empty()
    }

    fn failure_message(&self, collection: &V) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tto be empty", collection)
    }

    fn negated_failure_message(&self, collection: &V) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tnot to be empty", collection)
    }
}

pub trait Collection<T> {
    fn contains_element(&self, element: &T) -> bool;
    fn empty(&self) -> bool;
}

macro_rules! array {
    ($($N:expr),+) => {
        $(
            #[doc(hidden)]
            impl<T: std::cmp::PartialEq> Collection<T> for [T; $N] {
                fn contains_element(&self, element: &T) -> bool {
                    self.contains(element)
                }

                fn empty(&self) -> bool {
                    self.is_empty()
                }
            }
        )+
    }
}

array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 256
);

impl<T: std::cmp::PartialEq> Collection<T> for std::vec::Vec<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }

    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: std::cmp::PartialEq> Collection<T> for std::collections::VecDeque<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }

    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: std::cmp::PartialEq> Collection<T> for std::collections::LinkedList<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }

    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: std::cmp::Eq + std::hash::Hash> Collection<T> for std::collections::HashSet<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }

    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: std::cmp::Ord> Collection<T> for std::collections::BTreeSet<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }

    fn empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{be_empty, contain, Collection};
    use crate::Matcher;

    #[test]
    fn contain_matcher_should_match_if_collection_contains_element() {
        assert!(contain("foo").match_value(&vec!["foo"]))
    }

    #[test]
    fn contain_matcher_should_not_match_if_collection_does_not_contain_element() {
        assert!(!contain("foo").match_value(&vec!["bar"]))
    }

    #[test]
    fn contain_matcher_failure_messages() {
        assert_eq!(
            contain("foo").failure_message(&vec!["bar"]),
            String::from("\tExpected:\n\t\t[\"bar\"]\n\tto contain:\n\t\t\"foo\"")
        );
        assert_eq!(
            contain("foo").negated_failure_message(&vec!["foo"]),
            String::from("\tExpected:\n\t\t[\"foo\"]\n\tnot to contain:\n\t\t\"foo\"")
        );
    }

    #[test]
    fn be_empty_matcher_should_match_if_collection_is_empty() {
        assert!(be_empty().match_value(&std::vec::Vec::<i32>::new()))
    }

    #[test]
    fn be_empty_matcher_should_not_match_if_collection_is_not_empty() {
        assert!(!be_empty().match_value(&vec![42]))
    }

    #[test]
    fn be_empty_matcher_failure_messages() {
        assert_eq!(
            be_empty().failure_message(&vec!["bar"]),
            String::from("\tExpected:\n\t\t[\"bar\"]\n\tto be empty")
        );
        assert_eq!(
            be_empty().negated_failure_message(&std::vec::Vec::<i32>::new()),
            String::from("\tExpected:\n\t\t[]\n\tnot to be empty")
        );
    }

    #[test]
    fn arrays_with_up_to_256_elements_are_collections() {
        assert!([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
            109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
            126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
            143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
            160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
            177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193,
            194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
            211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227,
            228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
            245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
        ]
        .contains_element(&255));
        assert!(![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
            109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
            126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
            143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
            160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
            177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193,
            194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
            211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227,
            228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
            245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
        ]
        .empty());
    }

    #[test]
    fn vecs_are_collections() {
        assert!(vec![1, 2, 3].contains_element(&2));
        assert!(std::vec::Vec::<i32>::new().empty());
    }

    #[test]
    fn vecdeques_are_collections() {
        let mut numbers = std::collections::VecDeque::new();

        assert!(numbers.empty());

        numbers.push_back(1);
        numbers.push_back(2);
        numbers.push_back(3);

        assert!(numbers.contains_element(&2))
    }

    #[test]
    fn linkedlists_are_collections() {
        let mut numbers = std::collections::LinkedList::new();

        assert!(numbers.empty());

        numbers.push_back(1);
        numbers.push_back(2);
        numbers.push_back(3);

        assert!(numbers.contains_element(&2))
    }

    #[test]
    fn hashsets_are_collections() {
        let mut numbers = std::collections::HashSet::new();

        assert!(numbers.empty());

        numbers.insert(1);
        numbers.insert(2);
        numbers.insert(3);

        assert!(numbers.contains_element(&2))
    }

    #[test]
    fn btreesets_are_collections() {
        let mut numbers = std::collections::BTreeSet::new();

        assert!(numbers.empty());

        numbers.insert(1);
        numbers.insert(2);
        numbers.insert(3);

        assert!(numbers.contains_element(&2))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list.len().cmp(&_second_list.len()) {
        std::cmp::Ordering::Equal => {
            if _first_list.eq(_second_list) {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        std::cmp::Ordering::Less => _first_list
            .len()
            .eq(&0)
            .then_some(Comparison::Sublist)
            .unwrap_or_else(|| {
                if _second_list
                    .windows(_first_list.len())
                    .any(|w| w.eq(_first_list))
                {
                    Comparison::Sublist
                } else {
                    Comparison::Unequal
                }
            }),
        std::cmp::Ordering::Greater => _second_list
            .len()
            .eq(&0)
            .then_some(Comparison::Superlist)
            .unwrap_or_else(|| {
                if _first_list
                    .windows(_second_list.len())
                    .any(|w| w.eq(_second_list))
                {
                    Comparison::Superlist
                } else {
                    Comparison::Unequal
                }
            }),
    }
}

pub fn sublist_short<T: Eq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => {
            if _first_list.windows(n).any(|v| v == _second_list) {
                Superlist
            } else {
                Unequal
            }
        }
        (m, n) if m < n => {
            if _second_list.windows(m).any(|v| v == _first_list) {
                Sublist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if _first_list.eq(_second_list) {
                Equal
            } else {
                Unequal
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_equals_empty() {
        let v: &[u32] = &[];
        assert_eq!(Comparison::Equal, sublist(v, v));
    }
    #[test]
    fn test_empty_is_a_sublist_of_anything() {
        assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
    }
    #[test]
    fn test_anything_is_a_superlist_of_empty() {
        assert_eq!(Comparison::Superlist, sublist(&['a', 's', 'd', 'f'], &[]));
    }
    #[test]
    fn test_1_is_not_2() {
        assert_eq!(Comparison::Unequal, sublist(&[1], &[2]));
    }
    #[test]
    fn test_compare_larger_equal_lists() {
        use std::iter::repeat;
        let v: Vec<char> = repeat('x').take(1000).collect();
        assert_eq!(Comparison::Equal, sublist(&v, &v));
    }
    #[test]
    fn test_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
    }
    #[test]
    fn sublist_in_middle() {
        assert_eq!(Comparison::Sublist, sublist(&[4, 3, 2], &[5, 4, 3, 2, 1]));
    }
    #[test]
    fn sublist_at_end() {
        assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
    }
    #[test]
    fn partially_matching_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 1, 2], &[1, 1, 1, 2]));
    }
    #[test]
    fn sublist_early_in_huge_list() {
        let huge: Vec<u32> = (1..1_000_000).collect();
        assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &huge));
    }
    #[test]
    fn huge_sublist_not_in_huge_list() {
        let v1: Vec<u64> = (10..1_000_001).collect();
        let v2: Vec<u64> = (1..1_000_000).collect();
        assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
    }
    #[test]
    fn superlist_at_start() {
        assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[1, 2, 3]));
    }
    #[test]
    fn superlist_in_middle() {
        assert_eq!(Comparison::Superlist, sublist(&[5, 4, 3, 2, 1], &[4, 3, 2]));
    }
    #[test]
    fn superlist_at_end() {
        assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[3, 4, 5]));
    }
    #[test]
    fn second_list_missing_element_from_first_list() {
        assert_eq!(Comparison::Unequal, sublist(&[1, 2, 3], &[1, 3]));
    }
    #[test]
    fn superlist_early_in_huge_list() {
        let huge: Vec<u32> = (1..1_000_000).collect();
        assert_eq!(Comparison::Superlist, sublist(&huge, &[3, 4, 5]));
    }
    #[test]
    fn recurring_values_sublist() {
        assert_eq!(
            Comparison::Sublist,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1])
        );
    }
    #[test]
    fn recurring_values_unequal() {
        assert_eq!(
            Comparison::Unequal,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1])
        );
    }
}

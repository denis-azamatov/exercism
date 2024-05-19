pub fn is_armstrong_number(num: u32) -> bool {
    let digits = to_digits(num);
    let len = digits.len() as u32;

    match digits.into_iter().try_fold(0_u32, |acc, x| acc.checked_add(x.pow(len))) {
        None => false,
        Some(res) => res == num
    }
}

fn to_digits(mut v: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::with_capacity(10);

    while v > 0 {
        let n = v % 10;
        v /= 10;
        digits.push(n);
    }
    digits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }

    #[test]
    fn single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }

    #[test]
    fn there_are_no_2_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }

    #[test]
    fn three_digit_armstrong_number() {
        assert!(is_armstrong_number(153))
    }

    #[test]
    fn three_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }

    #[test]
    fn four_digit_armstrong_number() {
        assert!(is_armstrong_number(9474))
    }

    #[test]
    fn four_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9475))
    }

    #[test]
    fn seven_digit_armstrong_number() {
        assert!(is_armstrong_number(9_926_315))
    }

    #[test]
    fn seven_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9_926_316))
    }

    #[test]
    fn nine_digit_armstrong_number() {
        assert!(is_armstrong_number(912_985_153));
    }

    #[test]
    fn nine_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(999_999_999));
    }

    #[test]
    fn ten_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(3_999_999_999));
    }

    // The following number has an Armstrong sum equal to 2^32 plus itself,
    // and therefore will be detected as an Armstrong number if you are
    // incorrectly using wrapping arithmetic.
    #[test]
    fn properly_handles_overflow() {
        assert!(!is_armstrong_number(4_106_098_957));
    }
}

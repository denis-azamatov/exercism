pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) {
        panic!("Square must be between 1 and 64");
    }

    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    18_446_744_073_709_551_615
}

#[cfg(test)]
mod test {
    use crate::grains;

    fn process_square_case(input: u32, expected: u64) {
        assert_eq!(grains::square(input), expected);
    }


    #[test]
    fn one() {
        process_square_case(1, 1);
    }


    #[test]
    fn two() {
        process_square_case(2, 2);
    }


    #[test]
    fn three() {
        process_square_case(3, 4);
    }


    #[test]
    fn four() {
        process_square_case(4, 8);
    }


    #[test]
    fn sixteen() {
        process_square_case(16, 32_768);
    }


    #[test]
    fn thirty_two() {
        process_square_case(32, 2_147_483_648);
    }


    #[test]
    fn sixty_four() {
        process_square_case(64, 9_223_372_036_854_775_808);
    }


    #[test]
    #[should_panic(expected = "Square must be between 1 and 64")]
    fn square_0_raises_an_exception() {
        grains::square(0);
    }


    #[test]
    #[should_panic(expected = "Square must be between 1 and 64")]
    fn square_greater_than_64_raises_an_exception() {
        grains::square(65);
    }


    #[test]
    fn returns_the_total_number_of_grains_on_the_board() {
        assert_eq!(grains::total(), 18_446_744_073_709_551_615);
    }
}
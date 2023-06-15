use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn reverse_graphemes(input: &str) -> String {
    input.graphemes(true).rev().collect()
}

#[cfg(test)]
mod test {
    use super::reverse;
    use super::reverse_graphemes;

    fn process_reverse_case(input: &str, expected: &str) {
        assert_eq!(&reverse(input), expected);
    }

    fn process_reverse_graphemes_case(input: &str, expected: &str) {
        assert_eq!(&reverse_graphemes(input), expected)
    }

    #[test]
    fn test_an_empty_string() {
        process_reverse_case("", "");
        process_reverse_graphemes_case("", "");
    }

    #[test]
    fn test_a_word() {
        process_reverse_case("robot", "tobor");
        process_reverse_graphemes_case("robot", "tobor");
    }

    #[test]
    fn test_a_capitalized_word() {
        process_reverse_case("Ramen", "nemaR");
        process_reverse_graphemes_case("Ramen", "nemaR");
    }

    #[test]
    fn test_a_sentence_with_punctuation() {
        process_reverse_case("I'm hungry!", "!yrgnuh m'I");
        process_reverse_graphemes_case("I'm hungry!", "!yrgnuh m'I");
    }

    #[test]
    fn test_a_palindrome() {
        process_reverse_case("racecar", "racecar");
        process_reverse_graphemes_case("racecar", "racecar");
    }

    #[test]
    fn test_an_even_sized_word() {
        process_reverse_case("drawer", "reward");
        process_reverse_graphemes_case("drawer", "reward");
    }

    #[test]
    fn test_wide_characters() {
        process_reverse_case("子猫", "猫子");
        process_reverse_graphemes_case("子猫", "猫子");
    }

    #[test]
    #[cfg(feature = "grapheme")]
    fn test_grapheme_clusters() {
        process_reverse_graphemes_case("uüu", "uüu");
    }
}

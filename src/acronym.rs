pub fn abbreviate(phrase: &str) -> String {
    let mut prev: Option<char> = None;
    let mut result = String::new();

    for char in phrase.chars() {
        if !char.is_alphabetic() {
            prev = Some(char);
            continue;
        }

        let is_prev_some = prev.is_some();
        let is_prev_alphabetic = is_prev_some && prev.unwrap().is_alphabetic();
        let is_prev_lowercase = is_prev_alphabetic && prev.unwrap().is_lowercase();
        let is_char_uppercase = char.is_uppercase();

        match (is_prev_some, is_prev_alphabetic, is_prev_lowercase, is_char_uppercase) {
            (true, true, true, true) => result.push(char), // CamelCase
            (true, false, _, _) => if prev.unwrap() != '\'' { result.push_str(&char.to_uppercase().to_string()) }, // Other non-alphabetic characters except apostrophe
            (false, _, _, _) => result.push_str(&char.to_uppercase().to_string()), // First letter
            _ => {}
        }

        prev = Some(char);
    }

    result
}

#[cfg(test)]
mod test {
    use crate::acronym;

    #[test]
    fn basic() {
        let input = "Portable Network Graphics";

        let output = acronym::abbreviate(input);

        let expected = "PNG";

        assert_eq!(output, expected);
    }

    #[test]
    fn lowercase_words() {
        let input = "Ruby on Rails";

        let output = acronym::abbreviate(input);

        let expected = "ROR";

        assert_eq!(output, expected);
    }

    #[test]
    fn punctuation() {
        let input = "First In, First Out";

        let output = acronym::abbreviate(input);

        let expected = "FIFO";

        assert_eq!(output, expected);
    }

    #[test]
    fn all_caps_word() {
        let input = "GNU Image Manipulation Program";

        let output = acronym::abbreviate(input);

        let expected = "GIMP";

        assert_eq!(output, expected);
    }

    #[test]
    fn punctuation_without_whitespace() {
        let input = "Complementary metal-oxide semiconductor";

        let output = acronym::abbreviate(input);

        let expected = "CMOS";

        assert_eq!(output, expected);
    }

    #[test]
    fn very_long_abbreviation() {
        let input = "Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me";

        let output = acronym::abbreviate(input);

        let expected = "ROTFLSHTMDCOALM";

        assert_eq!(output, expected);
    }

    #[test]
    fn consecutive_delimiters() {
        let input = "Something - I made up from thin air";

        let output = acronym::abbreviate(input);

        let expected = "SIMUFTA";

        assert_eq!(output, expected);
    }

    #[test]
    fn apostrophes() {
        let input = "Halley's Comet";

        let output = acronym::abbreviate(input);

        let expected = "HC";

        assert_eq!(output, expected);
    }

    #[test]
    fn underscore_emphasis() {
        let input = "The Road _Not_ Taken";

        let output = acronym::abbreviate(input);

        let expected = "TRNT";

        assert_eq!(output, expected);
    }

    #[test]
    fn camelcase() {
        assert_eq!(acronym::abbreviate("HyperText Markup Language"), "HTML");
    }
}

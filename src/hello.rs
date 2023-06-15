pub fn hello() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod test {
    use super::hello;

    #[test]
    fn check_correct_output() {
        assert_eq!("Hello, World!", hello());
    }
}

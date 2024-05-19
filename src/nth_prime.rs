pub fn nth(n: u32) -> u32 {
    (2..).  filter(|x| is_prime(*x)).nth(n as usize).unwrap()
}

fn is_prime(n: u32) -> bool {
    let number = (n as f64).sqrt() as u32;

    !(2..=number).any(|k| n % k == 0)
}

#[cfg(test)]
mod test {
    use crate::nth_prime as np;

    #[test]
    fn first_prime() {
        assert_eq!(np::nth(0), 2);
    }

    #[test]
    fn second_prime() {
        assert_eq!(np::nth(1), 3);
    }

    #[test]
    fn sixth_prime() {
        assert_eq!(np::nth(5), 13);
    }

    #[test]
    fn big_prime() {
        assert_eq!(np::nth(10_000), 104_743);
    }
}

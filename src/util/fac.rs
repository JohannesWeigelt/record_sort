pub fn fac(n: u64) -> u64 {
    if n == 0 { 1 } else { n * fac(n - 1) }
}

#[cfg(test)]
mod test {
    use crate::util::fac::fac;

    #[test]
    fn success() {
        let expected = 24;

        assert_eq!(expected, fac(4))
    }
}
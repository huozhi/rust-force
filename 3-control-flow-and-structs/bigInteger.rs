#[derive(Debug)]
struct BigInteger {
    number: Vec<char>,
}

struct BigDecimal {
    suffix: BigInteger,
    prefix: BigInteger,
}

impl BigInteger {
    fn add(&self, other: &BigInteger) -> BigInteger {}

    fn multiply_with_int(&self, other: i64) -> BigInteger {}

    fn multiply(&self, other: &BigInteger) -> BigInteger {}
}

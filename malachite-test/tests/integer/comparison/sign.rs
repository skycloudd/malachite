use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, GenerationMode};
use malachite_test::integer::comparison::sign::{num_sign, select_inputs};
use num;
use rugint;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_sign() {
    let test = |s, out| {
        assert_eq!(native::Integer::from_str(s).unwrap().sign(), out);
        assert_eq!(gmp::Integer::from_str(s).unwrap().sign(), out);
        assert_eq!(num_sign(&num::BigInt::from_str(s).unwrap()), out);
        assert_eq!(rugint::Integer::from_str(s).unwrap().sign(), out);
    };
    test("0", Ordering::Equal);
    test("123", Ordering::Greater);
    test("-123", Ordering::Less);
    test("1000000000000", Ordering::Greater);
    test("-1000000000000", Ordering::Less);
}

#[test]
fn sign_properties() {
    // n.sign() is equivalent for malachite-gmp and malachite-native.
    // n.sign() == n.partial_cmp(&0)
    // (-n).sign() == n.sign().reverse()
    let one_integer = |gmp_n: gmp::Integer| {
        let n = gmp_integer_to_native(&gmp_n);
        let sign = n.sign();
        assert_eq!(gmp_n.sign(), sign);
        assert_eq!(n.partial_cmp(&0), Some(sign));
        assert_eq!((-n).sign(), sign.reverse());
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}

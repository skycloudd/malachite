use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::gmp_integer_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::str::FromStr;

#[test]
fn test_is_even() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().is_even(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().is_even(), out);
    };
    test("0", true);
    test("1", false);
    test("2", true);
    test("3", false);
    test("123", false);
    test("1000000000000", true);
    test("1000000000001", false);
    test("-1", false);
    test("-2", true);
    test("-3", false);
    test("-123", false);
    test("-1000000000000", true);
    test("-1000000000001", false);
}

#[test]
fn test_is_odd() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().is_odd(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().is_odd(), out);
    };
    test("0", false);
    test("1", true);
    test("2", false);
    test("3", true);
    test("123", true);
    test("1000000000000", false);
    test("1000000000001", true);
    test("-1", true);
    test("-2", false);
    test("-3", true);
    test("-123", true);
    test("-1000000000000", false);
    test("-1000000000001", true);
}

#[test]
fn is_even_properties() {
    // x.is_even() is equivalent for malachite-gmp and malachite-native.
    // x.is_even() == !x.is_odd()
    // x.is_even() == (-x).is_even()
    // x.is_even() == (!x).is_even()
    // x.is_even == (x + 1).is_odd()
    // x.is_even == (x - 1).is_odd()
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let is_even = x.is_even();
        assert_eq!(gmp_x.is_even(), is_even);
        assert_eq!(!x.is_odd(), is_even);
        assert_eq!((&x + 1u32).is_odd(), is_even);
        assert_eq!((x - 1u32).is_odd(), is_even);
    };

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }
}

#[test]
fn is_odd_properties() {
    // x.is_odd() is equivalent for malachite-gmp and malachite-native.
    // x.is_odd() == !x.is_even()
    // x.is_odd() == (-x).is_odd()
    // x.is_odd() == (!x).is_odd()
    // x.is_odd == (x + 1).is_even()
    // x.is_odd == (x - 1).is_even()
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let is_odd = x.is_odd();
        assert_eq!(gmp_x.is_odd(), is_odd);
        assert_eq!(!x.is_even(), is_odd);
        assert_eq!((&x + 1u32).is_even(), is_odd);
        assert_eq!((x - 1u32).is_even(), is_odd);
    };

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }
}

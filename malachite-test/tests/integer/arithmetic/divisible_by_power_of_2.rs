use common::LARGE_LIMIT;
use malachite_base::traits::Zero;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use malachite_test::common::{gmp_integer_to_native, GenerationMode};
use malachite_test::integer::arithmetic::divisible_by_power_of_2::select_inputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use std::str::FromStr;

#[test]
fn test_divisible_by_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            native::Integer::from_str(n)
                .unwrap()
                .divisible_by_power_of_2(pow),
            out
        );
        assert_eq!(
            gmp::Integer::from_str(n)
                .unwrap()
                .divisible_by_power_of_2(pow,),
            out
        );
    };
    test("0", 0, true);
    test("0", 10, true);
    test("0", 100, true);
    test("123", 0, true);
    test("123", 1, false);
    test("-123", 0, true);
    test("-123", 1, false);
    test("1000000000000", 0, true);
    test("1000000000000", 12, true);
    test("1000000000000", 13, false);
    test("-1000000000000", 0, true);
    test("-1000000000000", 12, true);
    test("-1000000000000", 13, false);
    test("4294967295", 0, true);
    test("4294967295", 1, false);
    test("-4294967295", 0, true);
    test("-4294967295", 1, false);
    test("4294967296", 0, true);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("-4294967296", 0, true);
    test("-4294967296", 32, true);
    test("-4294967296", 33, false);
    test("18446744073709551615", 0, true);
    test("18446744073709551615", 1, false);
    test("-18446744073709551615", 0, true);
    test("-18446744073709551615", 1, false);
    test("18446744073709551616", 0, true);
    test("18446744073709551616", 64, true);
    test("18446744073709551616", 65, false);
    test("-18446744073709551616", 0, true);
    test("-18446744073709551616", 64, true);
    test("-18446744073709551616", 65, false);
}

#[test]
fn divisible_by_power_of_2_properties() {
    // x.divisible_by_power_of_2(pow) is equivalent for malachite-gmp and malachite-native.
    // if x != 0, x.divisible_by_power_of_2(pow) == (x.trailing_zeros().unwrap() >= pow)
    // (-x).divisible_by_power_of_2(pow) == x.divisible_by_power_of_2()
    // (x << pow).divisible_by_power_of_2(pow)
    // x.divisible_by_power_of_2(pow) == (x >> pow << pow == x)
    let integer_and_u32 = |gmp_x: gmp::Integer, pow: u32| {
        let x = gmp_integer_to_native(&gmp_x);
        let divisible = x.divisible_by_power_of_2(pow);
        assert_eq!(gmp_x.divisible_by_power_of_2(pow), divisible);
        if x != 0 {
            assert_eq!(x.trailing_zeros().unwrap() >= pow.into(), divisible);
        }
        assert_eq!((-&x).divisible_by_power_of_2(pow), divisible);
        assert!((&x << pow as u32).divisible_by_power_of_2(pow));
        assert_eq!(&x >> pow << pow == x, divisible);
    };

    // x.divisible_by_power_of_2(0)
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        assert!(x.divisible_by_power_of_2(0));
    };

    // 0.divisible_by_power_of_2(pow)
    let one_u32 = |pow: u32| {
        assert!(native::Integer::ZERO.divisible_by_power_of_2(pow));
    };

    for (x, pow) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(x, pow);
    }

    for (x, pow) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(x, pow);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in exhaustive_u().take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_u32(n);
    }
}

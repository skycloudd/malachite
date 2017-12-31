use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_rugint_integer,
                             rugint_integer_to_native_natural, GenerationMode};
use malachite_test::natural::logic::assign_bit::select_inputs;
use rugint;
use std::str::FromStr;

#[test]
fn test_assign_bit() {
    let test = |u, index, bit, out| {
        let mut n = native::Natural::from_str(u).unwrap();
        n.assign_bit(index, bit);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n.assign_bit(index, bit);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n.set_bit(index as u32, bit);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, true, "1024");
    test("100", 0, true, "101");
    test("1000000000000", 10, true, "1000000001024");
    test(
        "1000000000000",
        100,
        true,
        "1267650600228229402496703205376",
    );
    test("5", 100, true, "1267650600228229401496703205381");
    test("0", 10, false, "0");
    test("0", 100, false, "0");
    test("1024", 10, false, "0");
    test("101", 0, false, "100");
    test("1000000001024", 10, false, "1000000000000");
    test("1000000001024", 100, false, "1000000001024");
    test(
        "1267650600228229402496703205376",
        100,
        false,
        "1000000000000",
    );
    test("1267650600228229401496703205381", 100, false, "5");
}

#[test]
fn assign_bit_properties() {
    // n.assign_bit(index) is equivalent for malachite-gmp and malachite-native.
    let natural_u64_and_bool = |mut gmp_n: gmp::Natural, index: u64, bit: bool| {
        let mut n = gmp_natural_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.assign_bit(index, bit);
        assert!(gmp_n.is_valid());

        n.assign_bit(index, bit);
        assert!(n.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_n), n);

        let mut rugint_n = native_natural_to_rugint_integer(&old_n);
        rugint_n.set_bit(index as u32, bit);
        assert_eq!(rugint_integer_to_native_natural(&rugint_n), n);
    };

    for (n, index, bit) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_u64_and_bool(n, index, bit);
    }

    for (n, index, bit) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_u64_and_bool(n, index, bit);
    }
}

use common::LARGE_LIMIT;
use malachite_base::traits::{One, Zero};
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_num_bigint,
                             native_integer_to_rugint, num_bigint_to_native_integer,
                             rugint_integer_to_native, GenerationMode};
use malachite_test::integer::arithmetic::mul::select_inputs;
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_triples_from_single,
                                     random_pairs, random_triples_from_single};
use std::str::FromStr;

#[test]
fn test_mul() {
    #[allow(unknown_lints, cyclomatic_complexity)]
    let test = |u, v, out| {
        let mut n = native::Integer::from_str(u).unwrap();
        n *= native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = native::Integer::from_str(u).unwrap();
        n *= &native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Integer::from_str(u).unwrap();
        n *= gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Integer::from_str(u).unwrap();
        n *= &gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Integer::from_str(u).unwrap() * native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &native::Integer::from_str(u).unwrap() * native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = native::Integer::from_str(u).unwrap() * &native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &native::Integer::from_str(u).unwrap() * &native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Integer::from_str(u).unwrap() * gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Integer::from_str(u).unwrap() * &gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Integer::from_str(u).unwrap() * gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &gmp::Integer::from_str(u).unwrap() * &gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num::BigInt::from_str(u).unwrap() * num::BigInt::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() * rugint::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("0", "-123", "0");
    test("123", "0", "0");
    test("-123", "0", "0");
    test("1", "123", "123");
    test("1", "-123", "-123");
    test("-1", "123", "-123");
    test("-1", "-123", "123");
    test("123", "1", "123");
    test("123", "-1", "-123");
    test("-123", "1", "-123");
    test("-123", "-1", "123");
    test("123", "456", "56088");
    test("123", "-456", "-56088");
    test("-123", "456", "-56088");
    test("-123", "-456", "56088");
    test("0", "1000000000000", "0");
    test("0", "-1000000000000", "0");
    test("1000000000000", "0", "0");
    test("-1000000000000", "0", "0");
    test("1", "1000000000000", "1000000000000");
    test("1", "-1000000000000", "-1000000000000");
    test("-1", "1000000000000", "-1000000000000");
    test("-1", "-1000000000000", "1000000000000");
    test("1000000000000", "1", "1000000000000");
    test("1000000000000", "-1", "-1000000000000");
    test("-1000000000000", "1", "-1000000000000");
    test("-1000000000000", "-1", "1000000000000");
    test("1000000000000", "123", "123000000000000");
    test("1000000000000", "-123", "-123000000000000");
    test("-1000000000000", "123", "-123000000000000");
    test("-1000000000000", "-123", "123000000000000");
    test("123", "1000000000000", "123000000000000");
    test("123", "-1000000000000", "-123000000000000");
    test("-123", "1000000000000", "-123000000000000");
    test("-123", "-1000000000000", "123000000000000");
    test("123456789000", "987654321000", "121932631112635269000000");
    test("123456789000", "-987654321000", "-121932631112635269000000");
    test("-123456789000", "987654321000", "-121932631112635269000000");
    test("-123456789000", "-987654321000", "121932631112635269000000");
    test("4294967295", "2", "8589934590");
    test("4294967295", "-2", "-8589934590");
    test("-4294967295", "2", "-8589934590");
    test("-4294967295", "-2", "8589934590");
    test("4294967295", "4294967295", "18446744065119617025");
    test("4294967295", "-4294967295", "-18446744065119617025");
    test("-4294967295", "4294967295", "-18446744065119617025");
    test("-4294967295", "-4294967295", "18446744065119617025");
}

#[test]
fn mul_properties() {
    // x * y is valid.
    // x * &y is valid.
    // &x * y is valid.
    // &x * &y is valid.
    // x * y is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // x *= y, x *= &y, x * y, x * &y, &x * y, and &x * &y give the same result.
    // x * y == y * x
    //TODO x * y / y == x and x * y / x == y
    // (-x) * y == -(x * y)
    // x * (-y) == -(x * y)
    #[allow(unknown_lints, cyclomatic_complexity)]
    let two_integers = |gmp_x: gmp::Integer, gmp_y: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let raw_gmp_product = gmp_x.clone() * gmp_y.clone();
        assert!(raw_gmp_product.is_valid());
        let gmp_product = gmp_integer_to_native(&raw_gmp_product);
        let num_product = num_bigint_to_native_integer(
            &(native_integer_to_num_bigint(&x) * native_integer_to_num_bigint(&y)),
        );
        let rugint_product = rugint_integer_to_native(
            &(native_integer_to_rugint(&x) * native_integer_to_rugint(&y)),
        );

        let product_val_val = gmp_x.clone() * gmp_y.clone();
        let product_val_ref = gmp_x.clone() * &gmp_y;
        let product_ref_val = &gmp_x * gmp_y.clone();
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert_eq!(product_val_val, raw_gmp_product);
        assert_eq!(product_val_ref, raw_gmp_product);
        assert_eq!(product_ref_val, raw_gmp_product);

        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * &y;
        let product_ref_val = &x * y.clone();
        let product = &x * &y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x *= &y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let mut mut_x = gmp_x.clone();
        mut_x *= gmp_y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, raw_gmp_product);
        let mut mut_x = gmp_x.clone();
        mut_x *= &gmp_y;
        assert_eq!(mut_x, raw_gmp_product);
        assert!(mut_x.is_valid());

        let mut mut_x = native_integer_to_rugint(&x);
        mut_x *= native_integer_to_rugint(&y);
        assert_eq!(rugint_integer_to_native(&mut_x), product);

        let reverse_product = &y * &x;
        //TODO let inv_1 = (&product / &x).unwrap();
        //TODO let inv_2 = (&product / &y).unwrap();
        assert_eq!(gmp_product, product);
        assert_eq!(num_product, product);
        assert_eq!(rugint_product, product);
        assert_eq!(reverse_product, product);
        //TODO assert_eq!(inv_1, y);
        //TODO assert_eq!(inv_2, x);

        assert_eq!(-&x * &y, -&product);
        assert_eq!(x * -y, -product);
    };

    // x * (y: u32) == x * from(y)
    // (y: u32) * x == x * from(y)
    let integer_and_u32 = |gmp_x: gmp::Integer, y: u32| {
        let x = gmp_integer_to_native(&gmp_x);
        let primitive_product_1 = &x * y;
        let primitive_product_2 = y * &x;
        let product = x * native::Integer::from(y);
        assert_eq!(primitive_product_1, product);
        assert_eq!(primitive_product_2, product);
    };

    // x * 0 == 0
    // 0 * x == 0
    // x * 1 == x
    // 1 * x == x
    //TODO x * x == x ^ 2
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let x_old = x.clone();
        assert_eq!(&x * native::Integer::ZERO, 0);
        assert_eq!(native::Integer::ZERO * 0, 0);
        let id_1 = &x * native::Integer::ONE;
        let id_2 = native::Integer::ONE * &x;
        //TODO let square = &x * &x;
        assert_eq!(id_1, x_old);
        assert_eq!(id_2, x_old);
        //TODO assert_eq!(square, x_old.pow(2));
    };

    // (x * y) * z == x * (y * z)
    // x * (y + z) == x * y + x * z
    // (x + y) * z == x * z + y * z
    let three_integers = |gmp_x: gmp::Integer, gmp_y: gmp::Integer, gmp_z: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let y = gmp_integer_to_native(&gmp_y);
        let z = gmp_integer_to_native(&gmp_z);
        assert_eq!((&x * &y) * &z, &x * (&y * &z));
        assert_eq!(&x * (&y + &z), &x * &y + &x * &z);
        assert_eq!((&x + &y) * &z, x * &z + y * z);
    };

    for (x, y) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        integer_and_u32(x, y);
    }

    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        integer_and_u32(x, y);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for (x, y, z) in exhaustive_triples_from_single(exhaustive_integers()).take(LARGE_LIMIT) {
        three_integers(x, y, z);
    }

    for (x, y, z) in
        random_triples_from_single(random_integers(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT)
    {
        three_integers(x, y, z);
    }
}

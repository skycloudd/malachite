use common::LARGE_LIMIT;
use malachite_base::round::RoundingMode;
use malachite_base::traits::{ShrRound, ShrRoundAssign, Zero};
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_rugint_integer, rugint_integer_to_natural, GenerationMode};
use malachite_test::natural::arithmetic::shr_i32::{select_inputs_1, select_inputs_2};
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::i32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_shr_i32() {
    let test = |u, v: i32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n >>= v;
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rugint::Integer::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);

        let n = &Natural::from_str(u).unwrap() >> v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, "0");
    test("0", 10, "0");
    test("123", 0, "123");
    test("245", 1, "122");
    test("246", 1, "123");
    test("247", 1, "123");
    test("491", 2, "122");
    test("492", 2, "123");
    test("493", 2, "123");
    test("4127195135", 25, "122");
    test("4127195136", 25, "123");
    test("4127195137", 25, "123");
    test("8254390271", 26, "122");
    test("8254390272", 26, "123");
    test("8254390273", 26, "123");
    test("155921023828072216384094494261247", 100, "122");
    test("155921023828072216384094494261248", 100, "123");
    test("155921023828072216384094494261249", 100, "123");
    test("4294967295", 1, "2147483647");
    test("4294967296", 1, "2147483648");
    test("4294967297", 1, "2147483648");
    test("1000000000000", 0, "1000000000000");
    test("7999999999999", 3, "999999999999");
    test("8000000000000", 3, "1000000000000");
    test("8000000000001", 3, "1000000000000");
    test("16777216000000000000", 24, "1000000000000");
    test("33554432000000000000", 25, "1000000000000");
    test("2147483648000000000000", 31, "1000000000000");
    test("4294967296000000000000", 32, "1000000000000");
    test("8589934592000000000000", 33, "1000000000000");
    test(
        "1267650600228229401496703205376000000000000",
        100,
        "1000000000000",
    );
    test("1000000000000", 10, "976562500");
    test("980657949", 72, "0");
    test("4294967295", 31, "1");
    test("4294967295", 32, "0");
    test("4294967296", 32, "1");
    test("4294967296", 33, "0");

    test("0", 0, "0");
    test("0", -10, "0");
    test("123", 0, "123");
    test("123", -1, "246");
    test("123", -2, "492");
    test("123", -25, "4127195136");
    test("123", -26, "8254390272");
    test("123", -100, "155921023828072216384094494261248");
    test("2147483648", -1, "4294967296");
    test("1000000000000", 0, "1000000000000");
    test("1000000000000", -3, "8000000000000");
    test("1000000000000", -24, "16777216000000000000");
    test("1000000000000", -25, "33554432000000000000");
    test("1000000000000", -31, "2147483648000000000000");
    test("1000000000000", -32, "4294967296000000000000");
    test("1000000000000", -33, "8589934592000000000000");
    test(
        "1000000000000",
        -100,
        "1267650600228229401496703205376000000000000",
    );
}

#[test]
fn shr_i32_properties() {
    // n >>= i is equivalent for malachite and rugint.
    // n >> i is equivalent for malachite and rugint.
    // n >>= i; n is valid.
    // n >> i is valid.
    // &n >> i is valid.
    // n >>= i, n >> i, and &n >> i give the same result.
    // n >> i == n.shr_round(u, Floor)
    let natural_and_i32 = |mut n: Natural, i: i32| {
        let old_n = n.clone();
        n >>= i;
        assert!(n.is_valid());

        let mut rugint_n = natural_to_rugint_integer(&old_n);
        rugint_n >>= i;
        assert_eq!(rugint_integer_to_natural(&rugint_n), n);

        let n2 = old_n.clone();
        let result = &n2 >> i;
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2 >> i;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let rugint_n2 = natural_to_rugint_integer(&old_n);
        assert_eq!(rugint_integer_to_natural(&(rugint_n2 >> i)), n);

        assert_eq!(&old_n >> i, (&old_n).shr_round(i, RoundingMode::Floor));
    };

    // n >> 0 == n
    #[allow(unknown_lints, identity_op)]
    let one_natural = |n: Natural| {
        assert_eq!(&n >> 0i32, n);
    };

    // 0 >> i == 0
    let one_i32 = |i: i32| {
        assert_eq!(Natural::ZERO >> i, 0);
    };

    for (n, i) in select_inputs_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_i32(n, i);
    }

    for (n, i) in select_inputs_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_i32(n, i);
    }

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in exhaustive_i().take(LARGE_LIMIT) {
        one_i32(n);
    }

    for n in i32s_geometric(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_i32(n);
    }
}

#[test]
fn test_shr_round_i32() {
    let test = |i, j: i32, rm: RoundingMode, out| {
        let mut n = Natural::from_str(i).unwrap();
        n.shr_round_assign(j, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(i).unwrap().shr_round(j, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(i).unwrap().shr_round(j, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, RoundingMode::Down, "0");
    test("0", 0, RoundingMode::Up, "0");
    test("0", 0, RoundingMode::Floor, "0");
    test("0", 0, RoundingMode::Ceiling, "0");
    test("0", 0, RoundingMode::Nearest, "0");
    test("0", 0, RoundingMode::Exact, "0");

    test("0", 10, RoundingMode::Down, "0");
    test("0", 10, RoundingMode::Up, "0");
    test("0", 10, RoundingMode::Floor, "0");
    test("0", 10, RoundingMode::Ceiling, "0");
    test("0", 10, RoundingMode::Nearest, "0");
    test("0", 10, RoundingMode::Exact, "0");

    test("123", 0, RoundingMode::Down, "123");
    test("123", 0, RoundingMode::Up, "123");
    test("123", 0, RoundingMode::Floor, "123");
    test("123", 0, RoundingMode::Ceiling, "123");
    test("123", 0, RoundingMode::Nearest, "123");
    test("123", 0, RoundingMode::Exact, "123");

    test("245", 1, RoundingMode::Down, "122");
    test("245", 1, RoundingMode::Up, "123");
    test("245", 1, RoundingMode::Floor, "122");
    test("245", 1, RoundingMode::Ceiling, "123");
    test("245", 1, RoundingMode::Nearest, "122");

    test("246", 1, RoundingMode::Down, "123");
    test("246", 1, RoundingMode::Up, "123");
    test("246", 1, RoundingMode::Floor, "123");
    test("246", 1, RoundingMode::Ceiling, "123");
    test("246", 1, RoundingMode::Nearest, "123");
    test("246", 1, RoundingMode::Exact, "123");

    test("247", 1, RoundingMode::Down, "123");
    test("247", 1, RoundingMode::Up, "124");
    test("247", 1, RoundingMode::Floor, "123");
    test("247", 1, RoundingMode::Ceiling, "124");
    test("247", 1, RoundingMode::Nearest, "124");

    test("491", 2, RoundingMode::Down, "122");
    test("491", 2, RoundingMode::Up, "123");
    test("491", 2, RoundingMode::Floor, "122");
    test("491", 2, RoundingMode::Ceiling, "123");
    test("491", 2, RoundingMode::Nearest, "123");

    test("492", 2, RoundingMode::Down, "123");
    test("492", 2, RoundingMode::Up, "123");
    test("492", 2, RoundingMode::Floor, "123");
    test("492", 2, RoundingMode::Ceiling, "123");
    test("492", 2, RoundingMode::Nearest, "123");
    test("492", 2, RoundingMode::Exact, "123");

    test("493", 2, RoundingMode::Down, "123");
    test("493", 2, RoundingMode::Up, "124");
    test("493", 2, RoundingMode::Floor, "123");
    test("493", 2, RoundingMode::Ceiling, "124");
    test("493", 2, RoundingMode::Nearest, "123");

    test("4127195135", 25, RoundingMode::Down, "122");
    test("4127195135", 25, RoundingMode::Up, "123");
    test("4127195135", 25, RoundingMode::Floor, "122");
    test("4127195135", 25, RoundingMode::Ceiling, "123");
    test("4127195135", 25, RoundingMode::Nearest, "123");

    test("4127195136", 25, RoundingMode::Down, "123");
    test("4127195136", 25, RoundingMode::Up, "123");
    test("4127195136", 25, RoundingMode::Floor, "123");
    test("4127195136", 25, RoundingMode::Ceiling, "123");
    test("4127195136", 25, RoundingMode::Nearest, "123");
    test("4127195136", 25, RoundingMode::Exact, "123");

    test("4127195137", 25, RoundingMode::Down, "123");
    test("4127195137", 25, RoundingMode::Up, "124");
    test("4127195137", 25, RoundingMode::Floor, "123");
    test("4127195137", 25, RoundingMode::Ceiling, "124");
    test("4127195137", 25, RoundingMode::Nearest, "123");

    test("8254390271", 26, RoundingMode::Down, "122");
    test("8254390271", 26, RoundingMode::Up, "123");
    test("8254390271", 26, RoundingMode::Floor, "122");
    test("8254390271", 26, RoundingMode::Ceiling, "123");
    test("8254390271", 26, RoundingMode::Nearest, "123");

    test("8254390272", 26, RoundingMode::Down, "123");
    test("8254390272", 26, RoundingMode::Up, "123");
    test("8254390272", 26, RoundingMode::Floor, "123");
    test("8254390272", 26, RoundingMode::Ceiling, "123");
    test("8254390272", 26, RoundingMode::Nearest, "123");
    test("8254390272", 26, RoundingMode::Exact, "123");

    test("8254390273", 26, RoundingMode::Down, "123");
    test("8254390273", 26, RoundingMode::Up, "124");
    test("8254390273", 26, RoundingMode::Floor, "123");
    test("8254390273", 26, RoundingMode::Ceiling, "124");
    test("8254390273", 26, RoundingMode::Nearest, "123");

    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Down,
        "122",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Up,
        "123",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Floor,
        "122",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Ceiling,
        "123",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Nearest,
        "123",
    );

    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Down,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Up,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Floor,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Ceiling,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Nearest,
        "123",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Exact,
        "123",
    );

    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Down,
        "123",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Up,
        "124",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Floor,
        "123",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Ceiling,
        "124",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Nearest,
        "123",
    );

    test("4294967295", 1, RoundingMode::Down, "2147483647");
    test("4294967295", 1, RoundingMode::Up, "2147483648");
    test("4294967295", 1, RoundingMode::Floor, "2147483647");
    test("4294967295", 1, RoundingMode::Ceiling, "2147483648");
    test("4294967295", 1, RoundingMode::Nearest, "2147483648");

    test("4294967296", 1, RoundingMode::Down, "2147483648");
    test("4294967296", 1, RoundingMode::Up, "2147483648");
    test("4294967296", 1, RoundingMode::Floor, "2147483648");
    test("4294967296", 1, RoundingMode::Ceiling, "2147483648");
    test("4294967296", 1, RoundingMode::Nearest, "2147483648");
    test("4294967296", 1, RoundingMode::Exact, "2147483648");

    test("4294967297", 1, RoundingMode::Down, "2147483648");
    test("4294967297", 1, RoundingMode::Up, "2147483649");
    test("4294967297", 1, RoundingMode::Floor, "2147483648");
    test("4294967297", 1, RoundingMode::Ceiling, "2147483649");
    test("4294967297", 1, RoundingMode::Nearest, "2147483648");

    test("1000000000000", 0, RoundingMode::Down, "1000000000000");
    test("1000000000000", 0, RoundingMode::Up, "1000000000000");
    test("1000000000000", 0, RoundingMode::Floor, "1000000000000");
    test("1000000000000", 0, RoundingMode::Ceiling, "1000000000000");
    test("1000000000000", 0, RoundingMode::Nearest, "1000000000000");
    test("1000000000000", 0, RoundingMode::Exact, "1000000000000");

    test("7999999999999", 3, RoundingMode::Down, "999999999999");
    test("7999999999999", 3, RoundingMode::Up, "1000000000000");
    test("7999999999999", 3, RoundingMode::Floor, "999999999999");
    test("7999999999999", 3, RoundingMode::Ceiling, "1000000000000");
    test("7999999999999", 3, RoundingMode::Nearest, "1000000000000");

    test("8000000000000", 3, RoundingMode::Down, "1000000000000");
    test("8000000000000", 3, RoundingMode::Up, "1000000000000");
    test("8000000000000", 3, RoundingMode::Floor, "1000000000000");
    test("8000000000000", 3, RoundingMode::Ceiling, "1000000000000");
    test("8000000000000", 3, RoundingMode::Nearest, "1000000000000");
    test("8000000000000", 3, RoundingMode::Exact, "1000000000000");

    test("8000000000001", 3, RoundingMode::Down, "1000000000000");
    test("8000000000001", 3, RoundingMode::Up, "1000000000001");
    test("8000000000001", 3, RoundingMode::Floor, "1000000000000");
    test("8000000000001", 3, RoundingMode::Ceiling, "1000000000001");
    test("8000000000001", 3, RoundingMode::Nearest, "1000000000000");

    test(
        "16777216000000000000",
        24,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "33554432000000000000",
        25,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "2147483648000000000000",
        31,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "4294967296000000000000",
        32,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "8589934592000000000000",
        33,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Exact,
        "1000000000000",
    );

    test("1000000000000", 10, RoundingMode::Down, "976562500");
    test("1000000000000", 10, RoundingMode::Up, "976562500");
    test("1000000000000", 10, RoundingMode::Floor, "976562500");
    test("1000000000000", 10, RoundingMode::Ceiling, "976562500");
    test("1000000000000", 10, RoundingMode::Nearest, "976562500");
    test("1000000000000", 10, RoundingMode::Exact, "976562500");

    test("980657949", 72, RoundingMode::Down, "0");
    test("980657949", 72, RoundingMode::Up, "1");
    test("980657949", 72, RoundingMode::Floor, "0");
    test("980657949", 72, RoundingMode::Ceiling, "1");
    test("980657949", 72, RoundingMode::Nearest, "0");

    test("4294967295", 31, RoundingMode::Down, "1");
    test("4294967295", 31, RoundingMode::Up, "2");
    test("4294967295", 31, RoundingMode::Floor, "1");
    test("4294967295", 31, RoundingMode::Ceiling, "2");
    test("4294967295", 31, RoundingMode::Nearest, "2");

    test("4294967295", 32, RoundingMode::Down, "0");
    test("4294967295", 32, RoundingMode::Up, "1");
    test("4294967295", 32, RoundingMode::Floor, "0");
    test("4294967295", 32, RoundingMode::Ceiling, "1");
    test("4294967295", 32, RoundingMode::Nearest, "1");

    test("4294967296", 32, RoundingMode::Down, "1");
    test("4294967296", 32, RoundingMode::Up, "1");
    test("4294967296", 32, RoundingMode::Floor, "1");
    test("4294967296", 32, RoundingMode::Ceiling, "1");
    test("4294967296", 32, RoundingMode::Nearest, "1");
    test("4294967296", 32, RoundingMode::Exact, "1");

    test("4294967296", 33, RoundingMode::Down, "0");
    test("4294967296", 33, RoundingMode::Up, "1");
    test("4294967296", 33, RoundingMode::Floor, "0");
    test("4294967296", 33, RoundingMode::Ceiling, "1");
    test("4294967296", 33, RoundingMode::Nearest, "0");

    test("0", -10, RoundingMode::Exact, "0");
    test("123", -1, RoundingMode::Exact, "246");
    test("123", -2, RoundingMode::Exact, "492");
    test("123", -25, RoundingMode::Exact, "4127195136");
    test("123", -26, RoundingMode::Exact, "8254390272");
    test(
        "123",
        -100,
        RoundingMode::Exact,
        "155921023828072216384094494261248",
    );
    test("2147483648", -1, RoundingMode::Exact, "4294967296");
    test("1000000000000", -3, RoundingMode::Exact, "8000000000000");
    test(
        "1000000000000",
        -24,
        RoundingMode::Exact,
        "16777216000000000000",
    );
    test(
        "1000000000000",
        -25,
        RoundingMode::Exact,
        "33554432000000000000",
    );
    test(
        "1000000000000",
        -31,
        RoundingMode::Exact,
        "2147483648000000000000",
    );
    test(
        "1000000000000",
        -32,
        RoundingMode::Exact,
        "4294967296000000000000",
    );
    test(
        "1000000000000",
        -33,
        RoundingMode::Exact,
        "8589934592000000000000",
    );
    test(
        "1000000000000",
        -100,
        RoundingMode::Exact,
        "1267650600228229401496703205376000000000000",
    );
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 1")]
fn shr_round_assign_i32_fail_1() {
    Natural::from(123u32).shr_round_assign(1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 100")]
fn shr_round_assign_i32_fail_2() {
    Natural::from(123u32).shr_round_assign(100i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 1")]
fn shr_round_assign_i32_fail_3() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shr_round_assign(1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 100")]
fn shr_round_assign_i32_fail_4() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shr_round_assign(100i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 1")]
fn shr_round_i32_fail_1() {
    Natural::from(123u32).shr_round(1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >>= 100")]
fn shr_round_i32_fail_2() {
    Natural::from(123u32).shr_round(100i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 1")]
fn shr_round_i32_fail_3() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shr_round(1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >>= 100")]
fn shr_round_i32_fail_4() {
    Natural::from_str("1000000000001")
        .unwrap()
        .shr_round(100i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >> 1")]
fn shr_round_ref_i32_fail_1() {
    (&Natural::from(123u32)).shr_round(1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 123 >> 100")]
fn shr_round_ref_i32_fail_2() {
    (&Natural::from(123u32)).shr_round(100i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >> 1")]
fn shr_round_ref_i32_fail_3() {
    (&Natural::from_str("1000000000001").unwrap()).shr_round(1i32, RoundingMode::Exact);
}

#[test]
#[should_panic(expected = "Right shift is not exact: 1000000000001 >> 100")]
fn shr_round_ref_i32_fail_4() {
    (&Natural::from_str("1000000000001").unwrap()).shr_round(100i32, RoundingMode::Exact);
}

#[test]
fn shr_round_u32_properties() {
    // n.shr_round_assign(i, rm); n is valid.
    // n.shr_round(i, rm) is valid.
    // (&n).shr_round(i, rm) is valid.
    // n.shr_round_assign(i, rm), n.shr_round(u, rm), and (&n).shr_round(u, rm) give the same
    //      result.
    // n.shr_round(u, rm) <= n
    let natural_i32_and_rounding_mode = |mut n: Natural, i: i32, rm: RoundingMode| {
        let old_n = n.clone();
        n.shr_round_assign(i, rm);
        assert!(n.is_valid());

        let n2 = old_n.clone();
        let result = (&n2).shr_round(i, rm);
        assert_eq!(result, n);
        assert!(result.is_valid());
        let result = n2.shr_round(i, rm);
        assert!(result.is_valid());
        assert_eq!(result, n);
    };

    // n.shr_round(0, rm) == n
    #[allow(unknown_lints, identity_op)]
    let natural_and_rounding_mode = |n: Natural, rm: RoundingMode| {
        assert_eq!((&n).shr_round(0i32, rm), n);
    };

    // 0.shr_round(u, rm) == 0
    let i32_and_rounding_mode = |i: i32, rm: RoundingMode| {
        assert_eq!(Natural::ZERO.shr_round(i, rm), 0);
    };

    for (n, u, rm) in select_inputs_2(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_i32_and_rounding_mode(n, u, rm);
    }

    for (n, u, rm) in select_inputs_2(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_i32_and_rounding_mode(n, u, rm);
    }

    for (n, rm) in log_pairs(exhaustive_naturals(), exhaustive_rounding_modes()).take(LARGE_LIMIT) {
        natural_and_rounding_mode(n, rm);
    }

    for (n, rm) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).take(LARGE_LIMIT)
    {
        natural_and_rounding_mode(n, rm);
    }

    for (u, rm) in log_pairs(exhaustive_i(), exhaustive_rounding_modes()).take(LARGE_LIMIT) {
        i32_and_rounding_mode(u, rm);
    }

    for (u, rm) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| i32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).take(LARGE_LIMIT)
    {
        i32_and_rounding_mode(u, rm);
    }
}
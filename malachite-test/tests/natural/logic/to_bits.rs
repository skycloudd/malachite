use std::str::FromStr;

use malachite_base::num::logic::integers::{_to_bits_asc_alt, _to_bits_desc_alt};
use malachite_base::num::logic::traits::{BitConvertible, BitIterable};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::naturals;
use malachite_test::natural::logic::to_bits::{_to_bits_asc_naive, _to_bits_desc_naive};

#[test]
fn test_to_bits_asc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.bits().collect::<Vec<bool>>(), out);
        assert_eq!(n.to_bits_asc(), out);
        assert_eq!(_to_bits_asc_naive(&n), out);
    };
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![false, true, true]);
    test("105", vec![true, false, false, true, false, true, true]);
    test(
        "1000000000000",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true,
        ],
    );
    test(
        "4294967295",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true,
        ],
    );
    test(
        "4294967296",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true,
        ],
    );
    test(
        "18446744073709551615",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true,
        ],
    );
    test(
        "18446744073709551616",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true,
        ],
    );
}

#[test]
fn test_to_bits_desc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.bits().rev().collect::<Vec<bool>>(), out);
        assert_eq!(n.to_bits_desc(), out);
        assert_eq!(_to_bits_desc_naive(&n), out);
        assert_eq!(n._to_bits_desc_alt(), out);
    };
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![true, true, false]);
    test("105", vec![true, true, false, true, false, false, true]);
    test(
        "1000000000000",
        vec![
            true, true, true, false, true, false, false, false, true, true, false, true, false,
            true, false, false, true, false, true, false, false, true, false, true, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false,
        ],
    );
    test(
        "4294967295",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true,
        ],
    );
    test(
        "4294967296",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false,
        ],
    );
    test(
        "18446744073709551615",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true,
        ],
    );
    test(
        "18446744073709551616",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false,
        ],
    );
}

#[test]
fn to_bits_asc_properties() {
    test_properties(naturals, |x| {
        let bits = x.to_bits_asc();
        assert_eq!(_to_bits_asc_naive(x), bits);
        assert_eq!(_to_bits_asc_alt(x), bits);
        assert_eq!(x.bits().collect::<Vec<bool>>(), bits);
        assert_eq!(Natural::from_bits_asc(&bits), *x);
        if *x != 0 {
            assert_eq!(*bits.last().unwrap(), true);
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(u.to_bits_asc(), Natural::from(u).to_bits_asc());
    });
}

#[test]
fn to_bits_desc_properties() {
    test_properties(naturals, |x| {
        let bits = x.to_bits_desc();
        assert_eq!(_to_bits_desc_naive(x), bits);
        assert_eq!(_to_bits_desc_alt(x), bits);
        assert_eq!(x._to_bits_desc_alt(), bits);
        assert_eq!(x.bits().rev().collect::<Vec<bool>>(), bits);
        assert_eq!(Natural::from_bits_desc(&bits), *x);
        if *x != 0 {
            assert_eq!(bits[0], true);
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(u.to_bits_desc(), Natural::from(u).to_bits_desc());
    });
}
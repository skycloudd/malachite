use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::natural::logic::and_u32::limbs_and_limb;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::base::{pairs_of_nonempty_unsigned_vec_and_unsigned, unsigneds};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::logic::and_u32::{
    natural_and_u32_alt_1, natural_and_u32_alt_2, num_and_u32,
};
use num::BigUint;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_limbs_and_limb() {
    let test = |limbs: &[u32], limb: u32, out: u32| {
        assert_eq!(limbs_and_limb(limbs, limb), out);
    };
    test(&[6, 7], 2, 2);
    test(&[100, 101, 102], 10, 0);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_and_limb_fail() {
    limbs_and_limb(&[], 10);
}

#[test]
fn test_and_u32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n &= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n &= v;
        assert_eq!(n.to_string(), out);

        let n = &Natural::from_str(u).unwrap() & v;
        assert_eq!(n.to_string(), out);

        let n = num_and_u32(BigUint::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() & v;
        assert_eq!(n.to_string(), out);

        let n = v & &Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v & rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from(0);
        n.assign(v & &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "0");
    test("123", 0, "0");
    test("123", 456, "72");
    test("1000000000000", 123, "0");
    test("1000000000001", 123, "1");
    test("12345678987654321", 987_654_321, "579887281");
}

#[test]
fn limbs_and_limb_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            assert_eq!(
                limbs_and_limb(limbs, limb),
                &Natural::from_limbs_asc(limbs) & limb
            );
        },
    );
}

#[test]
fn and_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let mut mut_n = n.clone();
            mut_n &= u;
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = natural_to_rug_integer(n);
            rug_n &= u;
            assert_eq!(rug_integer_to_natural(&rug_n), result);

            assert_eq!(n & u, result);
            assert_eq!(u & n, result);
            assert_eq!(natural_and_u32_alt_1(&n, u), result);
            assert_eq!(natural_and_u32_alt_2(&n, u), result);

            assert_eq!(n & Natural::from(u), result);
            assert_eq!(Natural::from(u) & n, result);

            assert_eq!(&result & u, result);

            assert_eq!(num_and_u32(natural_to_biguint(n), u), result);
            assert_eq!(
                rug_integer_to_natural(&(natural_to_rug_integer(n) & u)),
                result
            );

            assert!(result <= *n);
            assert!(result <= u);

            let ones = result.count_ones();
            assert!(ones <= u64::from(n.count_ones()));
            assert!(ones <= u64::from(u.count_ones()));
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n & 0u32, 0);
        assert_eq!(0u32 & n, 0);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(&Natural::ZERO & u, 0);
        assert_eq!(u & &Natural::ZERO, 0);
    });
}
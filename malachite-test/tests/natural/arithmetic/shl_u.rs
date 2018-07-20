use common::{test_properties, test_properties_no_special};
use malachite_base::num::{IsPowerOfTwo, One, Zero};
use malachite_nz::natural::arithmetic::shl_u::{
    limbs_shl, limbs_shl_to_out, limbs_slice_shl_in_place, limbs_vec_shl_in_place,
};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_small_u64, pairs_of_unsigned_vec_and_u32_var_1, small_unsigneds,
    triples_of_unsigned_vec_unsigned_vec_and_u32_var_5,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned,
    triples_of_natural_small_unsigned_and_small_unsigned,
};
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_limbs_shl_and_limbs_vec_shl_in_place() {
    let test = |limbs: &[u32], bits: u64, out: &[u32]| {
        assert_eq!(limbs_shl(limbs, bits), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_shl_in_place(&mut limbs, bits);
        assert_eq!(limbs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[], 100, &[0, 0, 0]);
    test(&[6, 7], 2, &[24, 28]);
    test(&[100, 101, 102], 10, &[102_400, 103_424, 104_448]);
    test(&[123, 456], 1, &[246, 912]);
    test(&[123, 456], 31, &[2_147_483_648, 61, 228]);
    test(&[123, 456], 32, &[0, 123, 456]);
    test(&[123, 456], 100, &[0, 0, 0, 1_968, 7_296]);
}

#[test]
fn test_limbs_shl_to_out() {
    let test = |limbs_out_before: &[u32],
                limbs_in: &[u32],
                bits: u32,
                carry: u32,
                limbs_out_after: &[u32]| {
        let mut limbs_out = limbs_out_before.to_vec();
        assert_eq!(limbs_shl_to_out(&mut limbs_out, limbs_in, bits), carry);
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[], 5, 0, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, 0, &[24, 28, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        0,
        &[102_400, 103_424, 104_448, 10],
    );
    test(&[10, 10, 10, 10], &[123, 456], 1, 0, &[246, 912, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        31,
        228,
        &[2_147_483_648, 61, 10, 10],
    );
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= len")]
fn limbs_shl_to_out_fail_1() {
    limbs_shl_to_out(&mut [10], &[10, 10], 10);
}

#[test]
#[should_panic(expected = "assertion failed: bits > 0")]
fn limbs_shl_to_out_fail_2() {
    limbs_shl_to_out(&mut [10, 10, 10], &[123, 456], 0);
}

#[test]
#[should_panic(expected = "assertion failed: bits < u32::WIDTH")]
fn limbs_shl_to_out_fail_3() {
    limbs_shl_to_out(&mut [10, 10, 10], &[123, 456], 100);
}

#[test]
fn test_limbs_slice_shl_in_place() {
    let test = |limbs: &[u32], bits: u32, carry: u32, out: &[u32]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_slice_shl_in_place(&mut limbs, bits), carry);
        assert_eq!(limbs, out);
    };
    test(&[], 5, 0, &[]);
    test(&[6, 7], 2, 0, &[24, 28]);
    test(&[100, 101, 102], 10, 0, &[102_400, 103_424, 104_448]);
    test(&[123, 456], 1, 0, &[246, 912]);
    test(&[123, 456], 31, 228, &[2_147_483_648, 61]);
}

#[test]
#[should_panic(expected = "assertion failed: bits > 0")]
fn limbs_slice_shl_in_place_fail_1() {
    limbs_slice_shl_in_place(&mut [123, 456], 0);
}

#[test]
#[should_panic(expected = "assertion failed: bits < u32::WIDTH")]
fn limbs_slice_shl_in_place_fail_2() {
    limbs_slice_shl_in_place(&mut [123, 456], 100);
}

#[test]
fn limbs_shl_properties() {
    test_properties(pairs_of_unsigned_vec_and_small_u64, |&(ref limbs, bits)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_shl(limbs, bits)),
            Natural::from_limbs_asc(limbs) << bits
        );
    });
}

#[test]
fn limbs_shl_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_u32_var_5,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            let carry = limbs_shl_to_out(&mut out_limbs, in_limbs, limb);
            let n = Natural::from_limbs_asc(in_limbs) << limb;
            let len = in_limbs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry != 0, limbs.len() == len + 1);
            let mut actual_limbs = out_limbs[..len].to_vec();
            if carry != 0 {
                actual_limbs.push(carry);
            }
            limbs.resize(actual_limbs.len(), 0);
            assert_eq!(limbs, actual_limbs);
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_slice_shl_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_u32_var_1, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        let carry = limbs_slice_shl_in_place(&mut limbs, limb);
        let n = Natural::from_limbs_asc(&old_limbs) << limb;
        let mut expected_limbs = n.into_limbs_asc();
        assert_eq!(carry != 0, expected_limbs.len() == limbs.len() + 1);
        if carry != 0 {
            limbs.push(carry);
        }
        expected_limbs.resize(limbs.len(), 0);
        assert_eq!(limbs, expected_limbs);
    });
}

#[test]
fn limbs_vec_shl_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_small_u64, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        limbs_vec_shl_in_place(&mut limbs, limb);
        let n = Natural::from_limbs_asc(&old_limbs) << limb;
        assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
    });
}

macro_rules! tests_and_properties {
    (
        $t: ident,
        $test_shl_u: ident,
        $shl_u_properties: ident,
        $u: ident,
        $v: ident,
        $out: ident,
        $library_comparison_tests: expr,
        $n: ident,
        $shifted: ident,
        $library_comparison_properties: expr
    ) => {
        #[test]
        fn $test_shl_u() {
            let test = |$u, $v: $t, $out| {
                let mut n = Natural::from_str($u).unwrap();
                n <<= $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = Natural::from_str($u).unwrap() << $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                let n = &Natural::from_str($u).unwrap() << $v;
                assert_eq!(n.to_string(), $out);
                assert!(n.is_valid());

                $library_comparison_tests
            };
            test("0", 0, "0");
            test("0", 10, "0");
            test("123", 0, "123");
            test("123", 1, "246");
            test("123", 2, "492");
            test("123", 25, "4127195136");
            test("123", 26, "8254390272");
            test("123", 100, "155921023828072216384094494261248");
            test("2147483648", 1, "4294967296");
            test("1000000000000", 0, "1000000000000");
            test("1000000000000", 3, "8000000000000");
            test("1000000000000", 24, "16777216000000000000");
            test("1000000000000", 25, "33554432000000000000");
            test("1000000000000", 31, "2147483648000000000000");
            test("1000000000000", 32, "4294967296000000000000");
            test("1000000000000", 33, "8589934592000000000000");
            test(
                "1000000000000",
                100,
                "1267650600228229401496703205376000000000000",
            );
        }

        #[test]
        fn $shl_u_properties() {
            test_properties(pairs_of_natural_and_small_unsigned::<$t>, |&(ref $n, $u)| {
                let mut mut_n = $n.clone();
                mut_n <<= $u;
                assert!(mut_n.is_valid());
                let $shifted = mut_n;

                let shifted_alt = $n << $u;
                assert!(shifted_alt.is_valid());
                assert_eq!(shifted_alt, $shifted);

                let shifted_alt = $n.clone() << $u;
                assert!(shifted_alt.is_valid());
                assert_eq!(shifted_alt, $shifted);

                $library_comparison_properties

                assert!($shifted >= *$n);
                assert_eq!($shifted, $n * (Natural::ONE << $u));
                //TODO assert_eq!(&$shifted >> $u, *$n);

                //TODO
                /*
                if u <= (i32::MAX as u32) {
                    assert_eq!(n << (u as i32), shifted);
                    assert_eq!(n >> -(u as i32), shifted);
                }*/
            });

            test_properties(
                triples_of_natural_small_unsigned_and_small_unsigned::<$t>,
                |&(ref n, u, v)| {
                    if let Some(sum) = u.checked_add(v) {
                        assert_eq!(n << u << v, n << sum);
                    }
                },
            );

            #[allow(unknown_lints, identity_op)]
            test_properties(naturals, |n| {
                assert_eq!(n << $t::ZERO, *n);
                assert_eq!(n << $t::ONE, n * 2);
            });

            test_properties_no_special(small_unsigneds::<$t>, |&u| {
                assert_eq!(Natural::ZERO << u, 0);
                assert!((Natural::ONE << u).is_power_of_two());
            });
        }
    }
}

tests_and_properties!(
    u8,
    test_shl_u8,
    shl_u8_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties!(
    u16,
    test_shl_u16,
    shl_u16_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
tests_and_properties!(
    u32,
    test_shl_u32,
    shl_u32_properties,
    u,
    v,
    out,
    {
        let mut n = rug::Integer::from_str(u).unwrap();
        n <<= v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() << v;
        assert_eq!(n.to_string(), out);

        let n = BigUint::from_str(u).unwrap() << v as usize;
        assert_eq!(n.to_string(), out);

        let n = &BigUint::from_str(u).unwrap() << v as usize;
        assert_eq!(n.to_string(), out);
    },
    n,
    shifted,
    {
        let mut rug_n = natural_to_rug_integer(n);
        rug_n <<= u;
        assert_eq!(rug_integer_to_natural(&rug_n), shifted);

        assert_eq!(
            biguint_to_natural(&(&natural_to_biguint(n) << u as usize)),
            shifted
        );
        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(n) << u as usize)),
            shifted
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(n) << u)),
            shifted
        );
    }
);
tests_and_properties!(
    u64,
    test_shl_u64,
    shl_u64_properties,
    u,
    v,
    out,
    {},
    n,
    shifted,
    {}
);
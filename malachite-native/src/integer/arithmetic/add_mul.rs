use integer::Integer;
use malachite_base::traits::{AddMul, AddMulAssign, SubMul, SubMulAssign};
use natural::arithmetic::add_mul::mpz_aorsmul;
use natural::Natural::{Large, Small};

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`, b,
/// and c by value.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMul;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), Integer::from(4u32)), 22);
///     assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                         .add_mul(Integer::from(65536u32),
///                         Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-65537000000000000");
/// }
/// ```
impl<'a> AddMul<Integer, Integer> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: Integer, c: Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` and
/// b by value and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMul;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), &Integer::from(4u32)), 22);
///     assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                         .add_mul(Integer::from(65536u32),
///                         &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-65537000000000000");
/// }
/// ```
impl<'a> AddMul<Integer, &'a Integer> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: Integer, c: &'a Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` and
/// c by value and b by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMul;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), Integer::from(4u32)), 22);
///     assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                         .add_mul(&Integer::from(65536u32),
///                         Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-65537000000000000");
/// }
/// ```
impl<'a> AddMul<&'a Integer, Integer> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: &'a Integer, c: Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` by
/// value and b and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMul;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), &Integer::from(4u32)), 22);
///     assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                         .add_mul(&Integer::from(65536u32),
///                         &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-65537000000000000");
/// }
/// ```
impl<'a, 'b> AddMul<&'a Integer, &'b Integer> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: &'a Integer, c: &'b Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`, b,
/// and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMul;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(&Integer::from(3u32), &Integer::from(4u32)), 22);
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap())
///                         .add_mul(&Integer::from(65536u32),
///                         &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                 "-65537000000000000");
/// }
/// ```
impl<'a, 'b, 'c> AddMul<&'a Integer, &'b Integer> for &'c Integer {
    type Output = Integer;

    fn add_mul(self, b: &'a Integer, c: &'b Integer) -> Integer {
        match (self, b, c) {
            (
                &Integer {
                    sign: true,
                    abs: Small(0),
                },
                b,
                c,
            ) => b * c,
            (
                a,
                &Integer {
                    sign: true,
                    abs: Small(b),
                },
                c,
            ) => a.add_mul(c, b),
            (
                a,
                &Integer {
                    sign: false,
                    abs: Small(b),
                },
                c,
            ) => a.sub_mul(c, b),
            (
                a,
                b,
                &Integer {
                    sign: true,
                    abs: Small(c),
                },
            ) => a.add_mul(b, c),
            (
                a,
                b,
                &Integer {
                    sign: false,
                    abs: Small(c),
                },
            ) => a.sub_mul(b, c),
            (
                &Integer {
                    sign: a_sign,
                    abs: ref a_abs,
                },
                &Integer {
                    sign: b_sign,
                    abs: Large(ref b_limbs),
                },
                &Integer {
                    sign: c_sign,
                    abs: Large(ref c_limbs),
                },
            ) => {
                let mut result_sign = !a_sign;
                let mut result_limbs = a_abs.to_limbs_le();
                mpz_aorsmul(
                    &mut result_sign,
                    &mut result_limbs,
                    !b_sign,
                    b_limbs,
                    !c_sign,
                    c_limbs,
                    true,
                );
                result_sign = !result_sign;
                let mut abs_result = Large(result_limbs);
                abs_result.trim();
                Integer {
                    sign: result_sign,
                    abs: abs_result,
                }
            }
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b and c by value.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(Integer::from(3u32), Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Integer::from_str("-1000000000000").unwrap();
///     x.add_mul_assign(Integer::from(65536u32), Integer::from_str("-1000000000000").unwrap());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl AddMulAssign<Integer, Integer> for Integer {
    fn add_mul_assign(&mut self, b: Integer, c: Integer) {
        match (self, b, c) {
            (
                a @ &mut Integer {
                    sign: true,
                    abs: Small(0),
                },
                b,
                c,
            ) => *a = b * c,
            (
                a,
                Integer {
                    sign: true,
                    abs: Small(b),
                },
                c,
            ) => a.add_mul_assign(c, b),
            (
                a,
                Integer {
                    sign: false,
                    abs: Small(b),
                },
                c,
            ) => a.sub_mul_assign(c, b),
            (
                a,
                b,
                Integer {
                    sign: true,
                    abs: Small(c),
                },
            ) => a.add_mul_assign(b, c),
            (
                a,
                b,
                Integer {
                    sign: false,
                    abs: Small(c),
                },
            ) => a.sub_mul_assign(b, c),
            (
                &mut Integer {
                    sign: ref mut a_sign,
                    abs: ref mut a_abs,
                },
                Integer {
                    sign: b_sign,
                    abs: Large(ref b_limbs),
                },
                Integer {
                    sign: c_sign,
                    abs: Large(ref c_limbs),
                },
            ) => {
                let mut result_sign = !*a_sign;
                mpz_aorsmul(
                    &mut result_sign,
                    a_abs.promote_in_place(),
                    !b_sign,
                    b_limbs,
                    !c_sign,
                    c_limbs,
                    true,
                );
                *a_sign = !result_sign;
                a_abs.trim();
            }
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b by value and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(Integer::from(3u32), &Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Integer::from_str("-1000000000000").unwrap();
///     x.add_mul_assign(Integer::from(65536u32), &Integer::from_str("-1000000000000").unwrap());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> AddMulAssign<Integer, &'a Integer> for Integer {
    fn add_mul_assign(&mut self, b: Integer, c: &'a Integer) {
        match (self, b, c) {
            (
                a @ &mut Integer {
                    sign: true,
                    abs: Small(0),
                },
                b,
                c,
            ) => *a = b * c,
            (
                a,
                Integer {
                    sign: true,
                    abs: Small(b),
                },
                c,
            ) => a.add_mul_assign(c, b),
            (
                a,
                Integer {
                    sign: false,
                    abs: Small(b),
                },
                c,
            ) => a.sub_mul_assign(c, b),
            (
                a,
                b,
                &Integer {
                    sign: true,
                    abs: Small(c),
                },
            ) => a.add_mul_assign(b, c),
            (
                a,
                b,
                &Integer {
                    sign: false,
                    abs: Small(c),
                },
            ) => a.sub_mul_assign(b, c),
            (
                &mut Integer {
                    sign: ref mut a_sign,
                    abs: ref mut a_abs,
                },
                Integer {
                    sign: b_sign,
                    abs: Large(ref b_limbs),
                },
                &Integer {
                    sign: c_sign,
                    abs: Large(ref c_limbs),
                },
            ) => {
                let mut result_sign = !*a_sign;
                mpz_aorsmul(
                    &mut result_sign,
                    a_abs.promote_in_place(),
                    !b_sign,
                    b_limbs,
                    !c_sign,
                    c_limbs,
                    true,
                );
                *a_sign = !result_sign;
                a_abs.trim();
            }
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b by reference and c by value.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(&Integer::from(3u32), Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Integer::from_str("-1000000000000").unwrap();
///     x.add_mul_assign(&Integer::from(65536u32), Integer::from_str("-1000000000000").unwrap());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> AddMulAssign<&'a Integer, Integer> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: Integer) {
        match (self, b, c) {
            (
                a @ &mut Integer {
                    sign: true,
                    abs: Small(0),
                },
                b,
                c,
            ) => *a = b * c,
            (
                a,
                &Integer {
                    sign: true,
                    abs: Small(b),
                },
                c,
            ) => a.add_mul_assign(c, b),
            (
                a,
                &Integer {
                    sign: false,
                    abs: Small(b),
                },
                c,
            ) => a.sub_mul_assign(c, b),
            (
                a,
                b,
                Integer {
                    sign: true,
                    abs: Small(c),
                },
            ) => a.add_mul_assign(b, c),
            (
                a,
                b,
                Integer {
                    sign: false,
                    abs: Small(c),
                },
            ) => a.sub_mul_assign(b, c),
            (
                &mut Integer {
                    sign: ref mut a_sign,
                    abs: ref mut a_abs,
                },
                &Integer {
                    sign: b_sign,
                    abs: Large(ref b_limbs),
                },
                Integer {
                    sign: c_sign,
                    abs: Large(ref c_limbs),
                },
            ) => {
                let mut result_sign = !*a_sign;
                mpz_aorsmul(
                    &mut result_sign,
                    a_abs.promote_in_place(),
                    !b_sign,
                    b_limbs,
                    !c_sign,
                    c_limbs,
                    true,
                );
                *a_sign = !result_sign;
                a_abs.trim();
            }
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(&Integer::from(3u32), &Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Integer::from_str("-1000000000000").unwrap();
///     x.add_mul_assign(&Integer::from(65536u32), &Integer::from_str("-1000000000000").unwrap());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a, 'b> AddMulAssign<&'a Integer, &'b Integer> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: &'b Integer) {
        match (self, b, c) {
            (
                a @ &mut Integer {
                    sign: true,
                    abs: Small(0),
                },
                b,
                c,
            ) => *a = b * c,
            (
                a,
                &Integer {
                    sign: true,
                    abs: Small(b),
                },
                c,
            ) => a.add_mul_assign(c, b),
            (
                a,
                &Integer {
                    sign: false,
                    abs: Small(b),
                },
                c,
            ) => a.sub_mul_assign(c, b),
            (
                a,
                b,
                &Integer {
                    sign: true,
                    abs: Small(c),
                },
            ) => a.add_mul_assign(b, c),
            (
                a,
                b,
                &Integer {
                    sign: false,
                    abs: Small(c),
                },
            ) => a.sub_mul_assign(b, c),
            (
                &mut Integer {
                    sign: ref mut a_sign,
                    abs: ref mut a_abs,
                },
                &Integer {
                    sign: b_sign,
                    abs: Large(ref b_limbs),
                },
                &Integer {
                    sign: c_sign,
                    abs: Large(ref c_limbs),
                },
            ) => {
                let mut result_sign = !*a_sign;
                mpz_aorsmul(
                    &mut result_sign,
                    a_abs.promote_in_place(),
                    !b_sign,
                    b_limbs,
                    !c_sign,
                    c_limbs,
                    true,
                );
                *a_sign = !result_sign;
                a_abs.trim();
            }
        }
    }
}

use num::arithmetic::traits::{ModPowerOfTwo, ModPowerOfTwoAdd, ModPowerOfTwoAddAssign};
use num::basic::integers::PrimitiveInteger;

#[inline]
pub fn _mod_power_of_two_add<T: PrimitiveInteger>(x: T, other: T, pow: u64) -> T
where
    T: ModPowerOfTwo<Output = T>,
{
    assert!(pow <= T::WIDTH);
    x.wrapping_add(other).mod_power_of_two(pow)
}

#[inline]
pub fn _mod_power_of_two_add_assign<T: PrimitiveInteger>(x: &mut T, other: T, pow: u64) {
    assert!(pow <= T::WIDTH);
    x.wrapping_add_assign(other);
    x.mod_power_of_two_assign(pow);
}

macro_rules! impl_mod_power_of_two_add {
    ($t:ident) => {
        impl ModPowerOfTwoAdd<$t> for $t {
            type Output = $t;

            /// Computes `self + other` mod 2<sup>`pow`</sup>. Assumes the inputs are already
            /// reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAdd;
            ///
            /// assert_eq!(0u8.mod_power_of_two_add(2, 5), 2);
            /// assert_eq!(10u32.mod_power_of_two_add(14, 4), 8);
            /// ```
            #[inline]
            fn mod_power_of_two_add(self, other: $t, pow: u64) -> $t {
                _mod_power_of_two_add(self, other, pow)
            }
        }

        impl ModPowerOfTwoAddAssign<$t> for $t {
            /// Replaces `self` with `self + other` mod 2<sup>`pow`</sup>. Assumes the inputs are
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAddAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_power_of_two_add_assign(2, 5);
            /// assert_eq!(n, 2);
            ///
            /// let mut n = 10u32;
            /// n.mod_power_of_two_add_assign(14, 4);
            /// assert_eq!(n, 8);
            /// ```
            #[inline]
            fn mod_power_of_two_add_assign(&mut self, other: $t, pow: u64) {
                _mod_power_of_two_add_assign(self, other, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_two_add);

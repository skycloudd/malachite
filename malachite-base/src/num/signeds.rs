use std::cmp::Ordering;
use std::num::ParseIntError;
use std::ops::Neg;

use comparison::{Max, Min};
use conversion::CheckedFrom;
use crement::Crementable;
use named::Named;
use num::integers::PrimitiveInteger;
use num::traits::{
    Abs, BitAccess, BitScan, CeilingDivAssignMod, CeilingDivMod, CeilingDivNegMod, CeilingMod,
    CeilingModAssign, CheckedAbs, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem,
    CheckedShl, CheckedShr, CheckedSub, CountOnes, CountZeros, DivAssignMod, DivAssignRem,
    DivExact, DivExactAssign, DivMod, DivRem, DivRound, DivRoundAssign, DivisibleBy,
    DivisibleByPowerOfTwo, Endian, EqMod, EqModPowerOfTwo, FromStrRadix, HammingDistance,
    LeadingZeros, Mod, ModAssign, NegAssign, NegMod, NegativeOne, NotAssign, One, OrdAbs,
    OverflowingAbs, OverflowingAdd, OverflowingAddAssign, OverflowingDiv, OverflowingDivAssign,
    OverflowingMul, OverflowingMulAssign, OverflowingNeg, OverflowingNegAssign, OverflowingRem,
    OverflowingRemAssign, OverflowingShl, OverflowingShr, OverflowingSub, OverflowingSubAssign,
    Parity, PartialOrdAbs, Pow, RotateLeft, RotateRight, SaturatingAdd, SaturatingAddAssign,
    SaturatingMul, SaturatingMulAssign, SaturatingSub, SaturatingSubAssign, Sign, SignificantBits,
    TrailingZeros, Two, UnsignedAbs, WrappingAbs, WrappingAdd, WrappingAddAssign, WrappingDiv,
    WrappingDivAssign, WrappingMul, WrappingMulAssign, WrappingNeg, WrappingNegAssign, WrappingRem,
    WrappingRemAssign, WrappingShl, WrappingShr, WrappingSub, WrappingSubAssign, Zero,
};
use num::unsigneds::PrimitiveUnsigned;
use round::RoundingMode;

//TODO docs
pub trait PrimitiveSigned:
    Abs<Output = Self>
    + CeilingMod
    + CeilingModAssign
    + CheckedAbs<Output = Self>
    + From<i8>
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + OverflowingAbs<Output = Self>
    + PrimitiveInteger
    + Sign
    + UnsignedAbs
    + WrappingAbs<Output = Self>
{
    type UnsignedOfEqualWidth: PrimitiveUnsigned;

    fn to_unsigned_bitwise(self) -> Self::UnsignedOfEqualWidth;

    fn from_unsigned_bitwise(u: Self::UnsignedOfEqualWidth) -> Self;
}

//TODO docs
macro_rules! signed_traits {
    (
        $t:ident,
        $ut:ident,
        $log_width:expr
    ) => {
        integer_traits!($t, $log_width);

        //TODO docs
        impl PrimitiveSigned for $t {
            type UnsignedOfEqualWidth = $ut;

            #[inline]
            fn to_unsigned_bitwise(self) -> Self::UnsignedOfEqualWidth {
                self as $ut
            }

            #[inline]
            fn from_unsigned_bitwise(u: Self::UnsignedOfEqualWidth) -> Self {
                u as $t
            }
        }

        impl OrdAbs for $t {
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                self.unsigned_abs().cmp(&other.unsigned_abs())
            }
        }

        impl Abs for $t {
            type Output = $t;

            #[inline]
            fn abs(self) -> $t {
                $t::abs(self)
            }
        }

        impl UnsignedAbs for $t {
            type Output = $ut;

            #[inline]
            fn unsigned_abs(self) -> $ut {
                $t::wrapping_abs(self) as $ut
            }
        }

        impl CheckedAbs for $t {
            type Output = $t;

            #[inline]
            fn abs(self) -> Option<$t> {
                $t::checked_abs(self)
            }
        }

        impl WrappingAbs for $t {
            type Output = $t;

            #[inline]
            fn abs(self) -> $t {
                $t::wrapping_abs(self)
            }
        }

        impl OverflowingAbs for $t {
            type Output = $t;

            #[inline]
            fn abs(self) -> ($t, bool) {
                $t::overflowing_abs(self)
            }
        }

        /// Returns the number of significant bits of a primitive signed integer; this is the
        /// integer's width minus the number of leading zeros of its absolute value.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Example
        /// ```
        /// use malachite_base::num::traits::SignificantBits;
        ///
        /// fn main() {
        ///     assert_eq!(0i8.significant_bits(), 0);
        ///     assert_eq!((-100i64).significant_bits(), 7);
        /// }
        /// ```
        impl SignificantBits for $t {
            #[inline]
            fn significant_bits(self) -> u64 {
                self.unsigned_abs().significant_bits()
            }
        }

        /// Provides functions for accessing and modifying the `index`th bit of a primitive signed
        /// integer, or the coefficient of 2^<pow>`index`</pow> in its binary expansion.
        ///
        /// Negative integers are represented in two's complement.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::traits::BitAccess;
        ///
        /// let mut x = 0i8;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, 100);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = -0x100i16;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, -156);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, -256);
        ///
        /// let mut x = 0i32;
        /// x.flip_bit(10);
        /// assert_eq!(x, 1024);
        /// x.flip_bit(10);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = -1i64;
        /// x.flip_bit(10);
        /// assert_eq!(x, -1025);
        /// x.flip_bit(10);
        /// assert_eq!(x, -1);
        /// ```
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive signed integer, or the
            /// coefficient of 2<pow>`index`</pow> in its binary expansion, is 0 or 1. `false` means
            /// 0, `true` means 1.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Accessing bits beyond the type's width is allowed; those bits are false if the
            /// integer is non-negative and true if it is negative.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::traits::BitAccess;
            ///
            /// assert_eq!(123i8.get_bit(2), false);
            /// assert_eq!(123i16.get_bit(3), true);
            /// assert_eq!(123i32.get_bit(100), false);
            /// assert_eq!((-123i8).get_bit(0), true);
            /// assert_eq!((-123i16).get_bit(1), false);
            /// assert_eq!((-123i32).get_bit(100), true);
            /// assert_eq!(1_000_000_000_000i64.get_bit(12), true);
            /// assert_eq!(1_000_000_000_000i64.get_bit(100), false);
            /// assert_eq!((-1_000_000_000_000i64).get_bit(12), true);
            /// assert_eq!((-1_000_000_000_000i64).get_bit(100), true);
            /// ```
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                if index < Self::WIDTH.into() {
                    self & (1 << index) != 0
                } else {
                    *self < 0
                }
            }

            /// Sets the `index`th bit of a primitive signed integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 1.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Setting bits beyond the type's width is disallowed if the integer is non-negative;
            /// if it is negative, it's allowed but does nothing since those bits are already true.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH && self >= 0`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::traits::BitAccess;
            ///
            /// let mut x = 0i8;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -0x100i16;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, -156);
            /// ```
            #[inline]
            fn set_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self |= 1 << index;
                } else if *self >= 0 {
                    panic!(
                        "Cannot set bit {} in non-negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }

            /// Sets the `index`th bit of a primitive signed integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 0.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Clearing bits beyond the type's width is disallowed if the integer is negative; if
            /// it is non-negative, it's allowed but does nothing since those bits are already
            /// false.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH && self < 0`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::traits::BitAccess;
            ///
            /// let mut x = 0x7fi8;
            /// x.clear_bit(0);
            /// x.clear_bit(1);
            /// x.clear_bit(3);
            /// x.clear_bit(4);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -156i16;
            /// x.clear_bit(2);
            /// x.clear_bit(5);
            /// x.clear_bit(6);
            /// assert_eq!(x, -256);
            /// ```
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self &= !(1 << index);
                } else if *self < 0 {
                    panic!(
                        "Cannot clear bit {} in negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }
        }

        //TODO docs, test
        impl NegAssign for $t {
            #[inline]
            fn neg_assign(&mut self) {
                *self = -*self;
            }
        }

        //TODO
        impl BitScan for $t {
            #[inline]
            fn index_of_next_false_bit(self, starting_index: u64) -> Option<u64> {
                if starting_index >= u64::from(Self::WIDTH) - 1 {
                    if self >= 0 {
                        Some(starting_index)
                    } else {
                        None
                    }
                } else {
                    let index = (!(self | ((1 << starting_index) - 1)))
                        .trailing_zeros()
                        .into();
                    if index == $t::WIDTH.into() {
                        None
                    } else {
                        Some(index)
                    }
                }
            }

            #[inline]
            fn index_of_next_true_bit(self, starting_index: u64) -> Option<u64> {
                if starting_index >= u64::from(Self::WIDTH) - 1 {
                    if self >= 0 {
                        None
                    } else {
                        Some(starting_index)
                    }
                } else {
                    let index = (self & !((1 << starting_index) - 1))
                        .trailing_zeros()
                        .into();
                    if index == $t::WIDTH.into() {
                        None
                    } else {
                        Some(index)
                    }
                }
            }
        }

        impl DivisibleByPowerOfTwo for $t {
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                self.to_unsigned_bitwise().divisible_by_power_of_two(pow)
            }
        }

        impl DivMod for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            #[inline]
            fn div_mod(self, other: $t) -> ($t, $t) {
                let (quotient, remainder) = if (self >= 0) == (other >= 0) {
                    let (quotient, remainder) = self.unsigned_abs().div_mod(other.unsigned_abs());
                    ($t::checked_from(quotient).unwrap(), remainder)
                } else {
                    let (quotient, remainder) = self
                        .unsigned_abs()
                        .ceiling_div_neg_mod(other.unsigned_abs());
                    (-$t::checked_from(quotient).unwrap(), remainder)
                };
                (
                    quotient,
                    if other >= 0 {
                        $t::checked_from(remainder).unwrap()
                    } else {
                        -$t::checked_from(remainder).unwrap()
                    },
                )
            }
        }

        impl DivAssignMod for $t {
            type ModOutput = $t;

            #[inline]
            fn div_assign_mod(&mut self, rhs: $t) -> $t {
                let (q, r) = self.div_mod(rhs);
                *self = q;
                r
            }
        }

        impl Mod for $t {
            type Output = $t;

            #[inline]
            fn mod_op(self, other: $t) -> $t {
                let remainder = if (self >= 0) == (other >= 0) {
                    self.unsigned_abs().mod_op(other.unsigned_abs())
                } else {
                    self.unsigned_abs().neg_mod(other.unsigned_abs())
                };
                if other >= 0 {
                    $t::checked_from(remainder).unwrap()
                } else {
                    -$t::checked_from(remainder).unwrap()
                }
            }
        }

        impl CeilingDivMod for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            #[inline]
            fn ceiling_div_mod(self, other: $t) -> ($t, $t) {
                let (quotient, remainder) = if (self >= 0) == (other >= 0) {
                    let (quotient, remainder) = self
                        .unsigned_abs()
                        .ceiling_div_neg_mod(other.unsigned_abs());
                    ($t::checked_from(quotient).unwrap(), remainder)
                } else {
                    let (quotient, remainder) = self.unsigned_abs().div_mod(other.unsigned_abs());
                    (-$t::checked_from(quotient).unwrap(), remainder)
                };
                (
                    quotient,
                    if other >= 0 {
                        -$t::checked_from(remainder).unwrap()
                    } else {
                        $t::checked_from(remainder).unwrap()
                    },
                )
            }
        }

        impl CeilingDivAssignMod for $t {
            type ModOutput = $t;

            #[inline]
            fn ceiling_div_assign_mod(&mut self, rhs: $t) -> $t {
                let (q, r) = self.ceiling_div_mod(rhs);
                *self = q;
                r
            }
        }

        impl CeilingMod for $t {
            type Output = $t;

            #[inline]
            fn ceiling_mod(self, other: $t) -> $t {
                let remainder = if (self >= 0) == (other >= 0) {
                    self.unsigned_abs().neg_mod(other.unsigned_abs())
                } else {
                    self.unsigned_abs().mod_op(other.unsigned_abs())
                };
                if other >= 0 {
                    -$t::checked_from(remainder).unwrap()
                } else {
                    $t::checked_from(remainder).unwrap()
                }
            }
        }

        impl CeilingModAssign for $t {
            #[inline]
            fn ceiling_mod_assign(&mut self, rhs: $t) {
                *self = self.ceiling_mod(rhs);
            }
        }

        impl Sign for $t {
            fn sign(&self) -> Ordering {
                self.cmp(&0)
            }
        }

        impl DivRound for $t {
            type Output = $t;

            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                let result_sign = (self >= 0) == (other >= 0);
                let abs = if result_sign {
                    self.unsigned_abs().div_round(other.unsigned_abs(), rm)
                } else {
                    self.unsigned_abs().div_round(other.unsigned_abs(), -rm)
                };
                if result_sign {
                    $t::checked_from(abs).unwrap()
                } else {
                    -$t::checked_from(abs).unwrap()
                }
            }
        }
    };
}

signed_traits!(i8, u8, 3);
signed_traits!(i16, u16, 4);
signed_traits!(i32, u32, 5);
signed_traits!(i64, u64, 6);
signed_traits!(i128, u128, 7);
signed_traits!(isize, usize, 0usize.trailing_zeros().trailing_zeros());

/// Implements the constants 0, 1, 2, and -1 for signed primitive integers.
macro_rules! impl01_signed {
    ($t:ty) => {
        impl01_unsigned!($t);

        /// The constant -1 for signed primitive integers.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl NegativeOne for $t {
            const NEGATIVE_ONE: $t = -1;
        }
    };
}

impl01_signed!(i8);
impl01_signed!(i16);
impl01_signed!(i32);
impl01_signed!(i64);
impl01_signed!(i128);
impl01_signed!(isize);
use comparison::traits::{Max, Min};
use named::Named;
use num::arithmetic::traits::{ModPowerOfTwo, NegAssign};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::{NegativeOne, One, Two, Zero};
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{ExactFrom, ExactInto, WrappingFrom};
use num::float::nice_float::FmtRyuString;
use num::logic::traits::{BitAccess, LowMask, TrailingZeros};
use std::fmt::{Debug, Display, LowerExp, UpperExp};
use std::iter::{Product, Sum};
use std::num::FpCategory;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;

//TODO docs
pub trait PrimitiveFloat:
    'static
    + Add<Output = Self>
    + AddAssign<Self>
    + Copy
    + Debug
    + Default
    + Display
    + Div<Output = Self>
    + DivAssign
    + Display
    + FmtRyuString
    + From<f32>
    + FromStr
    + LowerExp
    + Min
    + Max
    + Mul<Output = Self>
    + MulAssign<Self>
    + Named
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + One
    + PartialEq<Self>
    + PartialOrd<Self>
    + Product
    + Rem<Output = Self>
    + RemAssign<Self>
    + Sized
    + Sub<Output = Self>
    + SubAssign<Self>
    + Sum<Self>
    + Two
    + UpperExp
    + Zero
{
    type UnsignedOfEqualWidth: PrimitiveUnsigned;

    /// $M+E+1$
    /// - For `f32`, this is 32.
    /// - For `f64`, this is 64.
    const WIDTH: u64 = Self::UnsignedOfEqualWidth::WIDTH;
    /// - For `f32`, this is 8.
    /// - For `f64`, this is 11.
    const EXPONENT_WIDTH: u64 = Self::WIDTH - Self::MANTISSA_WIDTH - 1;
    /// - For `f32`, this is 23.
    /// - For `f64`, this is 52.
    const MANTISSA_WIDTH: u64;
    /// $2-2^{E-1}$
    /// - For `f32`, this is -126.
    /// - For `f64`, this is -1022.
    const MIN_NORMAL_EXPONENT: i64 = -(1 << (Self::EXPONENT_WIDTH - 1)) + 2;
    /// $2-2^{E-1}-M$
    /// - For `f32`, this is -149.
    /// - For `f64`, this is -1074.
    const MIN_EXPONENT: i64 = Self::MIN_NORMAL_EXPONENT - (Self::MANTISSA_WIDTH as i64);
    /// $2^{E-1}-1$
    /// - For `f32`, this is 127.
    /// - For `f64`, this is 1023.
    const MAX_EXPONENT: u64 = (1 << (Self::EXPONENT_WIDTH - 1)) - 1;
    /// $2^{2-2^{E-1}-M}$
    /// - For `f32`, this is $2^{-149}$, or `1.0e-45`.
    /// - For `f64`, this is $2^{-1074}$, or `5.0e-324`.
    const MIN_POSITIVE_SUBNORMAL: Self;
    /// $2^{2-2^{E-1}-M}(2^M-1)$
    /// - For `f32`, this is $2^{-149}(2^{23}-1)$, or `1.1754942e-38`.
    /// - For `f64`, this is $2^{-1074}(2^{52}-1)$, or `2.225073858507201e-308`.
    const MAX_SUBNORMAL: Self;
    /// $2^{2-2^{E-1}}$
    /// - For `f32`, this is $2^{-126}$, or `1.1754944e-38`.
    /// - For `f64`, this is $2^{-1022}$, or `2.2250738585072014e-308`.
    const MIN_POSITIVE_NORMAL: Self;
    /// $2^{2^{E-1}-1}(2-2^{-M})$
    /// - For `f32`, this is $2^{127}(2-2^{-23})$, or `3.4028235e38`.
    /// - For `f64`, this is $2^{1023}(2-2^{-52})$, or `1.7976931348623157e308`.
    const MAX_FINITE: Self;
    const NEGATIVE_ZERO: Self;
    const POSITIVE_INFINITY: Self;
    const NEGATIVE_INFINITY: Self;
    const NAN: Self;
    /// $2^{M+1}+1$
    /// - For `f32`, this is $2^{24}+1$, or 16777217.
    /// - For `f64`, this is $2^{53}+1$, or 9007199254740993.
    const SMALLEST_UNREPRESENTABLE_UINT: Self::UnsignedOfEqualWidth;
    /// $2^{M+1}(2^E-1)+1$
    /// - For `f32`, this is $2^{32}-2^{24}+1$, or 4278190081.
    /// - For `f64`, this is $2^{64}-2^{53}+1$, or 18437736874454810625.
    const LARGEST_ORDERED_REPRESENTATION: Self::UnsignedOfEqualWidth;

    fn is_nan(self) -> bool;

    fn is_infinite(self) -> bool;

    fn is_finite(self) -> bool;

    fn is_normal(self) -> bool;

    fn classify(self) -> FpCategory;

    fn is_sign_positive(self) -> bool;

    fn is_sign_negative(self) -> bool;

    fn to_bits(self) -> Self::UnsignedOfEqualWidth;

    fn from_bits(v: Self::UnsignedOfEqualWidth) -> Self;

    //TODO test
    #[inline]
    fn is_negative_zero(self) -> bool {
        self.is_sign_negative() && self == Self::ZERO
    }

    fn abs_negative_zeros(self) -> Self {
        if self == Self::ZERO {
            Self::ZERO
        } else {
            self
        }
    }

    fn abs_negative_zeros_assign(&mut self) {
        if *self == Self::ZERO {
            *self = Self::ZERO;
        }
    }

    fn raw_exponent(self) -> u64 {
        let exponent: u64 = (self.to_bits() >> Self::MANTISSA_WIDTH).exact_into();
        exponent.mod_power_of_two(Self::EXPONENT_WIDTH)
    }

    fn exponent(self) -> i64 {
        assert!(self.is_finite());
        assert!(self != Self::ZERO);
        i64::exact_from(self.raw_exponent()) - i64::exact_from(Self::MAX_EXPONENT)
    }

    fn raw_mantissa_and_exponent(self) -> (Self::UnsignedOfEqualWidth, u64) {
        let bits = self.to_bits();
        let mantissa = bits.mod_power_of_two(Self::MANTISSA_WIDTH);
        let exponent: u64 = (bits >> Self::MANTISSA_WIDTH).exact_into();
        let exponent = exponent.mod_power_of_two(Self::EXPONENT_WIDTH);
        (mantissa, exponent)
    }

    fn mantissa_and_exponent(self) -> (Self::UnsignedOfEqualWidth, i64) {
        assert!(self.is_finite());
        assert!(self != Self::ZERO);
        let (raw_mantissa, raw_exponent) = self.raw_mantissa_and_exponent();
        let mut mantissa;
        let exponent;
        if raw_exponent == 0 {
            let trailing_zeros = raw_mantissa.trailing_zeros();
            mantissa = raw_mantissa >> trailing_zeros;
            exponent = Self::MIN_EXPONENT + i64::wrapping_from(trailing_zeros);
        } else {
            mantissa = raw_mantissa;
            mantissa.set_bit(Self::MANTISSA_WIDTH);
            let trailing_zeros = mantissa.trailing_zeros();
            mantissa >>= trailing_zeros;
            exponent = i64::wrapping_from(raw_exponent)
                + i64::wrapping_from(trailing_zeros)
                + Self::MIN_EXPONENT
                - 1;
        }
        (mantissa, exponent)
    }

    fn from_raw_mantissa_and_exponent(mantissa: Self::UnsignedOfEqualWidth, exponent: u64) -> Self {
        Self::from_bits(
            (Self::UnsignedOfEqualWidth::exact_from(exponent) << Self::MANTISSA_WIDTH) | mantissa,
        )
    }

    fn to_ordered_representation(self) -> Self::UnsignedOfEqualWidth {
        assert!(!self.is_nan());
        let bits = self.to_bits();
        if self.is_sign_positive() {
            (Self::UnsignedOfEqualWidth::low_mask(Self::EXPONENT_WIDTH) << Self::MANTISSA_WIDTH)
                + bits
                + Self::UnsignedOfEqualWidth::ONE
        } else {
            (Self::UnsignedOfEqualWidth::low_mask(Self::EXPONENT_WIDTH + 1) << Self::MANTISSA_WIDTH)
                - bits
        }
    }

    fn from_ordered_representation(n: Self::UnsignedOfEqualWidth) -> Self {
        let offset =
            Self::UnsignedOfEqualWidth::low_mask(Self::EXPONENT_WIDTH) << Self::MANTISSA_WIDTH;
        let f = if n <= offset {
            Self::from_bits(
                (Self::UnsignedOfEqualWidth::low_mask(Self::EXPONENT_WIDTH + 1)
                    << Self::MANTISSA_WIDTH)
                    - n,
            )
        } else {
            let f = Self::from_bits(n - offset - Self::UnsignedOfEqualWidth::ONE);
            assert!(f.is_sign_positive());
            f
        };
        assert!(!f.is_nan());
        f
    }

    fn next_higher(self) -> Self {
        assert!(!self.is_nan());
        if self.is_sign_positive() {
            assert_ne!(self, Self::POSITIVE_INFINITY);
            Self::from_bits(self.to_bits() + Self::UnsignedOfEqualWidth::ONE)
        } else if self == Self::ZERO {
            // negative zero -> positive zero
            Self::ZERO
        } else {
            Self::from_bits(self.to_bits() - Self::UnsignedOfEqualWidth::ONE)
        }
    }

    fn next_lower(self) -> Self {
        assert!(!self.is_nan());
        if self.is_sign_negative() {
            assert_ne!(self, Self::NEGATIVE_INFINITY);
            Self::from_bits(self.to_bits() + Self::UnsignedOfEqualWidth::ONE)
        } else if self == Self::ZERO {
            // positive zero -> negative zero
            Self::NEGATIVE_ZERO
        } else {
            Self::from_bits(self.to_bits() - Self::UnsignedOfEqualWidth::ONE)
        }
    }
}

pub mod basic;
pub mod nice_float;

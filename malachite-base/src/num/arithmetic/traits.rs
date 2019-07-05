use std::cmp::Ordering;

use round::RoundingMode;

/// Checked addition. Computes `self + rhs`, returning `None` if there is no valid result.
pub trait CheckedAdd<RHS = Self> {
    type Output;

    fn checked_add(self, rhs: RHS) -> Option<Self::Output>;
}

/// Checked subtraction. Computes `self - rhs`, returning `None` if there is no valid result.
pub trait CheckedSub<RHS = Self> {
    type Output;

    fn checked_sub(self, rhs: RHS) -> Option<Self::Output>;
}

/// Checked multiplication. Computes `self * rhs`, returning `None` if there is no valid result.
pub trait CheckedMul<RHS = Self> {
    type Output;

    fn checked_mul(self, rhs: RHS) -> Option<Self::Output>;
}

/// Checked division. Computes `self / rhs`, returning `None` if there is no valid result.
pub trait CheckedDiv<RHS = Self> {
    type Output;

    fn checked_div(self, rhs: RHS) -> Option<Self::Output>;
}

/// Checked remainder. Computes `self % rhs`, returning `None` if there is no valid result.
pub trait CheckedRem<RHS = Self> {
    type Output;

    fn checked_rem(self, rhs: RHS) -> Option<Self::Output>;
}

/// Checked negation. Computes `-self`, returning `None` if there is no valid result.
pub trait CheckedNeg {
    type Output;

    fn checked_neg(self) -> Option<Self::Output>;
}

/// Checked absolute value. Computes `self.abs()`, returning `None` if there is no valid result.
pub trait CheckedAbs {
    type Output;

    fn abs(self) -> Option<Self::Output>;
}

/// Raises `self` to the power of `exp`, returning `None` if there is no valid result.
pub trait CheckedPow<RHS> {
    type Output;

    fn checked_pow(self, exp: RHS) -> Option<Self::Output>;
}

/// Saturating addition. Computes `self + rhs`, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingAdd<RHS = Self> {
    type Output;

    fn saturating_add(self, rhs: RHS) -> Self::Output;
}

/// Saturating subtraction. Computes `self - rhs`, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingSub<RHS = Self> {
    type Output;

    fn saturating_sub(self, rhs: RHS) -> Self::Output;
}

/// Saturating multiplication. Computes `self * rhs`, saturating at the numeric bounds instead of
/// overflowing.
pub trait SaturatingMul<RHS = Self> {
    type Output;

    fn saturating_mul(self, rhs: RHS) -> Self::Output;
}

/// Raises `self` to the power of `exp`, saturating at the numeric bounds instead of overflowing.
pub trait SaturatingPow<RHS> {
    type Output;

    fn saturating_pow(self, exp: RHS) -> Self::Output;
}

/// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at the boundary of the type.
pub trait WrappingAdd<RHS = Self> {
    type Output;

    fn wrapping_add(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around at the boundary of the
/// type.
pub trait WrappingSub<RHS = Self> {
    type Output;

    fn wrapping_sub(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) multiplication. Computes `self * rhs`, wrapping around at the boundary of the
/// type.
pub trait WrappingMul<RHS = Self> {
    type Output;

    fn wrapping_mul(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) division. Computes `self / rhs`, wrapping around at the boundary of the type.
pub trait WrappingDiv<RHS = Self> {
    type Output;

    fn wrapping_div(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) remainder. Computes `self % rhs`, wrapping around at the boundary of the
/// type.
pub trait WrappingRem<RHS = Self> {
    type Output;

    fn wrapping_rem(self, rhs: RHS) -> Self::Output;
}

/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary of the type.
pub trait WrappingNeg {
    type Output;

    fn wrapping_neg(self) -> Self::Output;
}

/// Wrapping (modular) absolute value. Computes `self.abs()`, wrapping around at the boundary of the
/// type.
pub trait WrappingAbs {
    type Output;

    fn abs(self) -> Self::Output;
}

/// Wrapping (modular) exponentiation. Raises `self` to the power of `exp`, wrapping around at the
/// boundary of the type.
pub trait WrappingPow<RHS> {
    type Output;

    fn wrapping_pow(self, exp: RHS) -> Self::Output;
}

/// Calculates `self` + `rhs`.
///
/// Returns a tuple of the addition along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingAdd<RHS = Self> {
    type Output;

    fn overflowing_add(self, rhs: RHS) -> (Self::Output, bool);
}

/// Calculates `self` - `rhs`.
///
/// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic
/// overflow would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingSub<RHS = Self> {
    type Output;

    fn overflowing_sub(self, rhs: RHS) -> (Self::Output, bool);
}

/// Calculates `self` * `rhs`.
///
/// Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic
/// overflow would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingMul<RHS = Self> {
    type Output;

    fn overflowing_mul(self, rhs: RHS) -> (Self::Output, bool);
}

/// Calculates `self` / `rhs`.
///
/// Returns a tuple of the quotient along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingDiv<RHS = Self> {
    type Output;

    fn overflowing_div(self, rhs: RHS) -> (Self::Output, bool);
}

/// Calculates `self` % `rhs`.
///
/// Returns a tuple of the remainder along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingRem<RHS = Self> {
    type Output;

    fn overflowing_rem(self, rhs: RHS) -> (Self::Output, bool);
}

/// Calculates -`self`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingNeg {
    type Output;

    fn overflowing_neg(self) -> (Self::Output, bool);
}

/// Calculates `self.abs()`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingAbs {
    type Output;

    fn abs(self) -> (Self::Output, bool);
}

/// Calculates `self.pow(exp)`.
///
/// Returns a tuple of the result along with a boolean indicating whether an arithmetic overflow
/// would occur. If an overflow would have occurred then the wrapped value is returned.
pub trait OverflowingPow<RHS> {
    type Output;

    fn overflowing_pow(self, exp: RHS) -> (Self::Output, bool);
}

/// Raises `self` to the power of `exp`.
pub trait Pow<RHS> {
    type Output;

    fn pow(self, exp: RHS) -> Self::Output;
}

/// Computes the absolute value of `self`.
pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

/// Returns `true` iff `self == 2^k` for some integer `k`.
pub trait IsPowerOfTwo {
    fn is_power_of_two(self) -> bool;
}

/// Returns the smallest power of two greater than or equal to `self`.
pub trait NextPowerOfTwo {
    type Output;

    fn next_power_of_two(self) -> Self::Output;
}

/// Returns the smallest power of two greater than or equal to `self`. If the next power of two is
/// greater than the type's maximum value, `None` is returned; otherwise the power of two is wrapped
/// in `Some`.
pub trait CheckedNextPowerOfTwo {
    type Output;

    fn checked_next_power_of_two(self) -> Option<Self::Output>;
}

// Nontrivial traits start here

pub trait NextPowerOfTwoAssign {
    fn next_power_of_two_assign(&mut self);
}

pub trait EqModPowerOfTwo<Rhs = Self> {
    fn eq_mod_power_of_two(self, other: Rhs, pow: u64) -> bool;
}

pub trait EqMod<Rhs = Self, Mod = Self> {
    fn eq_mod(self, other: Rhs, modulus: Mod) -> bool;
}

pub trait WrappingNegAssign {
    fn wrapping_neg_assign(&mut self);
}

pub trait Parity {
    fn even(self) -> bool;

    fn odd(self) -> bool;
}

pub trait WrappingAddAssign<RHS = Self> {
    fn wrapping_add_assign(&mut self, rhs: RHS);
}

pub trait WrappingSubAssign<RHS = Self> {
    fn wrapping_sub_assign(&mut self, rhs: RHS);
}

pub trait WrappingDivAssign<RHS = Self> {
    fn wrapping_div_assign(&mut self, rhs: RHS);
}

pub trait WrappingMulAssign<RHS = Self> {
    fn wrapping_mul_assign(&mut self, rhs: RHS);
}

pub trait WrappingRemAssign<RHS = Self> {
    fn wrapping_rem_assign(&mut self, rhs: RHS);
}

pub trait SaturatingAddAssign<RHS = Self> {
    fn saturating_add_assign(&mut self, rhs: RHS);
}

pub trait SaturatingMulAssign<RHS = Self> {
    fn saturating_mul_assign(&mut self, rhs: RHS);
}

pub trait SaturatingSubAssign<RHS = Self> {
    fn saturating_sub_assign(&mut self, rhs: RHS);
}

pub trait OverflowingAddAssign<RHS = Self> {
    fn overflowing_add_assign(&mut self, rhs: RHS) -> bool;
}

pub trait OverflowingSubAssign<RHS = Self> {
    fn overflowing_sub_assign(&mut self, rhs: RHS) -> bool;
}

pub trait OverflowingDivAssign<RHS = Self> {
    fn overflowing_div_assign(&mut self, rhs: RHS) -> bool;
}

pub trait OverflowingMulAssign<RHS = Self> {
    fn overflowing_mul_assign(&mut self, rhs: RHS) -> bool;
}

pub trait OverflowingRemAssign<RHS = Self> {
    fn overflowing_rem_assign(&mut self, rhs: RHS) -> bool;
}

pub trait OverflowingNegAssign {
    fn overflowing_neg_assign(&mut self) -> bool;
}

pub trait DivisibleByPowerOfTwo {
    fn divisible_by_power_of_two(self, pow: u64) -> bool;
}

pub trait ModPowerOfTwo {
    type Output;

    fn mod_power_of_two(self, other: u64) -> Self::Output;
}

pub trait ModPowerOfTwoAssign {
    fn mod_power_of_two_assign(&mut self, other: u64);
}

pub trait NegModPowerOfTwo {
    type Output;

    fn neg_mod_power_of_two(self, other: u64) -> Self::Output;
}

pub trait NegModPowerOfTwoAssign {
    fn neg_mod_power_of_two_assign(&mut self, other: u64);
}

pub trait RemPowerOfTwo {
    type Output;

    fn rem_power_of_two(self, other: u64) -> Self::Output;
}

pub trait RemPowerOfTwoAssign {
    fn rem_power_of_two_assign(&mut self, other: u64);
}

pub trait CeilingModPowerOfTwo {
    type Output;

    fn ceiling_mod_power_of_two(self, other: u64) -> Self::Output;
}

pub trait CeilingModPowerOfTwoAssign {
    fn ceiling_mod_power_of_two_assign(&mut self, other: u64);
}

pub trait DivMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn div_mod(self, rhs: RHS) -> (Self::DivOutput, Self::ModOutput);
}

pub trait DivAssignMod<RHS = Self> {
    type ModOutput;

    fn div_assign_mod(&mut self, rhs: RHS) -> Self::ModOutput;
}

pub trait DivRem<RHS = Self> {
    type DivOutput;
    type RemOutput;

    fn div_rem(self, rhs: RHS) -> (Self::DivOutput, Self::RemOutput);
}

pub trait DivAssignRem<RHS = Self> {
    type RemOutput;

    fn div_assign_rem(&mut self, rhs: RHS) -> Self::RemOutput;
}

pub trait Mod<RHS = Self> {
    type Output;

    // would be called `mod`, but that's a keyword
    fn mod_op(self, rhs: RHS) -> Self::Output;
}

pub trait ModAssign<RHS = Self> {
    fn mod_assign(&mut self, rhs: RHS);
}

pub trait NegMod<RHS = Self> {
    type Output;

    fn neg_mod(self, rhs: RHS) -> Self::Output;
}

pub trait NegModAssign<RHS = Self> {
    fn neg_mod_assign(&mut self, rhs: RHS);
}

pub trait CeilingMod<RHS = Self> {
    type Output;

    fn ceiling_mod(self, rhs: RHS) -> Self::Output;
}

pub trait CeilingModAssign<RHS = Self> {
    fn ceiling_mod_assign(&mut self, rhs: RHS);
}

pub trait DivRound<RHS = Self> {
    type Output;

    fn div_round(self, rhs: RHS, rm: RoundingMode) -> Self::Output;
}

pub trait DivRoundAssign<RHS = Self> {
    fn div_round_assign(&mut self, rhs: RHS, rm: RoundingMode);
}

pub trait CeilingDivNegMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn ceiling_div_neg_mod(self, rhs: RHS) -> (Self::DivOutput, Self::ModOutput);
}

pub trait CeilingDivAssignNegMod<RHS = Self> {
    type ModOutput;

    fn ceiling_div_assign_neg_mod(&mut self, rhs: RHS) -> Self::ModOutput;
}

pub trait CeilingDivMod<RHS = Self> {
    type DivOutput;
    type ModOutput;

    fn ceiling_div_mod(self, rhs: RHS) -> (Self::DivOutput, Self::ModOutput);
}

pub trait CeilingDivAssignMod<RHS = Self> {
    type ModOutput;

    fn ceiling_div_assign_mod(&mut self, rhs: RHS) -> Self::ModOutput;
}

pub trait DivExact<RHS = Self> {
    type Output;

    fn div_exact(self, rhs: RHS) -> Self::Output;
}

pub trait DivExactAssign<RHS = Self> {
    fn div_exact_assign(&mut self, rhs: RHS);
}

pub trait DivisibleBy<RHS = Self> {
    fn divisible_by(self, rhs: RHS) -> bool;
}

pub trait Sign {
    fn sign(&self) -> Ordering;
}

pub trait AbsAssign {
    fn abs_assign(&mut self);
}

pub trait NegAssign {
    fn neg_assign(&mut self);
}

pub trait AddMulAssign<B, C> {
    // Equivalent to self += b * c
    fn add_mul_assign(&mut self, b: B, c: C);
}

pub trait AddMul<B, C> {
    type Output;

    // Equivalent to self + b * c
    fn add_mul(self, b: B, c: C) -> Self::Output;
}

pub trait SubMulAssign<B, C> {
    // Equivalent to self -= b * c
    fn sub_mul_assign(&mut self, b: B, c: C);
}

pub trait SubMul<B, C> {
    type Output;

    // Equivalent to self - b * c
    fn sub_mul(self, b: B, c: C) -> Self::Output;
}

pub trait CheckedSubMul<B, C> {
    type Output;

    fn checked_sub_mul(self, b: B, c: C) -> Option<Self::Output>;
}

pub trait SaturatingSubMulAssign<B, C> {
    fn saturating_sub_mul_assign(&mut self, b: B, c: C);
}

pub trait SaturatingSubMul<B, C> {
    type Output;

    fn saturating_sub_mul(self, b: B, c: C) -> Self::Output;
}

/// Provides a function to get the floor of the base-2 logarithm of `self`.
pub trait FloorLogTwo {
    /// floor(log<sub>2</sub>(`self`))
    fn floor_log_two(self) -> u64;
}

/// Provides a function to get the ceiling of the base-2 logarithm of `self`.
pub trait CeilingLogTwo {
    /// ceiling(log<sub>2</sub>(`self`))
    fn ceiling_log_two(self) -> u64;
}

/// Computes the absolute value of `self` and converts to the unsigned equivalent.
pub trait UnsignedAbs {
    type Output;

    fn unsigned_abs(self) -> Self::Output;
}

pub trait ShlRound<RHS> {
    type Output;

    fn shl_round(self, rhs: RHS, rm: RoundingMode) -> Self::Output;
}

pub trait ShrRound<RHS> {
    type Output;

    fn shr_round(self, rhs: RHS, rm: RoundingMode) -> Self::Output;
}

pub trait ShlRoundAssign<Rhs = Self> {
    fn shl_round_assign(&mut self, rhs: Rhs, rm: RoundingMode);
}

pub trait ShrRoundAssign<Rhs = Self> {
    fn shr_round_assign(&mut self, rhs: Rhs, rm: RoundingMode);
}

/// Checked shift left. Computes `self << rhs`, returning `None` if there is no valid result.
pub trait TrueCheckedShl {
    type Output;

    fn true_checked_shl(self, rhs: u32) -> Option<Self::Output>;
}

/// Checked shift right. Computes `self >> rhs`, returning `None` if there is no valid result.
pub trait TrueCheckedShr {
    type Output;

    fn true_checked_shr(self, rhs: u32) -> Option<Self::Output>;
}
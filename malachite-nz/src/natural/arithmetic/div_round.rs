use std::cmp::Ordering;

use malachite_base::crement::Crementable;
use malachite_base::num::arithmetic::traits::{
    DivAssignMod, DivMod, DivRound, DivRoundAssign, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::round::RoundingMode;

use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs of a `Limb` divided by the `Natural` and rounded according to a specified
/// `RoundingMode`. The limb slice must have at least two elements and cannot have any trailing
/// zeros.
///
/// This function returns a `None` iff the rounding mode is `Exact` but the remainder of the
/// division would be nonzero.
///
/// Note that this function may only return `None`, `Some(0)`, or `Some(1)` because of the
/// restrictions placed on the input slice.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::round::RoundingMode;
/// use malachite_nz::natural::arithmetic::div_round::limbs_limb_div_round_limbs;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Down), Some(0));
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Floor), Some(0));
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Up), Some(1));
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Ceiling), Some(1));
/// assert_eq!(limbs_limb_div_round_limbs(0, &[123, 456], RoundingMode::Exact), Some(0));
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Exact), None);
/// assert_eq!(limbs_limb_div_round_limbs(789, &[123, 456], RoundingMode::Nearest), Some(0));
/// assert_eq!(limbs_limb_div_round_limbs(0xffff_ffff, &[123, 1], RoundingMode::Nearest),
///     Some(1));
/// assert_eq!(limbs_limb_div_round_limbs(0xffff_ffff, &[0xffff_ffff, 1],
///     RoundingMode::Nearest), Some(0));
///
/// assert_eq!(limbs_limb_div_round_limbs(0xffff_ffff, &[0xffff_fffe, 1],
///     RoundingMode::Nearest), Some(0));
///
/// assert_eq!(limbs_limb_div_round_limbs(0xffff_ffff, &[0xffff_fffd, 1],
///     RoundingMode::Nearest), Some(1));
/// ```
pub fn limbs_limb_div_round_limbs(limb: Limb, limbs: &[Limb], rm: RoundingMode) -> Option<Limb> {
    if limb == 0 {
        Some(0)
    } else {
        match rm {
            RoundingMode::Down | RoundingMode::Floor => Some(0),
            RoundingMode::Up | RoundingMode::Ceiling => Some(1),
            RoundingMode::Exact => None,
            // 1 if 2 * limb > Natural::from_limbs_asc(limbs); otherwise, 0
            RoundingMode::Nearest => Some(
                if limbs.len() == 2
                    && limbs[1] == 1
                    && limb.get_highest_bit()
                    && (limb << 1) > limbs[0]
                {
                    1
                } else {
                    0
                },
            ),
        }
    }
}

fn div_round_nearest(quotient: Natural, remainder: Natural, denominator: &Natural) -> Natural {
    let compare = (remainder << 1u64).cmp(denominator);
    if compare == Ordering::Greater || compare == Ordering::Equal && quotient.odd() {
        quotient.add_limb(1)
    } else {
        quotient
    }
}

fn div_round_assign_nearest(quotient: &mut Natural, remainder: Natural, denominator: &Natural) {
    let compare = (remainder << 1u64).cmp(denominator);
    if compare == Ordering::Greater || compare == Ordering::Equal && quotient.odd() {
        quotient.increment();
    }
}

impl DivRound<Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural` and rounds according to a specified rounding mode, taking
    /// both `Natural`s by value. See the `RoundingMode` documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Down).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::trillion().div_round(Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Up).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::trillion()
    ///         .div_round(Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///     "333333333334");
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32)
    ///         .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32)
    ///         .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "4"
    /// );
    /// ```
    #[inline]
    fn div_round(mut self, other: Natural, rm: RoundingMode) -> Natural {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural` and rounds according to a specified rounding mode, taking
    /// the first `Natural` by value and the second by reference. See the `RoundingMode`
    /// documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(&Natural::from(4u32), RoundingMode::Down).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::trillion()
    ///         .div_round(&Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(4u32), RoundingMode::Up).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::trillion()
    ///         .div_round(&Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///     "333333333334");
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(&Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32)
    ///         .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32)
    ///         .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "4"
    /// );
    /// ```
    #[inline]
    fn div_round(mut self, other: &'a Natural, rm: RoundingMode) -> Natural {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural` and rounds according to a specified rounding mode, taking
    /// the first `Natural` by reference and the second by value. See the `RoundingMode`
    /// documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(Natural::from(4u32), RoundingMode::Down).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::trillion())
    ///         .div_round(Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(Natural::from(4u32), RoundingMode::Up).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::trillion())
    ///         .div_round(Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///     "333333333334");
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32))
    ///         .div_round(Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32))
    ///         .div_round(Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "4"
    /// );
    /// ```
    fn div_round(self, other: Natural, rm: RoundingMode) -> Natural {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            self / other
        } else {
            let (q, r) = self.div_mod(&other);
            if r == 0 as Limb {
                q
            } else {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => q.add_limb(1),
                    RoundingMode::Exact => panic!("Division is not exact"),
                    RoundingMode::Nearest => div_round_nearest(q, r, &other),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl<'a, 'b> DivRound<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural` and rounds according to a specified rounding mode, taking
    /// both `Natural`s by reference. See the `RoundingMode` documentation for details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRound;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(&Natural::from(4u32), RoundingMode::Down).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::trillion())
    ///         .div_round(&Natural::from(3u32), RoundingMode::Floor).to_string(),
    ///     "333333333333"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(&Natural::from(4u32), RoundingMode::Up).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::trillion())
    ///         .div_round(&Natural::from(3u32), RoundingMode::Ceiling).to_string(),
    ///     "333333333334");
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(&Natural::from(5u32), RoundingMode::Exact).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32))
    ///         .div_round(&Natural::from(3u32), RoundingMode::Nearest).to_string(),
    ///     "7"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "2"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32))
    ///         .div_round(&Natural::from(4u32), RoundingMode::Nearest).to_string(),
    ///     "4"
    /// );
    /// ```
    fn div_round(self, other: &'b Natural, rm: RoundingMode) -> Natural {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            self / other
        } else {
            let (q, r) = self.div_mod(other);
            if r == 0 as Limb {
                q
            } else {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => q.add_limb(1),
                    RoundingMode::Exact => panic!("Division is not exact: {} / {}", self, other),
                    RoundingMode::Nearest => div_round_nearest(q, r, other),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl DivRoundAssign<Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place and rounds according to a specified rounding
    /// mode, taking the `Natural` on the RHS by value. See the `RoundingMode` documentation for
    /// details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Down);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::trillion();
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Floor);
    /// assert_eq!(n.to_string(), "333333333333");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Up);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Natural::trillion();
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Ceiling);
    /// assert_eq!(n.to_string(), "333333333334");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(5u32), RoundingMode::Exact);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Natural::from(20u32);
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "7");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::from(14u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "4");
    /// ```
    fn div_round_assign(&mut self, other: Natural, rm: RoundingMode) {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            *self /= other;
        } else {
            let r = self.div_assign_mod(&other);
            if r != 0 as Limb {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => {
                        self.increment();
                    }
                    RoundingMode::Exact => panic!("Division is not exact"),
                    RoundingMode::Nearest => div_round_assign_nearest(self, r, &other),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl<'a> DivRoundAssign<&'a Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place and rounds according to a specified rounding
    /// mode, taking the `Natural` on the RHS by reference. See the `RoundingMode` documentation for
    /// details.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRoundAssign;
    /// use malachite_base::round::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Down);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::trillion();
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Floor);
    /// assert_eq!(n.to_string(), "333333333333");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Up);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Natural::trillion();
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Ceiling);
    /// assert_eq!(n.to_string(), "333333333334");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(5u32), RoundingMode::Exact);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "3");
    ///
    /// let mut n = Natural::from(20u32);
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "7");
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "2");
    ///
    /// let mut n = Natural::from(14u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n.to_string(), "4");
    /// ```
    fn div_round_assign(&mut self, other: &'a Natural, rm: RoundingMode) {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            *self /= other;
        } else {
            let r = self.div_assign_mod(other);
            if r != 0 as Limb {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => {
                        self.increment();
                    }
                    RoundingMode::Exact => panic!("Division is not exact"),
                    RoundingMode::Nearest => div_round_assign_nearest(self, r, other),
                    _ => unreachable!(),
                }
            }
        }
    }
}
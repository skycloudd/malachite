use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::traits::Zero;

use natural::Natural;

impl ModIsReduced for Natural {
    /// Returns whether `self` is reduced mod `modulus`; in other words whether it is less than
    /// `modulus`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `modulus` is 0.
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModIsReduced;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_is_reduced(&Natural::from(5u32)), true);
    /// assert_eq!(Natural::trillion().mod_is_reduced(&Natural::trillion()), false);
    /// assert_eq!(Natural::trillion().mod_is_reduced(&(Natural::trillion() + Natural::ONE)), true);
    /// ```
    #[inline]
    fn mod_is_reduced(&self, modulus: &Natural) -> bool {
        assert_ne!(*modulus, Natural::ZERO);
        self < modulus
    }
}
use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns the smallest number of bits necessary to represent `self`. 0 has zero significant
    /// bits.
    ///
    /// # Example
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(0).significant_bits(), 0);
    /// assert_eq!(Natural::from(100).significant_bits(), 7);
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().significant_bits(), 40);
    /// ```
    pub fn significant_bits(&self) -> u64 {
        match *self {
            Small(x) => (32 - x.leading_zeros()) as u64,
            Large(x) => (unsafe { gmp::mpz_sizeinbase(&x, 2) }) as u64,
        }
    }
}
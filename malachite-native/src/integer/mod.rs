use natural::Natural;

/// An integer.
///
/// Any `Integer` whose absolute value is small enough to fit into an `u32` is represented inline.
/// Only integers outside this range incur the costs of heap-allocation.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Integer {
    sign: bool, // must be true if abs is zero
    abs: Natural,
}

impl Integer {
    /// Creates a new `Integer` equal to 0.
    ///
    /// # Example
    /// ```
    /// use malachite_native::integer::Integer;
    ///
    /// assert_eq!(Integer::new().to_string(), "0");
    /// ```
    pub fn new() -> Integer {
        Integer {
            sign: true,
            abs: Natural::new(),
        }
    }

    /// Returns true iff `self` is valid. To be valid, can only be Large when its absolute value
    /// is at least 2^(31). All Integers used outside this crate are valid, but temporary Integers
    /// used inside may not be.
    pub fn is_valid(&self) -> bool {
        self.abs.is_valid() && (self.sign || self.abs != 0)
    }
}

/// Creates a default `Integer` equal to 0.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!(Integer::default().to_string(), "0");
/// ```
impl Default for Integer {
    fn default() -> Integer {
        Integer {
            sign: true,
            abs: Natural::new(),
        }
    }
}

pub mod arithmetic {
    pub mod abs;
    pub mod neg;
}
pub mod comparison {
    pub mod ord_integer;
    pub mod partial_eq_i32;
    pub mod partial_eq_natural;
    pub mod partial_eq_u32;
    pub mod partial_ord_i32;
    pub mod partial_ord_u32;
    pub mod sign;
}
pub mod conversion;
pub mod logic;
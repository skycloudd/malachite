use num::random::random_unsigneds_less_than;
use random::random_values_from_slice::RandomValuesFromSlice;
use random::random_values_from_vec::RandomValuesFromVec;
use random::seed::Seed;

/// This seed was generated by random.org.
pub const EXAMPLE_SEED: Seed = Seed::from_bytes([
    0xbf, 0x18, 0x11, 0xce, 0x15, 0xee, 0xfd, 0x20, 0x2f, 0xdf, 0x67, 0x6a, 0x6b, 0xba, 0xaf, 0x04,
    0xff, 0x71, 0xe0, 0xf8, 0x0b, 0x2a, 0xcf, 0x27, 0x85, 0xb3, 0x32, 0xc6, 0x20, 0x80, 0x5e, 0x36,
]);

/// Uniformly generates a random reference to a value from a nonempty slice. The iterator cannot
/// outlive the slice. It may be more convenient for the iterator to own the data, in which case you
/// may use `random_values_from_vec` instead.
///
/// Length is infinite.
///
/// Time per iteration: worst case O(1)
///
/// Additional memory per iteration: worst case O(1)
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_base::random::{EXAMPLE_SEED, random_values_from_slice};
///
/// let xs = &[2, 3, 5, 7, 11];
/// assert_eq!(
///     random_values_from_slice(EXAMPLE_SEED, xs).cloned().take(10).collect::<Vec<_>>(),
///     &[3, 7, 3, 5, 11, 3, 5, 11, 2, 2]
/// );
/// ```
#[inline]
pub fn random_values_from_slice<T>(seed: Seed, xs: &[T]) -> RandomValuesFromSlice<T> {
    if xs.is_empty() {
        panic!("empty slice");
    }
    RandomValuesFromSlice {
        xs,
        indices: random_unsigneds_less_than(seed, xs.len()),
    }
}

/// Uniformly generates a random value from a nonempty `Vec`. The iterator owns the data. It may be
/// more convenient for the iterator to return references to a pre-existing slice, in which case you
/// may  use `random_values_from_slice` instead.
///
/// Length is infinite.
///
/// Time per iteration: worst case O(1)
///
/// Additional memory per iteration: worst case O(1)
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Examples
/// ```
/// use malachite_base::random::{EXAMPLE_SEED, random_values_from_vec};
///
/// let xs = vec![2, 3, 5, 7, 11];
/// assert_eq!(
///     random_values_from_vec(EXAMPLE_SEED, xs).take(10).collect::<Vec<_>>(),
///     &[3, 7, 3, 5, 11, 3, 5, 11, 2, 2]
/// );
/// ```
#[inline]
pub fn random_values_from_vec<T: Clone>(seed: Seed, xs: Vec<T>) -> RandomValuesFromVec<T> {
    if xs.is_empty() {
        panic!("empty Vec");
    }
    let indices = random_unsigneds_less_than(seed, xs.len());
    RandomValuesFromVec { xs, indices }
}

pub mod random_values_from_slice;
pub mod random_values_from_vec;
pub mod seed;

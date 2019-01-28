use malachite_base::num::Assign;
use natural::Natural::{self, Small};
use platform::Limb;

/// Assigns a `Limb` to a `Natural`.
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
/// use malachite_base::num::Assign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(123u32);
///     x.assign(456u32);
///     assert_eq!(x.to_string(), "456");
/// }
/// ```
impl Assign<Limb> for Natural {
    fn assign(&mut self, other: Limb) {
        *self = Small(other);
    }
}
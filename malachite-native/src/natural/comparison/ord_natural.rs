use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;

/// Compares `self` to a `Natural`.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert!(Natural::from(123) > Natural::from(122));
/// assert!(Natural::from(123) >= Natural::from(122));
/// assert!(Natural::from(123) < Natural::from(124));
/// assert!(Natural::from(123) <= Natural::from(124));
/// assert!(Natural::from_str("1000000000000").unwrap() > Natural::from(123));
/// assert!(Natural::from_str("1000000000000").unwrap() >= Natural::from(123));
/// ```
impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Asserts that `Natural` ordering is a total order.
impl Ord for Natural {
    fn cmp(&self, other: &Natural) -> Ordering {
        match (self, other) {
            (&Small(ref x), &Small(ref y)) => x.cmp(y),
            (&Small(_), &Large(_)) => Ordering::Less,
            (&Large(_), &Small(_)) => Ordering::Greater,
            (&Large(ref xs), &Large(ref ys)) => {
                let len_compare = xs.len().cmp(&ys.len());
                if len_compare != Ordering::Equal {
                    return len_compare;
                }
                for (x, y) in xs.into_iter().zip(ys.into_iter()).rev() {
                    let limb_compare = x.cmp(y);
                    if limb_compare != Ordering::Equal {
                        return limb_compare;
                    }
                }
                Ordering::Equal
            }
        }
    }
}
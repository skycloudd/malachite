use num::basic::traits::Zero;

/// Generates nonzero values from an iterator.
#[derive(Clone, Debug)]
pub struct NonzeroValues<I: Iterator>(I)
where
    I::Item: Eq + Zero;

impl<I: Iterator> Iterator for NonzeroValues<I>
where
    I::Item: Eq + Zero,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        loop {
            let x = self.0.next();
            if x != Some(I::Item::ZERO) {
                return x;
            }
        }
    }
}

/// Generates nonzero values from an iterator. This iterator will hang if given an iterator that
/// produces an infinite suffix of zeros.
///
/// The following applies to any filtering iterator if "nonzero" is replaced by the filtering
/// predicate.
///
/// Length is the number of nonzero values produced by `xs`.
///
/// Let I(j) be the number of nonzero values in the first j values generated by `xs`.
/// Let T(j) be the worst case time for the jth iteration of `xs.next()`.
/// Let TT(j) = integral from 0 to j of T(x) dx.
/// Let M(j) be the worst case additional memory for the jth iteration of `xs.next()`.
/// Let J(i) be the smallest solution j of P(j) = i. (If there is no solution, the iterator hangs
///     before reaching index i).
///
/// Time for the `i`th iteration: worst case O(TT(J(i)) - TT(J(i - 1)))
///
/// Additional memory for the `i`th iteration: worst case O(M(J(i)))
///
/// For example, suppose `xs` takes O(j ^ 2) time and O(j) memory to produce its jth element, and
/// as j goes to infinity, the number of nonzero values in the first j elements produced by `xs`
/// approaches log(j). Then I(j) = log(j), T(j) = O(j ^ 2), M(j) = O(1), and J(i) = exp(i).
///
/// So in this case, the worst-case additional memory of `nonzero_values(xs)` is O(exp(i)) and the
/// worst-case time is O(exp(3 * i - 3)).
///
/// # Examples
/// ```
/// use malachite_base::iterators::nonzero_values;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_primitive_integers;
///
/// assert_eq!(
///     nonzero_values(random_primitive_integers::<u8>(EXAMPLE_SEED)).take(10)
///         .collect::<Vec<_>>(),
///     &[113, 239, 69, 108, 228, 210, 168, 161, 87, 32]
/// )
/// ```
#[inline]
pub fn nonzero_values<I: Iterator>(xs: I) -> NonzeroValues<I>
where
    I::Item: Eq + Zero,
{
    NonzeroValues(xs)
}

pub mod comparison;

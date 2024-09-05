// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::random::geometric::SimpleRational;
use crate::num::random::{
    random_unsigneds_less_than, RandomUnsignedsLessThan, VariableRangeGenerator,
};
use crate::random::Seed;
use rand::Rng;
use rand_chacha::ChaCha20Rng;

/// Uniformly generates random [`bool`]s.
///
/// This `struct` is created by [`random_bools`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomBools {
    rng: ChaCha20Rng,
    x: u32,
    bits_left: u8,
}

impl Iterator for RandomBools {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        if self.bits_left == 0 {
            self.x = self.rng.gen();
            self.bits_left = 31;
        } else {
            self.x >>= 1;
            self.bits_left -= 1;
        }
        Some(self.x.odd())
    }
}

/// Uniformly generates random [`bool`]s.
///
/// $P(\text{false}) = P(\text{true}) = \frac{1}{2}$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::bools::random::random_bools;
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_bools(EXAMPLE_SEED), 10),
///     "[true, false, false, false, true, true, true, false, true, true, ...]"
/// )
/// ```
///
/// # Notes
/// The resulting iterator uses every random bit generated by the PRNG, unlike some implementations
/// which only use one bit out of 32 or 64.
#[inline]
pub fn random_bools(seed: Seed) -> RandomBools {
    RandomBools {
        rng: seed.get_rng(),
        x: 0,
        bits_left: 0,
    }
}

/// Generates random [`bool`]s, with a fixed probability of generating `true`.
///
/// This `struct` is created by [`weighted_random_bools`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct WeightedRandomBools {
    numerator: u64,
    xs: RandomUnsignedsLessThan<u64>,
}

impl Iterator for WeightedRandomBools {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        Some(self.xs.next().unwrap() < self.numerator)
    }
}

/// Generates random [`bool`]s, with a fixed probability of generating `true`.
///
/// Let $n_p$ be `p_numerator`, $d_p$ be `p_denominator`, and let $p=n_p/d_p$. Then
///
/// $P(\text{true}) = p$,
///
/// $P(\text{false}) = 1-p$.
///
/// The output length is infinite.
///
/// # Panics
/// Panics if `p_denominator` is 0 or `p_numerator > p_denominator`.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `p_denominator.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::bools::random::weighted_random_bools;
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(weighted_random_bools(EXAMPLE_SEED, 3, 4), 10),
///     "[true, true, false, true, false, false, true, false, true, true, ...]"
/// )
/// ```
pub fn weighted_random_bools(
    seed: Seed,
    p_numerator: u64,
    p_denominator: u64,
) -> WeightedRandomBools {
    assert!(p_numerator <= p_denominator);
    let p = SimpleRational::new(p_numerator, p_denominator);
    WeightedRandomBools {
        numerator: p.n,
        xs: random_unsigneds_less_than(seed, p.d),
    }
}

/// Generates a random [`bool`] with a particular probability of being `true`.
///
/// Let $n_p$ be `p_numerator`, $d_p$ be `p_denominator`, and let $p=n_p/d_p$. Then
///
/// $P(\text{true}) = p$,
///
/// $P(\text{false}) = 1-p$.
///
/// # Panics
/// Panics if `p_denominator` is 0 or `p_numerator > p_denominator`.
///
/// # Expected complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `p_denominator.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::bools::random::get_weighted_random_bool;
/// use malachite_base::num::random::VariableRangeGenerator;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     get_weighted_random_bool(&mut VariableRangeGenerator::new(EXAMPLE_SEED), 1, 10),
///     false
/// );
/// ```
pub fn get_weighted_random_bool(
    range_generator: &mut VariableRangeGenerator,
    p_numerator: u64,
    p_denominator: u64,
) -> bool {
    assert_ne!(p_denominator, 0);
    assert!(p_numerator <= p_denominator);
    if p_numerator == 0 {
        return false;
    } else if p_numerator == p_denominator {
        return true;
    }
    let p = SimpleRational::new(p_numerator, p_denominator);
    range_generator.next_less_than(p.d) < p.n
}

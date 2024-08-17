// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::primes::prime_indicator_sequence;

#[test]
pub fn test_prime_indicator_sequence() {
    let s: String = prime_indicator_sequence()
        .take(1000)
        .map(|b| if b { '1' } else { '0' })
        .collect();
    assert_eq!(
        s,
        "01101010001010001010001000001010000010001010001000001000001010000010001010000010001000001\
        000000010001010001010001000000000000010001000001010000000001010000010000010001000001000001\
        010000000001010001010000000000010000000000010001010001000001010000000001000001000001000001\
        010000010001010000000001000000000000010001010001000000000000010000010000000001010001000001\
        000000010000010000010001000001000000010001000000010000000001010000000001010000010001000001\
        000000010001010001000000000001000000010001000000010001000001000000000001010000000000000000\
        010000010000000001000001000001010000010000000001000001000001010000010000010001010000000000\
        010000000001010001000001000001010000000000010001000001000000010000000001000000010000000001\
        000000010000010000010001000000010000010001000000010001000000000000010000000001000000000001\
        010000000001010001010000000001000000000000010001010001000000000000010001010001000000000000\
        000000010001000000010000000001000000010001000001000001000000000000010001000001000001000000\
        01000001000"
    );
}

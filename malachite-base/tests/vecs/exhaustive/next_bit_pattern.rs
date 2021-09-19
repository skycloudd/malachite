use itertools::repeat_n;
use malachite_base::vecs::exhaustive::next_bit_pattern;

fn pattern_to_string(pattern: &[bool]) -> String {
    let mut s = String::with_capacity(pattern.len());
    for &b in pattern.iter().rev() {
        s.push(if b { '1' } else { '0' });
    }
    s
}

fn next_bit_pattern_helper(
    width: usize,
    min_bits: usize,
    max_bits: usize,
    expected_patterns: &[&'static str],
) {
    assert!(min_bits <= max_bits);
    assert_ne!(max_bits, 0);
    assert!(width >= min_bits);
    let mut pattern: Vec<bool> = repeat_n(false, width).collect();
    for b in &mut pattern[..min_bits] {
        *b = true;
    }
    let mut bit_count = min_bits;
    let mut patterns = Vec::new();
    while pattern.len() == width {
        assert_eq!(pattern.iter().filter(|&&b| b).count(), bit_count);
        assert!(bit_count >= min_bits);
        assert!(bit_count <= max_bits);
        patterns.push(pattern_to_string(&pattern));
        next_bit_pattern(&mut pattern, &mut bit_count, min_bits, max_bits);
    }
    //TODO length is sum of binomial coefficients
    assert_eq!(patterns, expected_patterns);
}

#[test]
fn test_next_bit_pattern() {
    next_bit_pattern_helper(5, 1, 1, &["00001", "00010", "00100", "01000", "10000"]);
    next_bit_pattern_helper(
        5,
        2,
        2,
        &["00011", "00101", "00110", "01001", "01010", "01100", "10001", "10010", "10100", "11000"],
    );
    next_bit_pattern_helper(
        5,
        3,
        3,
        &["00111", "01011", "01101", "01110", "10011", "10101", "10110", "11001", "11010", "11100"],
    );
    next_bit_pattern_helper(5, 4, 4, &["01111", "10111", "11011", "11101", "11110"]);
    next_bit_pattern_helper(5, 5, 5, &["11111"]);

    next_bit_pattern_helper(
        5,
        0,
        1,
        &["00000", "00001", "00010", "00100", "01000", "10000"],
    );
    next_bit_pattern_helper(
        5,
        1,
        2,
        &[
            "00001", "00010", "00011", "00100", "00101", "00110", "01000", "01001", "01010",
            "01100", "10000", "10001", "10010", "10100", "11000",
        ],
    );
    next_bit_pattern_helper(
        5,
        2,
        3,
        &[
            "00011", "00101", "00110", "00111", "01001", "01010", "01011", "01100", "01101",
            "01110", "10001", "10010", "10011", "10100", "10101", "10110", "11000", "11001",
            "11010", "11100",
        ],
    );
    next_bit_pattern_helper(
        5,
        3,
        4,
        &[
            "00111", "01011", "01101", "01110", "01111", "10011", "10101", "10110", "10111",
            "11001", "11010", "11011", "11100", "11101", "11110",
        ],
    );
    next_bit_pattern_helper(
        5,
        4,
        5,
        &["01111", "10111", "11011", "11101", "11110", "11111"],
    );

    next_bit_pattern_helper(
        5,
        0,
        2,
        &[
            "00000", "00001", "00010", "00011", "00100", "00101", "00110", "01000", "01001",
            "01010", "01100", "10000", "10001", "10010", "10100", "11000",
        ],
    );
    next_bit_pattern_helper(
        5,
        1,
        3,
        &[
            "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001",
            "01010", "01011", "01100", "01101", "01110", "10000", "10001", "10010", "10011",
            "10100", "10101", "10110", "11000", "11001", "11010", "11100",
        ],
    );
    next_bit_pattern_helper(
        5,
        2,
        4,
        &[
            "00011", "00101", "00110", "00111", "01001", "01010", "01011", "01100", "01101",
            "01110", "01111", "10001", "10010", "10011", "10100", "10101", "10110", "10111",
            "11000", "11001", "11010", "11011", "11100", "11101", "11110",
        ],
    );
    next_bit_pattern_helper(
        5,
        3,
        5,
        &[
            "00111", "01011", "01101", "01110", "01111", "10011", "10101", "10110", "10111",
            "11001", "11010", "11011", "11100", "11101", "11110", "11111",
        ],
    );

    next_bit_pattern_helper(
        5,
        0,
        3,
        &[
            "00000", "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000",
            "01001", "01010", "01011", "01100", "01101", "01110", "10000", "10001", "10010",
            "10011", "10100", "10101", "10110", "11000", "11001", "11010", "11100",
        ],
    );
    next_bit_pattern_helper(
        5,
        1,
        4,
        &[
            "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001",
            "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001", "10010",
            "10011", "10100", "10101", "10110", "10111", "11000", "11001", "11010", "11011",
            "11100", "11101", "11110",
        ],
    );
    next_bit_pattern_helper(
        5,
        2,
        5,
        &[
            "00011", "00101", "00110", "00111", "01001", "01010", "01011", "01100", "01101",
            "01110", "01111", "10001", "10010", "10011", "10100", "10101", "10110", "10111",
            "11000", "11001", "11010", "11011", "11100", "11101", "11110", "11111",
        ],
    );

    next_bit_pattern_helper(
        5,
        0,
        4,
        &[
            "00000", "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000",
            "01001", "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001",
            "10010", "10011", "10100", "10101", "10110", "10111", "11000", "11001", "11010",
            "11011", "11100", "11101", "11110",
        ],
    );
    next_bit_pattern_helper(
        5,
        1,
        5,
        &[
            "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000", "01001",
            "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001", "10010",
            "10011", "10100", "10101", "10110", "10111", "11000", "11001", "11010", "11011",
            "11100", "11101", "11110", "11111",
        ],
    );

    next_bit_pattern_helper(
        5,
        0,
        5,
        &[
            "00000", "00001", "00010", "00011", "00100", "00101", "00110", "00111", "01000",
            "01001", "01010", "01011", "01100", "01101", "01110", "01111", "10000", "10001",
            "10010", "10011", "10100", "10101", "10110", "10111", "11000", "11001", "11010",
            "11011", "11100", "11101", "11110", "11111",
        ],
    );
}

use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::random::striped::striped_random_natural_signeds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;

fn striped_random_natural_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    m_numerator: u64,
    m_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (T, Option<T>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = striped_random_natural_signeds::<T>(EXAMPLE_SEED, m_numerator, m_denominator);
    let actual_values = xs
        .clone()
        .map(|x| x.to_binary_string())
        .take(20)
        .collect::<Vec<_>>();
    let actual_common_values = common_values_map(1_000_000, 10, xs.clone())
        .iter()
        .map(|(x, frequency)| (x.to_binary_string(), *frequency))
        .collect::<Vec<_>>();
    let actual_sample_median = median(xs.clone().take(1_000_000));
    let actual_sample_moment_stats = moment_stats(xs.take(1_000_000));
    assert_eq!(
        (
            actual_values,
            actual_common_values,
            actual_sample_median,
            actual_sample_moment_stats
        ),
        (
            expected_values
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            expected_common_values
                .iter()
                .map(|(x, frequency)| (x.to_string(), *frequency))
                .collect::<Vec<_>>(),
            expected_sample_median,
            expected_sample_moment_stats
        )
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_striped_random_natural_signeds() {
    // i8, m = 4
    let values = &[
        "0", "101100", "110000", "1111100", "1111", "1111110", "0", "111", "11101", "1100000",
        "1111111", "1100000", "0", "10", "1000011", "111111", "1", "0", "1111", "1",
    ];
    let common_values = &[
        ("0", 89042),
        ("1111111", 88624),
        ("11111", 29871),
        ("1111000", 29848),
        ("1000000", 29802),
        ("1111110", 29796),
        ("1100000", 29664),
        ("1110000", 29649),
        ("111", 29644),
        ("111111", 29621),
    ];
    let sample_median = (64, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(63.526448000001764),
        standard_deviation: NiceFloat(47.66137677522695),
        skewness: NiceFloat(-0.0011056592983105758),
        excess_kurtosis: NiceFloat(-1.5649370173869874),
    };
    striped_random_natural_signeds_helper::<i8>(
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i8, m = 2
    let values = &[
        "11001", "11100", "101010", "1001011", "101101", "1111100", "111100", "10100", "1101",
        "1111111", "1100011", "1101101", "1100", "100", "1100001", "100011", "110100", "110101",
        "110100", "10011",
    ];
    let common_values = &[
        ("11010", 8131),
        ("1111", 8059),
        ("1010101", 8004),
        ("1001101", 7998),
        ("11011", 7993),
        ("1110111", 7978),
        ("1010100", 7959),
        ("111", 7958),
        ("1011010", 7953),
        ("1100011", 7947),
    ];
    let sample_median = (64, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(63.555225000001755),
        standard_deviation: NiceFloat(36.938359582441294),
        skewness: NiceFloat(-0.0007106807748730345),
        excess_kurtosis: NiceFloat(-1.2008391146615376),
    };
    striped_random_natural_signeds_helper::<i8>(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i8, m = 5/4
    let values = &[
        "101001", "100101", "100110", "1101001", "101101", "1010101", "111010", "101010", "10110",
        "1010101", "1001000", "1001011", "1000", "11100", "1111010", "101101", "110101", "101010",
        "100101", "100010",
    ];
    let common_values = &[
        ("101010", 131212),
        ("1010101", 131202),
        ("1001010", 33119),
        ("1011010", 33073),
        ("10101", 32947),
        ("100101", 32868),
        ("1010010", 32851),
        ("101011", 32817),
        ("101001", 32765),
        ("110101", 32761),
    ];
    let sample_median = (65, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(63.51375499999882),
        standard_deviation: NiceFloat(27.10334306669828),
        skewness: NiceFloat(-0.0017292127163026868),
        excess_kurtosis: NiceFloat(-1.1007498380278757),
    };
    striped_random_natural_signeds_helper::<i8>(
        5,
        4,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i64, m = 32
    let values = &[
        "11111111111111111111111111111",
        "111111111111111111111111111111111111111111111111111111111",
        "1111111111111111111111111111",
        "111111111111111111111111110000000000000000000000000000000000000",
        "111111111111111111111111111111111111111111111111111111111",
        "111111111000000000000000000000000000000000000001100000000111111",
        "11111111111111100000",
        "11111111111111111",
        "111111111111",
        "111111111111111111111111111111111111111111111111111100000000000",
        "111111111111100000000000000000000000000000000000000000000000000",
        "111111111000000000000000000000000000000000000000000000000000000",
        "1111111111111111111111111111100000000000000",
        "1000000011111111111111111111",
        "111111000000000000000111111111111111111111111111111111111111111",
        "0",
        "11111111111100000000000000000000000000000000000000",
        "111111111111111111111110000000000000000001111111",
        "1111111111111111111111111111111111111111111111111111111111111",
        "1111111",
    ];
    let common_values = &[
        (
            "111111111111111111111111111111111111111111111111111111111111111",
            69948,
        ),
        ("0", 69809),
        (
            "111111111111111111111111111111111111111111111111111111111111000",
            2383,
        ),
        (
            "11111111111111111111111111111111111111111111111111111111111",
            2362,
        ),
        ("11111111111111111111111", 2334),
        (
            "111111111111111100000000000000000000000000000000000000000000000",
            2334,
        ),
        (
            "100000000000000000000000000000000000000000000000000000000000000",
            2328,
        ),
        (
            "111111111111111111111111111111100000000000000000000000000000000",
            2327,
        ),
        ("111111111", 2320),
        ("11", 2318),
    ];
    let sample_median = (4611686018427387904, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.614976015729421e18),
        standard_deviation: NiceFloat(4.4270184647985137e18),
        skewness: NiceFloat(-0.0014267894129673844),
        excess_kurtosis: NiceFloat(-1.9430528242638783),
    };
    striped_random_natural_signeds_helper::<i64>(
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i64, m = 2
    let values = &[
        "1100110001101010100101101001000001100001110101111001000000001",
        "111011011100111110111000010111001101001101010010111011001100",
        "11110000010100111000110001101100111001001010101110001000100000",
        "110010001100111101011100111111100001001110001100001001000000011",
        "1100110011001011000001001111001010100010110100011010010001",
        "111000101110010000110100101010100100111001100100001101010011011",
        "10000011110001100010001010101101101100100000010011101111010",
        "1110110110110011010011011010000111001101110110010001101011110",
        "11010010000001101100111000010011100101110010101101001100110000",
        "110000010001000110001011100111001101110010001111000110001111001",
        "100011101111011001001101101011100000110001110100111011011011111",
        "111101111100000110000001010001001101011110011110110100010110010",
        "1010001010011101001011011111100101110000001010101000111100001",
        "11110011011100110101011110010001110100010111001010000100011101",
        "110001001100111101011111000100111101011110111101110011010100111",
        "1111011101101101011111011010011011001011010001101011100101",
        "11111101110101010010000100011110100110100000110100101000110111",
        "10101011010100000101011100111011000001101010001000101111111010",
        "110000111110001111000001110011101110100001101011111010100110",
        "1101010111111111000111001111000111110001111110100101000001111",
    ];
    let common_values = &[
        ("10101000110110101100110011001101011011101", 1),
        ("1110100010100111110111100000011111000010100", 1),
        ("10011111100110010100000010001100001001111011", 1),
        ("10111110110011101010110111100010100101101100", 1),
        ("110101001100100110010011010000011100100111011", 1),
        ("1001000111011010110011001111101001111101110011", 1),
        ("1010101100111011110111011011011100011101010101", 1),
        ("1100110000110110000100011110000110101010110010", 1),
        ("1100110011000110101101111111111110111011101001", 1),
        ("1101011001000111111110011010000001001001000110", 1),
    ];
    let sample_median = (4616064909372870741, Some(4616067528870766644));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.614689648935556e18),
        standard_deviation: NiceFloat(2.6622250799886193e18),
        skewness: NiceFloat(-0.0006757801147527129),
        excess_kurtosis: NiceFloat(-1.199752325803772),
    };
    striped_random_natural_signeds_helper::<i64>(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i64, m = 33/32
    let values = &[
        "10101010101010101010101101010101010101010101010101010101010101",
        "10101010101010100010101010010101101010101010101011010101011010",
        "10101010010101011010101010101010101101010101010101010101010101",
        "101010101010101010101101010101010101010101010101010101010101010",
        "10101010101010101010101011010101010101001010010101001010101010",
        "101010101010101010101010101010101010100101010010101010110101010",
        "10101010101010101010101010101010101010101010101010110101010101",
        "10101010101010101010010101010101010101010101010101010101010101",
        "10101010010101010101010101010101010101010101010101010101010101",
        "101010101100101010101010101010101010101010101010101010010101010",
        "101010101010101010101010101010101011001010101010101101010101010",
        "101101010101010011010101010101010101010101010101010101010101010",
        "10101001010101010101011010101010101010101010101010101010101010",
        "10101010101010101011010010010110101010101010101010101010101010",
        "101010101010101010110101010101010101010101010010101010101010101",
        "10101010101010101010101010101010101010101010101010101010101010",
        "10101010101010101010101010101010101010101010101011010101010101",
        "10101010101010101010101010101010101010101101010101010101010101",
        "10101010101010101011011010100101010101001010101010101010101010",
        "10101010101010101010101010101011010101010101010101010101010101",
    ];
    let common_values = &[
        (
            "101010101010101010101010101010101010101010101010101010101010101",
            74299,
        ),
        (
            "10101010101010101010101010101010101010101010101010101010101010",
            74072,
        ),
        (
            "101010101010101010101010010101010101010101010101010101010101010",
            2429,
        ),
        (
            "101010101010101010101010101010101010101010101010101010101011010",
            2419,
        ),
        (
            "101010101011010101010101010101010101010101010101010101010101010",
            2419,
        ),
        (
            "101010101010101010101001010101010101010101010101010101010101010",
            2411,
        ),
        (
            "101010101010101010101010101010101010101010101010010101010101010",
            2393,
        ),
        (
            "101010101001010101010101010101010101010101010101010101010101010",
            2390,
        ),
        (
            "10101010101010101010101010101001010101010101010101010101010101",
            2389,
        ),
        (
            "10101010101010101010101010101010101010101010101010101010100101",
            2389,
        ),
    ];
    let sample_median = (5281221163029801642, Some(5281221163029804373));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.613154278246113e18),
        standard_deviation: NiceFloat(1.599140542162029e18),
        skewness: NiceFloat(-0.0019511120341977268),
        excess_kurtosis: NiceFloat(-1.7372862317601716),
    };
    striped_random_natural_signeds_helper::<i64>(
        33,
        32,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

macro_rules! striped_random_natural_signeds_fail {
    (
        $t:ident,
        $striped_random_natural_signeds_fail_1:ident,
        $striped_random_natural_signeds_fail_2:ident
    ) => {
        #[test]
        #[should_panic]
        fn $striped_random_natural_signeds_fail_1() {
            striped_random_natural_signeds::<$t>(EXAMPLE_SEED, 1, 0);
        }

        #[test]
        #[should_panic]
        fn $striped_random_natural_signeds_fail_2() {
            striped_random_natural_signeds::<$t>(EXAMPLE_SEED, 2, 3);
        }
    };
}

striped_random_natural_signeds_fail!(
    i8,
    striped_random_natural_signeds_u8_fail_1,
    striped_random_natural_signeds_u8_fail_2
);
striped_random_natural_signeds_fail!(
    i16,
    striped_random_natural_signeds_u16_fail_1,
    striped_random_natural_signeds_u16_fail_2
);
striped_random_natural_signeds_fail!(
    i32,
    striped_random_natural_signeds_u32_fail_1,
    striped_random_natural_signeds_u32_fail_2
);
striped_random_natural_signeds_fail!(
    i64,
    striped_random_natural_signeds_u64_fail_1,
    striped_random_natural_signeds_u64_fail_2
);
striped_random_natural_signeds_fail!(
    i128,
    striped_random_natural_signeds_u128_fail_1,
    striped_random_natural_signeds_u128_fail_2
);
striped_random_natural_signeds_fail!(
    isize,
    striped_random_natural_signeds_usize_fail_1,
    striped_random_natural_signeds_usize_fail_2
);

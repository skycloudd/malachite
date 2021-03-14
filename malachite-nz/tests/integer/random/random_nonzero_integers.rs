use itertools::Itertools;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::integer::random::random_nonzero_integers;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::integer::IntegerCheckedToF64Wrapper;

fn random_nonzero_integers_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: Option<MomentStats>,
) {
    let xs = random_nonzero_integers(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator);
    let actual_values = xs
        .clone()
        .map(|x| Integer::to_string(&x))
        .take(20)
        .collect_vec();
    let actual_values = actual_values.iter().map(String::as_str).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone())
        .into_iter()
        .map(|(x, freq)| (x.to_string(), freq))
        .collect_vec();
    let actual_common_values = actual_common_values
        .iter()
        .map(|(x, freq)| (x.as_str(), *freq))
        .collect_vec();
    let (median_lo, median_hi) = median(xs.clone().take(1000000));
    let (median_lo, median_hi) = (
        median_lo.to_string(),
        median_hi.map(|x| Integer::to_string(&x)),
    );
    let actual_sample_median = (median_lo.as_str(), median_hi.as_deref());
    // Note that the population moments do not exist.
    let actual_sample_moment_stats = if expected_sample_moment_stats.is_some() {
        Some(moment_stats(
            xs.take(1000000).map(IntegerCheckedToF64Wrapper),
        ))
    } else {
        None
    };
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            actual_sample_median,
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_common_values,
            expected_sample_median,
            expected_sample_moment_stats
        )
    );
}

#[test]
fn test_random_nonzero_integers() {
    // mean bits = 65/64
    let values = &[
        "1", "1", "1", "-1", "-1", "-1", "1", "-1", "-1", "1", "1", "1", "-1", "-1", "-1", "-1",
        "1", "1", "-1", "-1",
    ];
    let common_values = &[
        ("1", 492842),
        ("-1", 491818),
        ("2", 3836),
        ("3", 3803),
        ("-2", 3744),
        ("-3", 3718),
        ("6", 39),
        ("5", 33),
        ("-6", 32),
        ("-5", 31),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.001516999999999985),
        standard_deviation: NiceFloat(1.0444624404988267),
        skewness: NiceFloat(-0.00031392947248860435),
        excess_kurtosis: NiceFloat(-1.2267327517337354),
    };
    random_nonzero_integers_helper(
        65,
        64,
        values,
        common_values,
        sample_median,
        Some(sample_moment_stats),
    );
    // mean bits = 2
    let values = &[
        "6", "1", "6", "-14", "-1", "-1", "1", "-1", "-3", "6", "5", "7", "-1", "-1", "-2", "-14",
        "1", "5", "-14", "-1",
    ];
    let common_values = &[
        ("1", 249934),
        ("-1", 249480),
        ("3", 62818),
        ("-3", 62545),
        ("2", 62282),
        ("-2", 62281),
        ("-7", 15874),
        ("7", 15794),
        ("6", 15750),
        ("-5", 15696),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.677845999999954),
        standard_deviation: NiceFloat(1282.3370760759833),
        skewness: NiceFloat(271.563399149848),
        excess_kurtosis: NiceFloat(320730.53239180415),
    };
    random_nonzero_integers_helper(
        2,
        1,
        values,
        common_values,
        sample_median,
        Some(sample_moment_stats),
    );
    // mean bits = 32
    let values = &[
        "6",
        "373973144",
        "46887963477285686350042496363292819122",
        "-93254818",
        "-126908",
        "-4471675267836600",
        "1860142159",
        "-118004986915853475",
        "-98",
        "346513",
        "18250435",
        "511570",
        "-7230971744056",
        "-28344",
        "-1006",
        "-1",
        "45",
        "53968471397952150",
        "-1",
        "-5655261",
    ];
    let common_values = &[
        ("1", 15709),
        ("-1", 15677),
        ("-3", 7735),
        ("3", 7574),
        ("-2", 7514),
        ("2", 7484),
        ("-7", 3755),
        ("7", 3713),
        ("4", 3712),
        ("5", 3676),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.1307602130166568e111),
        standard_deviation: NiceFloat(2.1307597326341758e114),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_nonzero_integers_helper(
        32,
        1,
        values,
        common_values,
        sample_median,
        Some(sample_moment_stats),
    );
    // mean bits = 64
    let values = &[
        "121396406",
        "28091526743610625648357683081915367485486380400402584",
        "674",
        "-503972648740684226424764",
        "-261225431937620249973589071",
        "-1744034",
        "1",
        "-96963",
        "-33563594322",
        "1085240",
        "221961187000",
        "104563169774",
        "-799255527",
        "-32987221556354134413",
        "-8799240150400653965518",
        "-940783",
        "954",
        "16689620",
        "-392696864519",
        "-31641",
    ];
    let common_values = &[
        ("1", 7878),
        ("-1", 7830),
        ("-3", 3920),
        ("-2", 3913),
        ("3", 3882),
        ("2", 3809),
        ("-7", 1974),
        ("4", 1950),
        ("-6", 1939),
        ("-4", 1892),
    ];
    let sample_median = ("1", None);
    random_nonzero_integers_helper(64, 1, values, common_values, sample_median, None);
}

#[test]
#[should_panic]
fn random_nonzero_integers_fail_1() {
    random_nonzero_integers(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_nonzero_integers_fail_2() {
    random_nonzero_integers(EXAMPLE_SEED, 2, 3);
}

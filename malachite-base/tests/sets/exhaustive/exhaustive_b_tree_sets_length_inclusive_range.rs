use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::sets::exhaustive::exhaustive_b_tree_sets_length_inclusive_range;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn exhaustive_b_tree_sets_length_inclusive_range_small_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Ord,
{
    let xss = exhaustive_b_tree_sets_length_inclusive_range(a, b, xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_exhaustive_b_tree_sets_length_inclusive_range() {
    exhaustive_b_tree_sets_length_inclusive_range_small_helper(0, 4, nevers(), 1, &[btreeset! {}]);
    exhaustive_b_tree_sets_length_inclusive_range_small_helper(6, 9, nevers(), 0, &[]);
    exhaustive_b_tree_sets_length_inclusive_range_small_helper(
        0,
        4,
        exhaustive_units(),
        2,
        &[btreeset! {}, btreeset! {()}],
    );
    exhaustive_b_tree_sets_length_inclusive_range_small_helper(1, 0, exhaustive_bools(), 0, &[]);
    exhaustive_b_tree_sets_length_inclusive_range_small_helper(
        0,
        1,
        exhaustive_bools(),
        3,
        &[btreeset! {}, btreeset! {false}, btreeset! {true}],
    );
    exhaustive_b_tree_sets_length_inclusive_range_small_helper(
        2,
        3,
        exhaustive_bools(),
        1,
        &[btreeset! {false, true}],
    );
    exhaustive_b_tree_sets_length_inclusive_range_small_helper(
        1,
        1,
        'a'..='c',
        3,
        &[btreeset! {'a'}, btreeset! {'b'}, btreeset! {'c'}],
    );
}

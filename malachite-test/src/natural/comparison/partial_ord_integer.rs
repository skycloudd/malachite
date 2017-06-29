use common::{gmp_integer_to_native, gmp_integer_to_rugint, gmp_natural_to_native,
             gmp_natural_to_rugint_integer};
use malachite_native as native;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::{max, Ordering};

pub fn demo_exhaustive_natural_partial_cmp_integer(limit: usize) {
    for (x, y) in exhaustive_pairs(exhaustive_naturals(), exhaustive_integers()).take(limit) {
        match x.partial_cmp(&y).unwrap() {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn demo_random_natural_partial_cmp_integer(limit: usize) {
    for (x, y) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_naturals(seed, 32)),
                               &(|seed| random_integers(seed, 32)))
                .take(limit) {
        match x.partial_cmp(&y).unwrap() {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn benchmark_exhaustive_natural_partial_cmp_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.partial_cmp(&Integer)");
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_pairs(exhaustive_naturals(), exhaustive_integers()),
                    function_f: &(|(x, y)| x.partial_cmp(&y)),
                    function_g: &(|(x, y): (native::natural::Natural,
                                            native::integer::Integer)| x.partial_cmp(&y)),
                    function_h: &(|(x, y): (rugint::Integer, rugint::Integer)| x.partial_cmp(&y)),
                    x_to_y: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_integer_to_native(y))
                              }),
                    x_to_z: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x), gmp_integer_to_rugint(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "Natural.partial\\\\_cmp(&Integer)",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_partial_cmp_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.partial_cmp(&Integer)");
    benchmark_3(BenchmarkOptions3 {
                    xs: random_pairs(&EXAMPLE_SEED,
                                     &(|seed| random_naturals(seed, scale)),
                                     &(|seed| random_integers(seed, scale))),
                    function_f: &(|(x, y)| x.partial_cmp(&y)),
                    function_g: &(|(x, y): (native::natural::Natural,
                                            native::integer::Integer)| x.partial_cmp(&y)),
                    function_h: &(|(x, y): (rugint::Integer, rugint::Integer)| x.partial_cmp(&y)),
                    x_to_y: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_integer_to_native(y))
                              }),
                    x_to_z: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x), gmp_integer_to_rugint(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "Natural.partial\\\\_cmp(&Integer)",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
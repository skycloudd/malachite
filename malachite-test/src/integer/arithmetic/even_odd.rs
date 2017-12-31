use common::{gmp_integer_to_native, GenerationMode};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

type It = Iterator<Item = gmp::Integer>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_integers())
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_integers(&EXAMPLE_SEED, scale))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_is_even(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        if n.is_even() {
            println!("{} is even", n);
        } else {
            println!("{} is not even", n);
        }
    }
}

pub fn demo_integer_is_odd(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        if n.is_odd() {
            println!("{} is odd", n);
        } else {
            println!("{} is not odd", n);
        }
    }
}

pub fn benchmark_integer_is_even(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.is_even()", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: gmp::Integer| n.is_even()),
        function_g: &(|n: native::Integer| n.is_even()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.is\\\\_even()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_is_odd(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.is_odd()", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: gmp::Integer| n.is_odd()),
        function_g: &(|n: native::Integer| n.is_odd()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.is\\\\_odd()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

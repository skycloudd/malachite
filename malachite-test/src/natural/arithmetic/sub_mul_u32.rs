use common::{gmp_natural_to_native, GenerationMode};
use malachite_base::traits::{SubMul, SubMulAssign};
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_triples, random_triples};
use std::cmp::max;

type It1 = Iterator<Item = (gmp::Natural, gmp::Natural, u32)>;

pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(exhaustive_inputs_2().filter(|&(ref a, ref b, c)| a >= &(b * c)))
}

pub fn random_inputs_1(scale: u32) -> Box<It1> {
    Box::new(random_inputs_2(scale).filter(|&(ref a, ref b, c)| a >= &(b * c)))
}

pub fn select_inputs_1(gm: GenerationMode) -> Box<It1> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_1(),
        GenerationMode::Random(scale) => random_inputs_1(scale),
    }
}

type It2 = Iterator<Item = (gmp::Natural, gmp::Natural, u32)>;

pub fn exhaustive_inputs_2() -> Box<It2> {
    Box::new(exhaustive_triples(
        exhaustive_naturals(),
        exhaustive_naturals(),
        exhaustive_u(),
    ))
}

pub fn random_inputs_2(scale: u32) -> Box<It2> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs_2(gm: GenerationMode) -> Box<It2> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_2(),
        GenerationMode::Random(scale) => random_inputs_2(scale),
    }
}

pub fn demo_natural_sub_mul_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in select_inputs_1(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_natural_sub_mul_u32(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs_2(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {:?}", a_old, b, c, a.sub_mul(&b, c));
    }
}

pub fn demo_natural_sub_mul_u32_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs_2(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, {}) = {:?}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

pub fn benchmark_natural_sub_mul_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.sub_mul_assign(&Natural, u32)",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_1(gm),
        function_f: &(|(mut a, b, c): (gmp::Natural, gmp::Natural, u32)| a.sub_mul_assign(&b, c)),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
            a.sub_mul_assign(&b, c)
        }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_assign_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.sub_mul_assign(&Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_1(gm),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| {
            a.sub_mul_assign(&b, c)
        }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, u32)| a -= &(&b * c)),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, u32)",
        g_name: "Natural -= \\\\&Natural * u32",
        title: "Natural.sub\\\\_mul\\\\_assign(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.sub_mul(&Natural, u32)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(a, b, c): (gmp::Natural, gmp::Natural, u32)| a.sub_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.sub_mul(&b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.sub\\\\_mul(\\\\&Natural, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.sub_mul(&Natural, u32) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.sub_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| (&a).sub_mul(&b, c)),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.sub\\\\_mul(\\\\&Natural, u32)",
        g_name: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, u32)",
        title: "Natural.sub\\\\_mul(\\\\&Natural, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.sub_mul(&Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| a.sub_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| a - &(&b * c)),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.sub\\\\_mul(\\\\&Natural, u32)",
        g_name: "Natural - \\\\&Natural * u32",
        title: "Natural.sub\\\\_mul(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_mul_u32_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} (&Natural).sub_mul(&Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, u32)| (&a).sub_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, u32)| &a - &(&b * c)),
        x_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_natural_to_native(a), gmp_natural_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, u32)",
        g_name: "(\\\\&Natural) - \\\\&Natural * u32",
        title: "(\\\\&Natural).sub\\\\_mul(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

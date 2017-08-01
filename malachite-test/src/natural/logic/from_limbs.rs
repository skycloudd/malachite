use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::vecs::{exhaustive_vecs, random_vecs};

pub fn demo_exhaustive_natural_from_limbs_le(limit: usize) {
    for xs in exhaustive_vecs(exhaustive_u::<u32>()).take(limit) {
        println!("from_limbs_le({:?}) = {:?}",
                 xs,
                 gmp::Natural::from_limbs_le(xs.as_slice()));
    }
}

pub fn demo_random_natural_from_limbs_le(limit: usize) {
    for xs in random_vecs(&EXAMPLE_SEED, 4, &(|seed| random_x::<u32>(seed))).take(limit) {
        println!("from_limbs_le({:?}) = {:?}",
                 xs,
                 gmp::Natural::from_limbs_le(xs.as_slice()));
    }
}

pub fn demo_exhaustive_natural_from_limbs_be(limit: usize) {
    for xs in exhaustive_vecs(exhaustive_u::<u32>()).take(limit) {
        println!("from_limbs_be({:?}) = {:?}",
                 xs,
                 gmp::Natural::from_limbs_be(xs.as_slice()));
    }
}

pub fn demo_random_natural_from_limbs_be(limit: usize) {
    for xs in random_vecs(&EXAMPLE_SEED, 4, &(|seed| random_x::<u32>(seed))).take(limit) {
        println!("from_limbs_be({:?}) = {:?}",
                 xs,
                 gmp::Natural::from_limbs_be(xs.as_slice()));
    }
}

pub fn benchmark_exhaustive_natural_from_limbs_le(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural::from_limbs_le(&[u32])");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_vecs(exhaustive_u::<u32>()),
                    function_f: &(|xs: Vec<u32>| gmp::Natural::from_limbs_le(xs.as_slice())),
                    function_g: &(|xs: Vec<u32>| native::Natural::from_limbs_le(xs.as_slice())),
                    x_cons: &(|xs| xs.clone()),
                    y_cons: &(|xs| xs.clone()),
                    x_param: &(|xs| xs.len()),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Natural::from\\\\_limbs\\\\_le(\\\\&[u32])",
                    x_axis_label: "xs.len()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_from_limbs_le(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural::from_limbs_le(&[u32])");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_vecs(&EXAMPLE_SEED, scale, &(|seed| random_x::<u32>(seed))),
                    function_f: &(|xs: Vec<u32>| gmp::Natural::from_limbs_le(xs.as_slice())),
                    function_g: &(|xs: Vec<u32>| native::Natural::from_limbs_le(xs.as_slice())),
                    x_cons: &(|xs| xs.clone()),
                    y_cons: &(|xs| xs.clone()),
                    x_param: &(|xs| xs.len()),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Natural::from\\\\_limbs\\\\_le(\\\\&[u32])",
                    x_axis_label: "xs.len()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_natural_from_limbs_be(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural::from_limbs_be(&[u32])");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_vecs(exhaustive_u::<u32>()),
                    function_f: &(|xs: Vec<u32>| gmp::Natural::from_limbs_le(xs.as_slice())),
                    function_g: &(|xs: Vec<u32>| native::Natural::from_limbs_be(xs.as_slice())),
                    x_cons: &(|xs| xs.clone()),
                    y_cons: &(|xs| xs.clone()),
                    x_param: &(|xs| xs.len()),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Natural::from\\\\_limbs\\\\_be(\\\\&[u32])",
                    x_axis_label: "xs.len()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_from_limbs_be(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural::from_limbs_be(&[u32])");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_vecs(&EXAMPLE_SEED, scale, &(|seed| random_x::<u32>(seed))),
                    function_f: &(|xs: Vec<u32>| gmp::Natural::from_limbs_le(xs.as_slice())),
                    function_g: &(|xs: Vec<u32>| native::Natural::from_limbs_be(xs.as_slice())),
                    x_cons: &(|xs| xs.clone()),
                    y_cons: &(|xs| xs.clone()),
                    x_param: &(|xs| xs.len()),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Natural::from\\\\_limbs\\\\_be(\\\\&[u32])",
                    x_axis_label: "xs.len()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

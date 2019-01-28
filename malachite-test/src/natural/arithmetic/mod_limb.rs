use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_and_positive_unsigned_var_1;
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_unsigned, rm_pairs_of_natural_and_positive_unsigned,
};
use inputs::natural::{
    pairs_of_natural_and_positive_unsigned, pairs_of_unsigned_and_positive_natural,
};
use malachite_base::num::{
    CeilingDivNegMod, DivMod, Mod, ModAssign, NegMod, NegModAssign, SignificantBits,
};
use malachite_nz::natural::arithmetic::mod_limb::limbs_mod_limb;
use malachite_nz::platform::Limb;
use num::{BigUint, ToPrimitive};
use rug::{self, ops::RemRounding};

// For `Natural`s, `mod` is equivalent to `rem`.

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mod_limb);
    register_demo!(registry, demo_natural_rem_assign_limb);
    register_demo!(registry, demo_natural_rem_limb);
    register_demo!(registry, demo_natural_rem_limb_ref);
    register_demo!(registry, demo_natural_mod_assign_limb);
    register_demo!(registry, demo_natural_mod_limb);
    register_demo!(registry, demo_natural_mod_limb_ref);
    register_demo!(registry, demo_natural_neg_mod_assign_limb);
    register_demo!(registry, demo_natural_neg_mod_limb);
    register_demo!(registry, demo_natural_neg_mod_limb_ref);
    register_demo!(registry, demo_limb_rem_natural);
    register_demo!(registry, demo_limb_rem_natural_ref);
    register_demo!(registry, demo_limb_rem_assign_natural);
    register_demo!(registry, demo_limb_rem_assign_natural_ref);
    register_demo!(registry, demo_limb_mod_natural);
    register_demo!(registry, demo_limb_mod_natural_ref);
    register_demo!(registry, demo_limb_mod_assign_natural);
    register_demo!(registry, demo_limb_mod_assign_natural_ref);
    register_demo!(registry, demo_limb_neg_mod_natural);
    register_demo!(registry, demo_limb_neg_mod_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_mod_limb);
    register_bench!(registry, Large, benchmark_natural_rem_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_rem_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_rem_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_rem_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_neg_mod_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_neg_mod_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_rem_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_rem_assign_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_mod_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_mod_assign_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_neg_mod_natural_evaluation_strategy
    );
}

pub fn num_rem_u32(x: BigUint, u: u32) -> u32 {
    (x % u).to_u32().unwrap()
}

pub fn rug_neg_mod_u32(x: rug::Integer, u: u32) -> u32 {
    (-x.rem_ceil(u)).to_u32_wrapping()
}

fn demo_limbs_mod_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_mod_limb({:?}, {}) = {}",
            limbs,
            limb,
            limbs_mod_limb(&limbs, limb)
        );
    }
}

fn demo_natural_rem_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n %= u;
        println!("x := {}; x %= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_rem_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", n_old, u, n % u);
    }
}

fn demo_natural_rem_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("&{} % {} = {}", n, u, &n % u);
    }
}

fn demo_natural_mod_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n.mod_assign(u);
        println!("x := {}; x.mod_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_natural_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {}", n_old, u, n.mod_op(u));
    }
}

fn demo_natural_mod_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).mod({}) = {}", n, u, (&n).mod_op(u));
    }
}

fn demo_natural_neg_mod_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n.neg_mod_assign(u);
        println!("x := {}; x.neg_mod_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_natural_neg_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod({}) = {}", n_old, u, n.neg_mod(u));
    }
}

fn demo_natural_neg_mod_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).neg_mod({}) = {}", n, u, (&n).neg_mod(u));
    }
}

fn demo_limb_rem_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", u, n_old, u % n);
    }
}

fn demo_limb_rem_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % &{} = {}", u, n_old, u % &n);
    }
}

fn demo_limb_rem_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u %= n;
        println!("x := {}; x %= {}; x = {}", u_old, n_old, u);
    }
}

fn demo_limb_rem_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        u %= &n;
        println!("x := {}; x %= &{}; x = {}", u_old, n, u);
    }
}

fn demo_limb_mod_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {:?}", u, n_old, u.mod_op(n));
    }
}

fn demo_limb_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod(&{}) = {:?}", u, n_old, u.mod_op(&n));
    }
}

fn demo_limb_mod_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u.mod_assign(n);
        println!("x := {}; x.mod_assign({}); x = {}", u_old, n_old, u);
    }
}

fn demo_limb_mod_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        u.mod_assign(&n);
        println!("x := {}; x.mod_assign(&{}); x = {}", u_old, n, u);
    }
}

fn demo_limb_neg_mod_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod({}) = {:?}", u, n_old, u.neg_mod(n));
    }
}

fn demo_limb_neg_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod(&{}) = {:?}", u, n_old, u.neg_mod(&n));
    }
}

fn benchmark_limbs_mod_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_mod_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_natural_rem_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural %= Limb",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x %= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_rem_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural % Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x % y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_rem_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_natural_rem_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural % Limb",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x % y))),
            ("naive", &mut (|(x, y)| no_out!(x._mod_limb_naive(y)))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).1))),
        ],
    );
}

fn benchmark_natural_rem_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural % Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural % Limb", &mut (|(x, y)| no_out!(x % y))),
            ("&Natural % Limb", &mut (|(x, y)| no_out!(&x % y))),
        ],
    );
}

fn benchmark_natural_mod_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_assign(Limb)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.mod_assign(y)))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.mod_op(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_rem_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_natural_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural.mod(Limb)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            (
                "(&Natural).mod(Limb)",
                &mut (|(x, y)| no_out!((&x).mod_op(y))),
            ),
        ],
    );
}

fn benchmark_natural_neg_mod_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.neg_mod_assign(Limb)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.neg_mod_assign(y)))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_neg_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod(Limb)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.neg_mod(y)))),
            ("rug", &mut (|((x, y), _)| no_out!(rug_neg_mod_u32(x, y)))),
        ],
    );
}

fn benchmark_natural_neg_mod_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.neg_mod(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.neg_mod(y)))),
            (
                "using ceiling_div_neg_mod",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y).1)),
            ),
        ],
    );
}

fn benchmark_natural_neg_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.neg_mod(Limb)",
                &mut (|(x, y)| no_out!(x.neg_mod(y))),
            ),
            (
                "(&Natural).neg_mod(Limb)",
                &mut (|(x, y)| no_out!((&x).neg_mod(y))),
            ),
        ],
    );
}

fn benchmark_limb_rem_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb % Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Limb % Natural", &mut (|(x, y)| no_out!(x % y))),
            ("Limb % &Natural", &mut (|(x, y)| no_out!(x % &y))),
        ],
    );
}

fn benchmark_limb_rem_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb %= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Limb %= Natural", &mut (|(mut x, y)| x %= y)),
            ("Limb %= &Natural", &mut (|(mut x, y)| x %= &y)),
        ],
    );
}

fn benchmark_limb_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Limb.mod(Natural)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("Limb.mod(&Natural)", &mut (|(x, y)| no_out!(x.mod_op(&y)))),
        ],
    );
}

fn benchmark_limb_mod_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.mod_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.mod_assign(Natural)",
                &mut (|(mut x, y)| x.mod_assign(y)),
            ),
            (
                "Limb.mod_assign(&Natural)",
                &mut (|(mut x, y)| x.mod_assign(&y)),
            ),
        ],
    );
}

fn benchmark_limb_neg_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.neg_mod(Natural)",
                &mut (|(x, y)| no_out!(x.neg_mod(y))),
            ),
            (
                "Limb.neg_mod(&Natural)",
                &mut (|(x, y)| no_out!(x.neg_mod(&y))),
            ),
        ],
    );
}
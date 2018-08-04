use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    pairs_of_natural_and_small_signed, rm_pairs_of_natural_and_small_signed,
    triples_of_natural_small_signed_and_rounding_mode_var_1,
};
use malachite_base::misc::Named;
use malachite_base::num::{ShlRound, ShlRoundAssign};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_shl_assign_i8);
    register_demo!(registry, demo_natural_shl_assign_i16);
    register_demo!(registry, demo_natural_shl_assign_i32);
    register_demo!(registry, demo_natural_shl_assign_i64);

    register_demo!(registry, demo_natural_shl_i8);
    register_demo!(registry, demo_natural_shl_i16);
    register_demo!(registry, demo_natural_shl_i32);
    register_demo!(registry, demo_natural_shl_i64);

    register_demo!(registry, demo_natural_shl_i8_ref);
    register_demo!(registry, demo_natural_shl_i16_ref);
    register_demo!(registry, demo_natural_shl_i32_ref);
    register_demo!(registry, demo_natural_shl_i64_ref);

    register_demo!(registry, demo_natural_shl_round_assign_i8);
    register_demo!(registry, demo_natural_shl_round_assign_i16);
    register_demo!(registry, demo_natural_shl_round_assign_i32);
    register_demo!(registry, demo_natural_shl_round_assign_i64);

    register_demo!(registry, demo_natural_shl_round_i8);
    register_demo!(registry, demo_natural_shl_round_i16);
    register_demo!(registry, demo_natural_shl_round_i32);
    register_demo!(registry, demo_natural_shl_round_i64);

    register_demo!(registry, demo_natural_shl_round_i8_ref);
    register_demo!(registry, demo_natural_shl_round_i16_ref);
    register_demo!(registry, demo_natural_shl_round_i32_ref);
    register_demo!(registry, demo_natural_shl_round_i64_ref);

    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_i8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_i16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_i64_evaluation_strategy
    );

    register_bench!(registry, Large, benchmark_natural_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_natural_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_natural_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_natural_shl_round_assign_i64);

    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_round_i8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_round_i16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_round_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_round_i64_evaluation_strategy
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_assign_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_i32_library_comparison
    );
}

macro_rules! demos_and_benches {
    (
        $t:ident,
        $demo_natural_shl_assign_i:ident,
        $demo_natural_shl_i:ident,
        $demo_natural_shl_i_ref:ident,
        $demo_natural_shl_round_assign_i:ident,
        $demo_natural_shl_round_i:ident,
        $demo_natural_shl_round_i_ref:ident,
        $benchmark_natural_shl_i_evaluation_strategy:ident,
        $benchmark_natural_shl_round_assign_i:ident,
        $benchmark_natural_shl_round_i_evaluation_strategy:ident
    ) => {
        fn $demo_natural_shl_assign_i(gm: GenerationMode, limit: usize) {
            for (mut n, i) in pairs_of_natural_and_small_signed::<$t>(gm).take(limit) {
                let n_old = n.clone();
                n <<= i;
                println!("x := {}; x <<= {}; x = {}", n_old, i, n);
            }
        }

        fn $demo_natural_shl_i(gm: GenerationMode, limit: usize) {
            for (n, i) in pairs_of_natural_and_small_signed::<$t>(gm).take(limit) {
                let n_old = n.clone();
                println!("{} << {} = {}", n_old, i, n << i);
            }
        }

        fn $demo_natural_shl_i_ref(gm: GenerationMode, limit: usize) {
            for (n, i) in pairs_of_natural_and_small_signed::<$t>(gm).take(limit) {
                println!("&{} << {} = {}", n, i, &n << i);
            }
        }

        fn $demo_natural_shl_round_assign_i(gm: GenerationMode, limit: usize) {
            for (mut n, i, rm) in
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                n.shl_round_assign(i, rm);
                println!(
                    "x := {}; x.shl_round_assign({}, {}); x = {}",
                    n_old, i, rm, n
                );
            }
        }

        fn $demo_natural_shl_round_i(gm: GenerationMode, limit: usize) {
            for (n, i, rm) in
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                let n_old = n.clone();
                println!(
                    "{}.shl_round({}, {}) = {}",
                    n_old,
                    i,
                    rm,
                    n.shl_round(i, rm)
                );
            }
        }

        fn $demo_natural_shl_round_i_ref(gm: GenerationMode, limit: usize) {
            for (n, i, rm) in
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>(gm).take(limit)
            {
                println!(
                    "(&{}).shl_round({}, {}) = {}",
                    n,
                    i,
                    rm,
                    (&n).shl_round(i, rm)
                );
            }
        }

        fn $benchmark_natural_shl_i_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural << {}", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                pairs_of_natural_and_small_signed::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| other as usize),
                "other",
                &mut [
                    (
                        &format!("Natural << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(x << y)),
                    ),
                    (
                        &format!("&Natural << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(&x << y)),
                    ),
                ],
            );
        }

        fn $benchmark_natural_shl_round_assign_i(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural.shl_round_assign({}, RoundingMode)", $t::NAME),
                BenchmarkType::Single,
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| other as usize),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, rm)| x.shl_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_natural_shl_round_i_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural.shl_round({}, RoundingMode)", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                triples_of_natural_small_signed_and_rounding_mode_var_1::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| other as usize),
                "other",
                &mut [
                    (
                        &format!("Natural.shl_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!(x.shl_round(y, rm))),
                    ),
                    (
                        &format!("(&Natural).shl_round({}, RoundingMode)", $t::NAME),
                        &mut (|(x, y, rm)| no_out!((&x).shl_round(y, rm))),
                    ),
                ],
            );
        }
    };
}
demos_and_benches!(
    i8,
    demo_natural_shl_assign_i8,
    demo_natural_shl_i8,
    demo_natural_shl_i8_ref,
    demo_natural_shl_round_assign_i8,
    demo_natural_shl_round_i8,
    demo_natural_shl_round_i8_ref,
    benchmark_natural_shl_i8_evaluation_strategy,
    benchmark_natural_shl_round_assign_i8,
    benchmark_natural_shl_round_i8_evaluation_strategy
);
demos_and_benches!(
    i16,
    demo_natural_shl_assign_i16,
    demo_natural_shl_i16,
    demo_natural_shl_i16_ref,
    demo_natural_shl_round_assign_i16,
    demo_natural_shl_round_i16,
    demo_natural_shl_round_i16_ref,
    benchmark_natural_shl_i16_evaluation_strategy,
    benchmark_natural_shl_round_assign_i16,
    benchmark_natural_shl_round_i16_evaluation_strategy
);
demos_and_benches!(
    i32,
    demo_natural_shl_assign_i32,
    demo_natural_shl_i32,
    demo_natural_shl_i32_ref,
    demo_natural_shl_round_assign_i32,
    demo_natural_shl_round_i32,
    demo_natural_shl_round_i32_ref,
    benchmark_natural_shl_i32_evaluation_strategy,
    benchmark_natural_shl_round_assign_i32,
    benchmark_natural_shl_round_i32_evaluation_strategy
);
demos_and_benches!(
    i64,
    demo_natural_shl_assign_i64,
    demo_natural_shl_i64,
    demo_natural_shl_i64_ref,
    demo_natural_shl_round_assign_i64,
    demo_natural_shl_round_i64,
    demo_natural_shl_round_i64_ref,
    benchmark_natural_shl_i64_evaluation_strategy,
    benchmark_natural_shl_round_assign_i64,
    benchmark_natural_shl_round_i64_evaluation_strategy
);

fn benchmark_natural_shl_assign_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural <<= i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| other as usize),
        "other",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x <<= y)),
            ("rug", &mut (|((mut x, y), _)| x <<= y)),
        ],
    );
}

fn benchmark_natural_shl_i32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural << i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| other as usize),
        "other",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x << y))),
            ("rug", &mut (|((x, y), _)| no_out!(x << y))),
        ],
    );
}
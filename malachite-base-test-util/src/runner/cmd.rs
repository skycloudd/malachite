use std::str::FromStr;

use clap::{App, Arg};
use itertools::Itertools;

use generators::common::{GenConfig, GenMode};

#[derive(Clone, Debug)]
pub struct CommandLineArguments {
    pub demo_key: Option<String>,
    pub bench_key: Option<String>,
    pub generation_mode: GenMode,
    pub config: GenConfig,
    pub limit: usize,
    pub out: String,
}

pub fn read_command_line_arguments() -> CommandLineArguments {
    let matches = App::new("malachite-base test utils")
        .version("0.1.0")
        .author("Mikhail Hogrefe <mikhailhogrefe@gmail.com>")
        .about("Runs demos and benchmarks for malachite-base functions.")
        .arg(
            Arg::with_name("generation_mode")
                .short("m")
                .long("generation_mode")
                .help("May be 'exhaustive', 'random', or 'special_random'.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("e.g. 'mean_run_length_n 4 mean_run_length_d 1'")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("limit")
                .short("l")
                .long("limit")
                .help("Specifies the maximum number of elements to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .help("Specifies the file name to write a benchmark to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("demo")
                .short("d")
                .long("demo")
                .help("Specifies the demo name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bench")
                .short("b")
                .long("bench")
                .help("Specifies the benchmark name")
                .takes_value(true),
        )
        .get_matches();

    let generation_mode = match matches.value_of("generation_mode").unwrap_or("exhaustive") {
        "exhaustive" => GenMode::Exhaustive,
        "random" => GenMode::Random,
        "special_random" => GenMode::SpecialRandom,
        _ => panic!("Invalid generation mode"),
    };
    let config_string = matches.value_of("config").unwrap_or("");
    let mut config = GenConfig::new();
    if !config_string.is_empty() {
        for mut chunk in &config_string.split(' ').chunks(2) {
            let key = chunk.next().unwrap();
            let value =
                u64::from_str(chunk.next().expect("Bad config")).expect("Invalid config value");
            config.insert(key.to_string(), value);
        }
    }
    let limit =
        usize::from_str(matches.value_of("limit").unwrap_or("10000")).expect("Invalid limit");
    let out = matches.value_of("out").unwrap_or("temp.gp").to_string();
    let demo_key = matches.value_of("demo").map(ToString::to_string);
    let bench_key = matches.value_of("bench").map(ToString::to_string);
    if demo_key.is_none() && bench_key.is_none() {
        panic!("Must specify demo or bench");
    }
    CommandLineArguments {
        demo_key,
        bench_key,
        generation_mode,
        config,
        limit,
        out,
    }
}

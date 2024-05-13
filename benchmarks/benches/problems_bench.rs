use benchmarks::{run_algebraic, run_radio, run_sat};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use loader_config::{Config, PopConfig, PopType};

pub const PARALLEL: bool = cfg!(feature = "parallel");

pub fn sat_benchmark(c: &mut Criterion) {
    for (group_name, configs) in create_sat_configs() {
        let mut group = c.benchmark_group(group_name);
        group.sample_size(20);
        for config in configs {
            group.bench_with_input(
                BenchmarkId::new("SAT-3", &config.pop_config.pop_size),
                &config,
                |b, config| {
                    b.iter(|| {
                        run_sat("../data/instances/sat-3/uf100-01.cnf", *config)
                    })
                },
            );
        }
    }
}

pub fn algebraic_benchmark(c: &mut Criterion) {
    for (group_name, configs) in create_algebraic_configs() {
        let mut group = c.benchmark_group(group_name);
        group.sample_size(20);
        for config in configs {
            group.bench_with_input(
                BenchmarkId::new("ALGEBRAIC", &config.pop_config.pop_size),
                &config,
                |b, config| {
                    b.iter(|| {
                        run_algebraic(
                            "../data/instances/algebraic-function/\
                             algebraic-function.txt",
                            *config,
                        )
                    })
                },
            );
        }
    }
}

pub fn radio_benchmark(c: &mut Criterion) {
    for (group_name, configs) in create_radio_configs() {
        let mut group = c.benchmark_group(group_name);
        group.sample_size(20);
        for config in configs {
            group.bench_with_input(
                BenchmarkId::new("RADIO", &config.pop_config.pop_size),
                &config,
                |b, config| {
                    b.iter(|| {
                        run_radio("../data/instances/radio/radio_1.txt", *config)
                    })
                },
            );
        }
    }
}

fn create_sat_configs() -> Vec<(String, Vec<Config>)> {
    let mut configs: Vec<(String, Vec<Config>)> = Vec::new();
    for gen_i in 1..=5 {
        let mut pop_configs = Vec::new();
        for pop_i in 0..=1 {
            let pop_config = PopConfig {
                dim: 100,
                pop_size: 30 + (50 * pop_i),
                pop_type: PopType::Binary,
                bounds: None,
            };
            let mut config = Config::default();
            config.pop_config = pop_config.clone();
            config.qtd_gen = 1000 * gen_i;
            config.qtd_runs = 2;
            config.generations_to_genocide = 200;
            pop_configs.push(config);
        }
        configs.push((
            format!("SAT-3 POP_SIZE {} PARALLEL {}", 1000 * gen_i, PARALLEL),
            pop_configs,
        ));
    }
    configs
}

fn create_algebraic_configs() -> Vec<(String, Vec<Config>)> {
    let mut configs: Vec<(String, Vec<Config>)> = Vec::new();
    for gen_i in 1..=5 {
        let mut pop_configs = Vec::new();
        for pop_i in 0..=1 {
            let pop_config = PopConfig {
                dim: 16,
                pop_size: 10 + (pop_i * 30),
                pop_type: PopType::Binary,
                bounds: None,
            };
            let mut config = Config::default();
            config.pop_config = pop_config.clone();
            config.qtd_gen = 10 * gen_i;
            config.qtd_runs = 2;
            config.generations_to_genocide = 50;
            pop_configs.push(config);
        }
        configs.push((
            format!("ALGEBRAIC GENS {} PARALLEL {}", 10 * gen_i, PARALLEL),
            pop_configs,
        ));
    }
    configs
}

fn create_radio_configs() -> Vec<(String, Vec<Config>)> {
    let mut configs: Vec<(String, Vec<Config>)> = Vec::new();
    for gen_i in 1..=5 {
        let mut pop_configs = Vec::new();
        for pop_i in 0..=1 {
            let pop_config = PopConfig {
                dim: 10,
                pop_size: 10 + (pop_i * 30),
                pop_type: PopType::Binary,
                bounds: None,
            };
            let mut config = Config::default();
            config.pop_config = pop_config.clone();
            config.qtd_gen = 10 * gen_i;
            config.qtd_runs = 2;
            config.generations_to_genocide = 50;
            pop_configs.push(config);
        }
        configs.push((
            format!("RADIO GENS {} PARALLEL {}", 10 * gen_i, PARALLEL),
            pop_configs,
        ));
    }
    configs
}

criterion_group!(benches, radio_benchmark, algebraic_benchmark, sat_benchmark);
criterion_main!(benches);

use benchmarks::{run_algebraic, run_nqueens, run_radio, run_sat};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use loader_config::{Config, CrossoverMethod, PopConfig, PopType};

pub const PARALLEL: bool = cfg!(feature = "parallel");

pub fn sat_benchmark(c: &mut Criterion) {
    for (group_name, configs) in create_sat_configs() {
        let mut group = c.benchmark_group(group_name);
        group.sample_size(10);
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
        group.sample_size(10);
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
        group.sample_size(10);
        for config in configs {
            group.bench_with_input(
                BenchmarkId::new("RADIO", &config.pop_config.pop_size),
                &config,
                |b, config| {
                    b.iter(|| {
                        run_radio(
                            "../data/instances/radio/radio_1.txt",
                            *config,
                        )
                    })
                },
            );
        }
    }
}

pub fn nqueens_benchmark(c: &mut Criterion) {
    for (group_name, configs) in create_nqueens_configs() {
        let mut group = c.benchmark_group(group_name);
        group.sample_size(10);
        for config in configs {
            group.bench_with_input(
                BenchmarkId::new("NQUEENS", &config.pop_config.pop_size),
                &config,
                |b, config| {
                    b.iter(|| {
                        run_nqueens(
                            format!(
                                "../data/instances/nqueens/nqueens_{}.txt",
                                config.pop_config.dim
                            )
                            .as_str(),
                            *config,
                        )
                    })
                },
            );
        }
    }
}

fn create_sat_configs() -> Vec<(String, Vec<Config>)> {
    let mut configs: Vec<(String, Vec<Config>)> = Vec::new();
    for gen_i in 0..1 {
        let mut pop_configs = Vec::new();
        for pop_i in 0..1 {
            let pop_config = PopConfig {
                dim: 100,
                pop_size: 100,
                pop_type: PopType::Binary,
                bounds: None,
            };
            let mut config = Config::default();
            config.pop_config = pop_config.clone();
            config.qtd_gen = 15000;
            config.qtd_runs = 1;
            config.generations_to_genocide = 200000;
            pop_configs.push(config);
        }
        configs.push((
            format!("SAT-3 POP_SIZE {} PARALLEL {}", 15000, PARALLEL),
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
    for gen_i in 0..1 {
        let mut pop_configs = Vec::new();
        for pop_i in 0..1 {
            let pop_config = PopConfig {
                dim: 10,
                pop_size: 30,
                pop_type: PopType::Binary,
                bounds: None,
            };
            let mut config = Config::default();
            config.pop_config = pop_config.clone();
            config.qtd_gen = 150;
            config.qtd_runs = 1;
            config.generations_to_genocide = 150;
            pop_configs.push(config);
        }
        configs.push((
            format!("RADIO GENS {} PARALLEL {}", 150, PARALLEL),
            pop_configs,
        ));
    }
    configs
}

fn create_nqueens_configs() -> Vec<(String, Vec<Config>)> {
    let mut configs: Vec<(String, Vec<Config>)> = Vec::new();
    for (board, pop_size, qtd_gen) in
        [(8, 30, 300), (128, 100, 7500), (512, 100, 15000)].iter()
    {
        let mut pop_configs = Vec::new();
        let pop_config = PopConfig {
            dim: *board,
            pop_size: *pop_size,
            pop_type: PopType::Permuted,
            bounds: None,
        };
        let mut config = Config::default();
        config.crossover_method = CrossoverMethod::Cycle;
        config.pop_config = pop_config.clone();
        config.qtd_gen = *qtd_gen;
        config.qtd_runs = 1;
        config.generations_to_genocide = *qtd_gen + 1;
        pop_configs.push(config);
        configs.push((
            format!("NQUEENS GENS {} PARALLEL {}", qtd_gen, PARALLEL),
            pop_configs,
        ));
    }
    configs
}

criterion_group!(benches, sat_benchmark);
criterion_main!(benches);

use benchmarks::{run_algebraic, run_radio, run_sat};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use loader_config::{Config, PopConfig, PopType};

pub const PARALLEL: bool = cfg!(features="parallel");

pub fn sat_benchmark(c: &mut Criterion) {
    for (group_name, configs) in create_sat_configs() {
        let mut group = c.benchmark_group(group_name);
        for config in configs {
            group.bench_with_input(
                BenchmarkId::new("SAT-3", &config.qtd_gen),
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
        for config in configs {
            group.bench_with_input(
                BenchmarkId::new("ALGEBRAIC", &config.qtd_gen),
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
        for config in configs {
            group.bench_with_input(
                BenchmarkId::new("RADIO", &config.qtd_gen),
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

fn create_sat_configs() -> Vec<(String, Vec<Config>)> {
    let mut configs: Vec<(String, Vec<Config>)> = Vec::new();
    for pop_i in 1..=10 {
        let mut gen_configs = Vec::new();
        for gen_i in 1..=100 {
            let config = Config {
                pop_config: PopConfig {
                    dim: 100,
                    pop_size: 10 * pop_i,
                    pop_type: PopType::BINARY,
                    bounds: None,
                },
                qtd_gen: 100 * gen_i,
                qtd_runs: 3,
                generations_to_genocide: 200,
                elitism: true,
                selection_method: loader_config::SelectionMethod::Tournament,
                crossover_method: loader_config::CrossOverMethod::OnePoint,
                crossover_chance: 0.9,
                mutation_chance: 0.02,
                constraint_penalty: -1.0,
                kp: 0.9,
            };
            gen_configs.push(config);
        }
        configs.push((
            format!("SAT-3 POP_SIZE {} PARALLEL {}", 10 * pop_i, PARALLEL),
            gen_configs,
        ));
    }
    configs
}

fn create_algebraic_configs() -> Vec<(String, Vec<Config>)> {
    let mut configs: Vec<(String, Vec<Config>)> = Vec::new();
    for pop_i in 1..=10 {
        let mut gen_configs = Vec::new();
        for gen_i in 1..=100 {
            let config = Config {
                pop_config: PopConfig {
                    dim: 16,
                    pop_size: 6 * pop_i,
                    pop_type: PopType::BINARY,
                    bounds: None,
                },
                qtd_gen: 10 * gen_i,
                qtd_runs: 3,
                generations_to_genocide: 50,
                elitism: true,
                selection_method: loader_config::SelectionMethod::Tournament,
                crossover_method: loader_config::CrossOverMethod::OnePoint,
                crossover_chance: 0.9,
                mutation_chance: 0.02,
                constraint_penalty: -1.0,
                kp: 0.9,
            };
            gen_configs.push(config);
        }
        configs.push((
            format!("ALGEBRAIC POP_SIZE {} PARALLEL {}", 10 * pop_i, PARALLEL),
            gen_configs,
        ));
    }
    configs
}

fn create_radio_configs() -> Vec<(String, Vec<Config>)> {
    let mut configs: Vec<(String, Vec<Config>)> = Vec::new();
    for pop_i in 1..=10 {
        let mut gen_configs = Vec::new();
        for gen_i in 1..=5 {
            let config = Config {
                pop_config: PopConfig {
                    dim: 10,
                    pop_size: 6 * pop_i,
                    pop_type: PopType::BINARY,
                    bounds: None,
                },
                qtd_gen: 10 * gen_i,
                qtd_runs: 3,
                generations_to_genocide: 50,
                elitism: true,
                selection_method: loader_config::SelectionMethod::Tournament,
                crossover_method: loader_config::CrossOverMethod::OnePoint,
                crossover_chance: 0.9,
                mutation_chance: 0.02,
                constraint_penalty: -1.2,
                kp: 0.9,
            };
            gen_configs.push(config);
        }
        configs.push((
            format!("RADIO POP_SIZE {} PARALLEL {}", 10 * pop_i, PARALLEL),
            gen_configs,
        ));
    }
    configs
}

criterion_group!(benches, radio_benchmark);
criterion_main!(benches);

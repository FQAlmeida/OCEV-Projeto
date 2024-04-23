use anyhow::Result;

#[derive(Debug, Copy, Clone)]
pub enum PopType {
    BINARY,
    REAL,
    INTEGER,
    PERMUTED,
}
#[derive(Debug, Copy, Clone)]
pub enum SelectionMethod {
    ROULETTE,
    TOURNAMENT,
}

#[derive(Debug, Copy, Clone)]
pub enum CrossOverMethod {
    OnePoint,
}

#[derive(Debug, Copy, Clone)]
pub struct BoundConfig {
    pub upper: f64,
    pub lower: f64,
}
#[derive(Debug, Copy, Clone)]
pub struct PopConfig {
    pub dim: usize,
    pub pop_size: usize,
    pub pop_type: PopType,
    pub bounds: Option<BoundConfig>,
}

#[derive(Clone, Debug, Copy)]
pub struct Config {
    pub pop_config: PopConfig,
    pub qtd_gen: usize,
    pub qtd_runs: usize,
    pub generations_to_genocide: usize,
    pub elitism: bool,
    pub selection_method: SelectionMethod,
    pub crossover_method: CrossOverMethod,
    pub crossover_chance: f64,
    pub mutation_chance: f64,
    pub constraint_penalty: f64,
    pub kp: f64,
}

impl Config {
    pub fn new() -> Config {
        Config {
            pop_config: PopConfig {
                dim: 100,
                pop_size: 30,
                pop_type: PopType::BINARY,
                bounds: None,
            },
            qtd_gen: 10000,
            qtd_runs: 5,
            constraint_penalty: 1.0,
            crossover_chance: 0.9,
            crossover_method: CrossOverMethod::OnePoint,
            elitism: true,
            generations_to_genocide: 200,
            kp: 0.9,
            mutation_chance: 0.03,
            selection_method: SelectionMethod::ROULETTE,
        }
    }
}

impl Config {
    pub fn load(_: &str) -> Result<Config> {
        return Ok(Config {
            pop_config: PopConfig {
                dim: 100,
                pop_size: 50,
                pop_type: PopType::BINARY,
                bounds: None,
            },
            qtd_gen: 5000,
            qtd_runs: 10,
            generations_to_genocide: 150,
            elitism: true,
            selection_method: SelectionMethod::TOURNAMENT,
            crossover_method: CrossOverMethod::OnePoint,
            crossover_chance: 0.85,
            mutation_chance: 0.025,
            constraint_penalty: -1.0,
            kp: 0.9,
        });
    }
}

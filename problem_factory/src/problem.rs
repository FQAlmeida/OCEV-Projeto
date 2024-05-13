use loader_config::Config;
use population::Individual;

pub trait Problem {
    fn get_name(&self) -> String;
    fn get_config(&self) -> &Config;
    fn objective(&self, individual: &[f64]) -> f64;
    fn normed_objective(&self, individual: &[f64]) -> f64;
    fn constraint(&self, individual: &[f64]) -> f64;
    fn fitness(&self, individual: &Individual) -> f64;
    fn decode(&self, individual: &Individual) -> Vec<f64>;
}
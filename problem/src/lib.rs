use individual_creation::Individual;
use loader_config::Config;

pub trait Problem {
    fn get_instance(&self);
    fn get_config(&self) -> &Config;
    fn objective(&self, individual: &Individual) -> f64;
    fn normed_objective(&self, individual: &Individual) -> f64;
    fn constraint(&self, individual: &Individual) -> f64;
    fn fitness(&self, individual: &Individual) -> f64;
}

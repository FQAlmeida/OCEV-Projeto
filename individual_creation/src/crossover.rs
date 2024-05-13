use crate::Population;

pub trait Crossover {
    #[allow(dead_code)]
    fn crossover(&self, mating_pool: &[(usize, usize)]) -> Population;
}

pub struct OnePoint {}

impl Crossover for OnePoint {
    fn crossover(&self, _: &[(usize, usize)]) -> Population {
        todo!()
    }
}

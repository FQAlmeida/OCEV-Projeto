use std::fmt::Display;

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum IndividualType {
    Binary(bool),
}

impl Display for IndividualType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndividualType::Binary(value) => write!(f, "{}", *value),
        }
    }
}

impl Into<bool> for IndividualType {
    fn into(self) -> bool {
        match self {
            IndividualType::Binary(value) => value,
        }
    }
}

impl IndividualType {
    pub fn mutate(self) -> IndividualType {
        match self {
            IndividualType::Binary(value) => IndividualType::Binary(!value),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Individual {
    pub chromosome: Vec<IndividualType>,
}

impl Individual {
    pub fn new(dim: usize, individual_type: &IndividualType) -> Individual {
        let mut rng = rand::thread_rng();
        let mut chromosome = Vec::with_capacity(dim);
        for _ in 0..dim {
            let value = match individual_type {
                IndividualType::Binary(_) => rng.gen::<bool>(),
            };
            chromosome.push(IndividualType::Binary(value));
        }

        Individual { chromosome }
    }
}

#[derive(Debug, Clone)]
pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    pub fn new(qtd_individuals: usize, dim: usize, individual_type: &IndividualType) -> Population {
        let individuals: Vec<Individual> = (0..qtd_individuals)
            .map(|_| Individual::new(dim, individual_type))
            .collect();
        Population { individuals }
    }
}

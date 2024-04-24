use std::fmt::Display;

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum IndividualType {
    Binary(bool),
    Permuted(usize),
}

impl Display for IndividualType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndividualType::Binary(value) => write!(f, "{}", *value),
            IndividualType::Permuted(value) => write!(f, "{}", *value),
        }
    }
}

impl From<IndividualType> for bool {
    fn from(val: IndividualType) -> Self {
        match val {
            IndividualType::Binary(value) => value,
            IndividualType::Permuted(value) => value != 0,
        }
    }
}

impl IndividualType {
    #[must_use]
    pub fn mutate(self) -> Self {
        match self {
            IndividualType::Binary(value) => IndividualType::Binary(!value),
            IndividualType::Permuted(_) => todo!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Individual {
    pub chromosome: Vec<IndividualType>,
}

impl Individual {
    #[must_use]
    pub fn new(dim: usize, individual_type: &IndividualType) -> Self {
        let mut rng = rand::thread_rng();
        let mut chromosome = Vec::with_capacity(dim);
        for _ in 0..dim {
            let value = match individual_type {
                IndividualType::Binary(_) => rng.gen::<bool>(),
                IndividualType::Permuted(_) => todo!(),
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
    #[must_use]
    pub fn new(
        qtd_individuals: usize,
        dim: usize,
        individual_type: &IndividualType,
    ) -> Self {
        let individuals: Vec<Individual> = (0..qtd_individuals)
            .map(|_| Individual::new(dim, individual_type))
            .collect();
        Population { individuals }
    }
}

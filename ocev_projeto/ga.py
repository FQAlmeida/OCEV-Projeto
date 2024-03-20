from multiprocessing.pool import Pool
from time import time

import numpy as np
from matplotlib.pylab import SeedSequence

from ocev_projeto.population_initialization import PopGenerator
from ocev_projeto.problem import Problem


class GA:
    def __init__(
        self,
        problem: Problem,
        pool: Pool,
    ) -> None:
        seed_seq = SeedSequence(round(time()))
        rng = np.random.default_rng(seed_seq)
        self.problem = problem
        self.pop_generator = PopGenerator(self.problem.config.pop_config, rng)
        self.population = self.pop_generator.generate_pop()
        self.best_individual_index = (None, None)
        self.pool = pool

    def run(self):
        for _ in range(self.problem.config.qtd_gen):
            result = self.__fitness(self.population)
            ranked_results = sorted(enumerate(result), key=lambda x: x[1])
            best_individual_index = ranked_results[0]
            if (
                not self.best_individual_index[1]
                or best_individual_index[1] > self.best_individual_index[1]
            ):
                self.best_individual_index = (
                    self.population[best_individual_index[0]],
                    best_individual_index[1],
                )
            self.population = self.pop_generator.generate_pop()
        return self.best_individual, self.best_individual_value

    def __fitness(self, population: np.ndarray):
        result = self.pool.map(self.problem.objective, population)
        return np.array(result)

    @property
    def best_individual(self):
        if self.best_individual_index[1] is None:
            return None
        return self.best_individual_index[0]

    @property
    def best_individual_value(self):
        return self.best_individual_index[1]

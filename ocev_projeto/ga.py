import logging
from multiprocessing.pool import Pool
from time import time

import numpy as np
from matplotlib.pylab import SeedSequence

from ocev_projeto.models.config import CrossOverMethod, PopType
from ocev_projeto.population_initialization import PopGenerator
from ocev_projeto.problem import Problem

logger = logging.getLogger("GA")


class GA:
    def __init__(
        self,
        problem: Problem,
        pool: Pool,
    ) -> None:
        seed_seq = SeedSequence(round(time()))
        self.rng = np.random.default_rng(seed_seq)
        self.problem = problem
        self.pop_generator = PopGenerator(self.problem.config.pop_config, self.rng)
        self.population = self.pop_generator.generate_pop()
        self.best_individual_index = (None, None)
        self.pool = pool

    def run(self):
        for _ in range(self.problem.config.qtd_gen):
            result = self.__fitness()
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
            logger.info(
                "Best Individual: "
                f"{self.best_individual} {self.best_individual_value}"
            )
            mating_pool = self.__selection(result)
            mating_pool = self.__cross_over(mating_pool)
            mating_pool = self.__mutation(mating_pool)
            self.population = mating_pool
        return self.best_individual, self.best_individual_value

    def __fitness(self):
        result = self.pool.map(self.problem.objective, self.population)
        return np.array(result)

    def __selection(self, result: np.ndarray):
        sum_fitness = np.sum(result)
        p_selection = result / sum_fitness
        mating_pool = self.rng.choice(
            len(self.population), len(self.population), True, p_selection
        )
        return self.population[mating_pool]

    def __cross_over(self, mating_pool):
        crossover_method = self.problem.config.crossover_method
        match crossover_method:
            case CrossOverMethod.ONE_POINT:
                return self.__cross_over_one_point(mating_pool)
        raise Exception("no cross impld")

    def __cross_over_one_point(self, mating_pool: np.ndarray):
        pop_type = self.problem.config.pop_config.pop_type
        pop_size = self.problem.config.pop_config.pop_size
        dim = self.problem.config.pop_config.dim
        crossover_chance = self.problem.config.crossover_chance
        match pop_type:
            case PopType.BINARY:
                mating_pool = mating_pool.reshape(-1, 2, dim)
                mask = self.rng.random((pop_size // 2, dim)) < crossover_chance
                cut = self.rng.choice(dim, (pop_size // 2, 1))
                mask = np.concatenate(
                    (np.arange(dim).reshape(1, -1) < cut, mask), axis=1
                )
                mating_pool = np.where(
                    mask, mating_pool, np.roll(mating_pool, 1, axis=1)
                )
                mating_pool = mating_pool.reshape(-1, dim)
                return mating_pool
        raise Exception("no cross type impld")

    def __mutation(self, mating_pool: np.ndarray):
        pop_type = self.problem.config.pop_config.pop_type
        pop_size = self.problem.config.pop_config.pop_size
        dim = self.problem.config.pop_config.dim
        mutation_chance = self.problem.config.mutation_chance
        match pop_type:
            case PopType.BINARY:
                mask = self.rng.choice(
                    [True, False],
                    (pop_size, dim),
                    True,
                    [mutation_chance, 1 - mutation_chance],
                )
                mating_pool = np.logical_or(
                    np.logical_and(mask, np.logical_not(mating_pool)),
                    np.logical_and(np.logical_not(mask), mating_pool),
                )
                return mating_pool
        raise Exception("no mutation type impld")

    @property
    def best_individual(self):
        if self.best_individual_index[1] is None:
            return None
        return self.best_individual_index[0]

    @property
    def best_individual_value(self):
        return self.best_individual_index[1]

from multiprocessing import Pool

import numpy as np
import pytest
from matplotlib.pylab import SeedSequence

from ocev_projeto.ga import GA
from ocev_projeto.models.config import Config, PopConfig, PopType


@pytest.fixture
def config():
    config = Config(pop_config=PopConfig(30, 100, pop_type=PopType.PERMINT))
    return config


def objective_function(problem: np.ndarray, individual: np.ndarray):
    return np.sum([problem[a][b] for (a, b) in zip(individual[:-1], individual)])


@pytest.fixture
def objective():
    return objective_function


@pytest.fixture
def problem():
    seed_seq = SeedSequence(123)
    rng = np.random.default_rng(seed_seq)
    problem = rng.integers(low=0, high=30, size=(30, 30)).astype(np.int32)
    return problem


def test_run(
    config,
    objective,
    problem,
):
    pool = Pool(4)
    ga = GA(config, objective, problem, pool)
    best_individual, best_individual_value = ga.run()
    assert best_individual is not None
    assert best_individual.shape == (30,)
    assert best_individual_value is not None


def test_best_individual(
    config,
    objective,
    problem,
):
    pool = Pool(4)
    ga = GA(config, objective, problem, pool)
    best_individual = ga.best_individual
    assert best_individual is None


def test_best_individual_value(
    config,
    objective,
    problem,
):
    pool = Pool(4)
    ga = GA(config, objective, problem, pool)
    best_individual_value = ga.best_individual_value
    assert best_individual_value is None

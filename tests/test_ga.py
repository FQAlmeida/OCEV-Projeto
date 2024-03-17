from multiprocessing import Pool

import numpy as np
import pytest
from matplotlib.pylab import SeedSequence

from ocev_projeto.ga import GA
from ocev_projeto.models.config import Config, PopConfig, PopType, pkl_to_config
from ocev_projeto.sat3 import SAT3


@pytest.fixture
def config():
    config = pkl_to_config("data/config/sat-3.pkl")
    return config


@pytest.fixture
def problem(config):
    problem = SAT3(config)
    return problem


def test_run(
    problem,
):
    pool = Pool(4)
    ga = GA(problem, pool)
    best_individual, best_individual_value = ga.run()
    assert best_individual is not None
    assert best_individual.shape == (30,)
    assert best_individual_value is not None


def test_best_individual(
    problem,
):
    pool = Pool(4)
    ga = GA(problem, pool)
    best_individual = ga.best_individual
    assert best_individual is None


def test_best_individual_value(
    problem,
):
    pool = Pool(4)
    ga = GA(problem, pool)
    best_individual_value = ga.best_individual_value
    assert best_individual_value is None

from multiprocessing.pool import Pool

import numpy as np
import pytest
from numpy.random import SeedSequence
from pytest_mock import MockerFixture

from ocev_projeto.framework import GAFramework
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


def test_ga_framework_run(mocker: MockerFixture, config, objective, problem):
    framework = GAFramework(config, objective, problem)
    framework_run = mocker.spy(framework, "run")
    ga_run = mocker.spy(GA, "run")
    with framework as fw:
        fw.run()
    qtd_runs = config.qtd_runs
    assert ga_run.call_count == qtd_runs
    assert framework_run.call_count == 1


def test_ga_framework_pool_close_called(
    mocker: MockerFixture, config, objective, problem
):
    framework_pool = mocker.spy(Pool, "close")
    with GAFramework(config, objective, problem) as framework:
        framework.run()

    assert framework_pool.call_count == 1


def test_ga_framework_run_best_individual_result(config, objective, problem):
    with GAFramework(config, objective, problem) as framework:
        best_individual, result = framework.run()

    assert best_individual is not None
    assert result is not None

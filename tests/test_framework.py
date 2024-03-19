from multiprocessing.pool import Pool

import pytest
from pytest_mock import MockerFixture

from ocev_projeto.framework import GAFramework
from ocev_projeto.ga import GA
from ocev_projeto.models.config import pkl_to_config
from ocev_projeto.sat3 import SAT3


@pytest.fixture
def config():
    config = pkl_to_config("data/config/sat-3-uf100-01.pkl")
    return config


@pytest.fixture
def problem(config):
    problem = SAT3(config, "uf100-01.cnf")
    return problem


def test_ga_framework_run(mocker: MockerFixture, config, problem):
    framework = GAFramework(config, problem)
    framework_run = mocker.spy(framework, "run")
    ga_run = mocker.spy(GA, "run")
    with framework as fw:
        fw.run()
    qtd_runs = config.qtd_runs
    assert ga_run.call_count == qtd_runs
    assert framework_run.call_count == 1


def test_ga_framework_pool_close_called(mocker: MockerFixture, config, problem):
    framework_pool = mocker.spy(Pool, "close")
    with GAFramework(config, problem) as framework:
        framework.run()

    assert framework_pool.call_count == 1


def test_ga_framework_run_best_individual_result(config, problem):
    with GAFramework(config, problem) as framework:
        best_individual, result = framework.run()

    assert best_individual is not None
    assert result is not None

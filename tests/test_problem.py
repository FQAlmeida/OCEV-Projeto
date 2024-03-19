import numpy as np
import pytest

from ocev_projeto.models.config import pkl_to_config
from ocev_projeto.problem import Problem


@pytest.fixture
def config():
    config = pkl_to_config("data/config/sat-3-uf100-01.pkl")
    return config


@pytest.fixture
def instance_file():
    return "uf100-01.cnf"


def test_read_instance_config(config, instance_file):
    problem = Problem("SAT-3", instance_file, config)
    config, _, _ = problem.read_instance()

    # Test config
    assert isinstance(config, str)
    assert "100" in config
    assert "430" in config
    qtd_elem_config = 4
    assert len(config.split()) == qtd_elem_config


def test_read_instance_problem(config, instance_file):
    problem = Problem("SAT-3", instance_file, config)
    _, problem, _ = problem.read_instance()

    # Test problem
    assert isinstance(problem, np.ndarray)
    assert problem.shape == (430, 3)
    assert problem.dtype == np.int32


def test_read_instance_expected_solution(config, instance_file):
    problem = Problem("SAT-3", instance_file, config)
    _, _, expected_solution = problem.read_instance()

    # Test expected_solution
    assert isinstance(expected_solution, int)
    assert expected_solution == 0

import numpy as np
import pytest

from ocev_projeto.models.config import pkl_to_config
from ocev_projeto.sat3 import SAT3


@pytest.fixture
def config():
    config = pkl_to_config("data/config/sat-3.pkl")
    return config


def test_objective_case1(config):
    sat3 = SAT3(config)
    sat3.problem = np.array([[1, 2, 3], [4, -2, -3], [-1, -5, -3]])
    individual = np.array([0, 1, 0, 1, 0])
    qtd_conflicts = 0
    assert sat3.objective(individual) == qtd_conflicts


def test_objective_case2(config):
    sat3 = SAT3(config)
    sat3.problem = np.array([[1, -2, 3], [-4, 5, -3], [2, 3, -4], [-5, -3, -1]])
    individual = np.array([1, 0, 1, 0, 1])
    qtd_conflicts = 1
    assert sat3.objective(individual) == qtd_conflicts


def test_objective_case3(config):
    sat3 = SAT3(config)
    sat3.problem = np.array([
        [1, 2, -3],
        [-4, 5, 2],
        [-3, 2, 1],
        [-4, 2, -3],
        [5, 1, -4],
    ])
    individual = np.array([0, 0, 1, 1, 0])
    qtd_conflicts = 5
    assert sat3.objective(individual) == qtd_conflicts



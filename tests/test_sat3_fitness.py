import numpy as np
import pytest

from ocev_projeto.sat3 import objective, read_instance


def test_objective_case1():
    problem = np.array([[1, 2, 3], [4, -2, -3], [-1, -5, -3]])
    individual = np.array([0, 1, 0, 1, 0])
    qtd_conflicts = 0
    assert objective(problem, individual) == qtd_conflicts


def test_objective_case2():
    problem = np.array([[1, -2, 3], [-4, 5, -3], [2, 3, -4], [-5, -3, -1]])
    individual = np.array([1, 0, 1, 0, 1])
    qtd_conflicts = 1
    assert objective(problem, individual) == qtd_conflicts


def test_objective_case3():
    problem = np.array([[1, 2, -3], [-4, 5, 2], [-3, 2, 1], [-4, 2, -3], [5, 1, -4]])
    individual = np.array([0, 0, 1, 1, 0])
    qtd_conflicts = 5
    assert objective(problem, individual) == qtd_conflicts


@pytest.fixture
def instance_file():
    return "uf100-01.cnf"


def test_read_instance_config(instance_file):
    config, _, _ = read_instance(instance_file)

    # Test config
    assert isinstance(config, str)
    assert "100" in config
    assert "430" in config
    qtd_elem_config = 4
    assert len(config.split()) == qtd_elem_config


def test_read_instance_problem(instance_file):
    _, problem, _ = read_instance(instance_file)

    # Test problem
    assert isinstance(problem, np.ndarray)
    assert problem.shape == (430, 3)
    assert problem.dtype == np.int32


def test_read_instance_expected_solution(instance_file):
    _, _, expected_solution = read_instance(instance_file)

    # Test expected_solution
    assert isinstance(expected_solution, int)
    assert expected_solution == 0

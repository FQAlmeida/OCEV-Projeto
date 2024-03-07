import numpy as np

from ocev_projeto.population_initialization import Configs, PopType, generate_pop


def test_generate_pop_int_with_config():
    dim = 50
    pop_size = 60
    upper = 10
    lower = 0
    config = Configs(upper, lower)
    result = generate_pop(dim, pop_size, PopType.INT, config)
    assert isinstance(result, np.ndarray)
    assert result.shape == (pop_size, dim)
    assert np.all(result <= upper)
    assert np.all(result >= lower)


def test_generate_pop_permint_with_config():
    dim = 80
    pop_size = 70
    upper = 100
    lower = 0
    config = Configs(upper, lower)
    result = generate_pop(dim, pop_size, PopType.PERMINT, config)
    assert isinstance(result, np.ndarray)
    assert result.shape == (pop_size, dim)
    assert np.all(result <= upper)
    assert np.all(result >= lower)
    assert np.all(np.apply_along_axis(lambda x: len(np.unique(x)) == len(x), axis=1, arr=result))


def test_generate_pop_real_with_config():
    dim = 30
    pop_size = 50
    upper = 10
    lower = -10
    config = Configs(upper, lower)
    result = generate_pop(dim, pop_size, PopType.REAL, config)
    assert isinstance(result, np.ndarray)
    assert result.shape == (pop_size, dim)
    assert np.all(result <= upper)
    assert np.all(result >= lower)


def test_generate_pop_binary_without_config():
    dim = 100
    pop_size = 150
    result = generate_pop(dim, pop_size, PopType.BINARY, None)
    assert isinstance(result, np.ndarray)
    assert np.all(np.logical_or(result == 0, result == 1))

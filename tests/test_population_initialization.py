from time import time

import numpy as np
import pytest
from matplotlib.pylab import SeedSequence

from ocev_projeto.models.config import BoundConfig, PopConfig, PopType
from ocev_projeto.population_initialization import PopGenerator

seed_seq = SeedSequence(round(time()))
rng = np.random.default_rng(seed_seq)


def test_generate_pop_int_with_config():
    dim = 50
    pop_size = 60
    upper = 10
    lower = 0
    pop_config = PopConfig(
        dim=dim, pop_size=pop_size, bounds=BoundConfig(lower=lower, upper=upper), pop_type=PopType.INT
    )
    pop_gen = PopGenerator(pop_config, rng)
    result = pop_gen.generate_pop()
    assert isinstance(result, np.ndarray)
    assert result.shape == (pop_size, dim)
    assert np.all(result <= upper)
    assert np.all(result >= lower)


def test_generate_pop_permint_without_config():
    dim = 6
    pop_size = 10
    pop_config = PopConfig(dim=dim, pop_size=pop_size, pop_type=PopType.PERMINT)
    pop_gen = PopGenerator(pop_config, rng)
    result = pop_gen.generate_pop()
    assert isinstance(result, np.ndarray)
    assert result.shape == (pop_size, dim)
    assert np.all(np.apply_along_axis(lambda x: len(np.unique(x)) == len(x), axis=1, arr=result))


def test_generate_pop_real_with_config():
    dim = 30
    pop_size = 50
    upper = 10
    lower = -10
    pop_config = PopConfig(
        dim=dim, pop_size=pop_size, bounds=BoundConfig(lower=lower, upper=upper), pop_type=PopType.REAL
    )
    pop_gen = PopGenerator(pop_config, rng)
    result = pop_gen.generate_pop()
    assert isinstance(result, np.ndarray)
    assert result.shape == (pop_size, dim)
    assert np.all(result <= upper)
    assert np.all(result >= lower)


def test_generate_pop_binary_without_config():
    dim = 100
    pop_size = 150
    pop_config = PopConfig(dim=dim, pop_size=pop_size, pop_type=PopType.BINARY)
    pop_gen = PopGenerator(pop_config, rng)
    result = pop_gen.generate_pop()
    assert isinstance(result, np.ndarray)
    assert np.all(np.logical_or(result == 0, result == 1))


def test_generate_pop_raises_exception_without_config_for_some_types():
    dim = 100
    pop_size = 150
    types_that_should_raise_exception = [
        PopType.INT,
        PopType.REAL,
    ]
    types_that_should_not_raise_exception = [
        pop_type for pop_type in PopType if pop_type not in types_that_should_raise_exception
    ]
    for pop_type in types_that_should_raise_exception:
        with pytest.raises(Exception):
            pop_config = PopConfig(dim=dim, pop_size=pop_size, pop_type=pop_type)
            pop_gen = PopGenerator(pop_config, rng)
            pop_gen.generate_pop()

    for pop_type in types_that_should_not_raise_exception:
        pop_config = PopConfig(dim=dim, pop_size=pop_size, pop_type=pop_type)
        pop_gen = PopGenerator(pop_config, rng)
        pop_gen.generate_pop()

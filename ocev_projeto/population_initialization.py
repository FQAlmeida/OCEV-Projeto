from dataclasses import dataclass
from enum import Enum

import numpy as np


class PopType(Enum):
    INT = "int"
    PERMINT = "permint"
    REAL = "real"
    BINARY = "binary"


@dataclass
class Configs:
    upper: float
    lower: float


rng = np.random.default_rng(1337)


def generate_pop(dim: int, pop_size: int, type: PopType, config: Configs | None):
    match type:
        case PopType.INT:
            if not config:
                raise Exception("Config is needed for the PopType")
            return generate_int_pop(dim, pop_size, config)
        case PopType.PERMINT:
            if not config:
                raise Exception("Config is needed for the PopType")
            return generate_permint_pop(dim, pop_size, config)
        case PopType.REAL:
            if not config:
                raise Exception("Config is needed for the PopType")
            return generate_real_pop(dim, pop_size, config)
        case PopType.BINARY:
            return generate_binary_pop(dim, pop_size)


def generate_int_pop(dim: int, pop_size: int, config: Configs):
    return rng.integers(low=int(config.lower), high=int(config.upper), size=(pop_size, dim))


def generate_permint_pop(dim: int, pop_size: int, config: Configs):
    return np.array(
        [
            rng.choice(
                a=np.arange(start=int(config.lower), stop=int(config.upper)),
                size=dim,
                replace=False,
            ).astype(np.int32)
            for _ in range(pop_size)
        ]
    )


def generate_real_pop(dim: int, pop_size: int, config: Configs):
    return rng.uniform(low=config.lower, high=config.upper, size=(pop_size, dim))


def generate_binary_pop(dim: int, pop_size: int):
    return rng.choice(a=(0, 1), size=(pop_size, dim)).astype(np.bool_)

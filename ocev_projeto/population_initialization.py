import numpy as np

from ocev_projeto.models.config import BoundsError, PopConfig, PopType


class PopGenerator:
    def __init__(self, config: PopConfig, rng: np.random.Generator) -> None:
        self.config = config
        self.rng = rng

    def generate_pop(self):
        match self.config.pop_type:
            case PopType.INT:
                return self.__generate_int_pop()
            case PopType.PERMINT:
                return self.__generate_permint_pop()
            case PopType.REAL:
                return self.__generate_real_pop()
            case PopType.BINARY:
                return self.__generate_binary_pop()

    def __generate_int_pop(self):
        if not self.config.bounds:
            raise BoundsError(self.config.pop_type)
        lower = self.config.bounds.lower
        upper = self.config.bounds.upper
        pop_size = self.config.pop_size
        dim = self.config.dim
        return self.rng.integers(low=int(lower), high=int(upper), size=(pop_size, dim)).astype(np.int32)

    def __generate_permint_pop(self):
        pop_size = self.config.pop_size
        dim = self.config.dim
        space = np.arange(0, dim)
        lines = np.arange(0, pop_size)
        x_map, _ = np.meshgrid(space, lines)
        return self.rng.permuted(x_map, axis=1).astype(np.int32)

    def __generate_real_pop(self):
        if not self.config.bounds:
            raise BoundsError(self.config.pop_type)
        lower = self.config.bounds.lower
        upper = self.config.bounds.upper
        pop_size = self.config.pop_size
        dim = self.config.dim
        return self.rng.uniform(low=lower, high=upper, size=(pop_size, dim)).astype(np.float32)

    def __generate_binary_pop(self):
        pop_size = self.config.pop_size
        dim = self.config.dim
        return self.rng.choice(a=(0, 1), size=(pop_size, dim)).astype(np.bool_)

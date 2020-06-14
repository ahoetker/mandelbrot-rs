import numpy as np
from mandelbrot_rs import generate


def test_generate():
    assert (generate(8, 4, 50) == np.array([
        [253, 253, 252, 252, 252],
        [253, 252, 252, 251, 205],
        [253, 251, 249, 205, 205],
        [253, 205, 205, 205, 205],
        [253, 251, 249, 205, 205]
    ]).T).all()
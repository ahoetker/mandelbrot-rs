# mandelbrot_rs

[![Build Status](https://travis-ci.com/ahoetker/mandelbrot-rs.svg?branch=master)](https://travis-ci.com/ahoetker/mandelbrot-rs)

`mandelbrot_rs` is a Python package for generating images of the Mandelbrot set. This project provides
Python bindings to the Rust demo crate [mandelbrot_common](https://github.com/ahoetker/mandelbrot) by the same author.

## Installation

### Linux

Use the package manager [pip](https://pip.pypa.io/en/stable/) to install foobar.

```bash
pip install mandelbrot_rs
```

### macOS or Windows

For now, CI builds are only configured for Linux. To build and install the package yourself, first install rust: https://www.rust-lang.org/tools/install 

Set the default toolchain to `nightly`:

```
rustup default nightly
```

Using a compatible version of Python (3.5+), install [Maturin](https://pypi.org/project/maturin/):

```
pip install maturin
```

Clone and `cd` to this repo, then install the wheel in your current Python environment:

```
maturin develop --release
```

A PR to add macOS/Windows builds to the CI pipeline would be a very valuable contribution.

## Usage

```python
from matplotlib import pyplot as plt
from mandelbrot_rs import generate

image = generate(n=1000, threshold=4, max_steps=50)
plt.imshow(image, cmap="Spectral")
plt.axis("off")
plt.show()
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Acknowledgments

Most of the CI/CD configuration for this package was taken directly from thedrow's `fastuuid` package, which is also
built using `maturin`: https://github.com/thedrow/fastuuid/

## License
[MIT](https://choosealicense.com/licenses/mit/)
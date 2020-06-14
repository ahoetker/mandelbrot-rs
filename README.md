# mandelbrot_rs

`mandelbrot_rs` is a Python package for generating images of the Mandelbrot set. 

## Installation

Use the package manager [pip](https://pip.pypa.io/en/stable/) to install foobar.

```bash
pip install mandelbrot_rs
```

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

## License
[MIT](https://choosealicense.com/licenses/mit/)
# OptimaImg

OptimaImg is an image processing toolkit that leverages the performance of Rust with the ease of Python.

## Features

- High-performance image operations written in Rust.
- Easy-to-use Python interface.
- Cross-platform support and optimization.
- Convert images to grayscale.
- Resize images to specified dimensions.

## Installation

To install OptimaImg, simply run the following command:

```bash
pip install optimaimg
```

Alternatively, if you have cloned the repository and want to install it directly from the source code, you can run:

```bash
poetry install
```

## Usage

After installing the package, you can use it to perform various image processing tasks:

### Convert an Image to Grayscale

To convert an image to grayscale, you can use the `convert_to_grayscale` function:

```python
from optimaimg import convert_to_grayscale

input_path = 'path/to/your/image.jpg'
output_path = 'path/to/save/grayscale_image.png'

# Convert the image to grayscale and save it
convert_to_grayscale(input_path, output_path)
```

### Resize an Image

To resize an image to specific dimensions, use the `resize_image` function:

```python
from optimaimg import resize_image

input_path = 'path/to/your/image.jpg'
output_path = 'path/to/save/resized_image.png'
width = 100  # desired width
height = 100 # desired height

# Resize the image and save it
resize_image(input_path, output_path, width, height)
```

## Contributing

Contributions are welcome! Please see `CONTRIBUTING.md` for details on how to contribute to the OptimaImg project.

## License

OptimaImg is distributed under the MIT license. See `LICENSE` for more information.

# OptimaImg

OptimaImg is an image processing toolkit that leverages the performance of Rust with the ease of Python.

## Table of Contents

- [OptimaImg](#optimaimg)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
  - [Important Note](#important-note)
  - [Usage](#usage)
    - [Convert an Image to Grayscale](#convert-an-image-to-grayscale)
    - [Resize an Image](#resize-an-image)
    - [Rotate an Image](#rotate-an-image)
    - [Apply Blur](#apply-blur)
    - [Apply Sharpen](#apply-sharpen)
    - [Apply Edge Detection](#apply-edge-detection)
    - [Apply Sepia Filter](#apply-sepia-filter)
    - [Adjust Saturation](#adjust-saturation)
    - [Adjust Brightness](#adjust-brightness)
    - [Adjust Contrast](#adjust-contrast)
    - [Adjust Hue](#adjust-hue)
  - [Benchmarks](#benchmarks)
  - [Contributing](#contributing)
  - [License](#license)

## Features

- High-performance image operations written in Rust.
- Easy-to-use Python interface.
- Cross-platform support and optimization.
- Convert images to grayscale.
- Resize images to specified dimensions.
- Rotate images by a specific rotation angle in degrees.
- Apply Gaussian blur to images.
- Sharpen images to enhance edges.
- Detect edges within images using the Sobel operator.
- Apply a sepia tone filter to images for a vintage effect.
- Adjust the saturation of images to enhance or mute colors.

## Installation

To install OptimaImg, simply run the following command:

```bash
pip install optimaimg
```

Alternatively, if you have cloned the repository and want to install it directly from the source code, you can run:

```bash
poetry install
```

## Important Note

**OptimaImg has not been tested on Windows OS and may not perform as expected on that platform.**

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

### Rotate an Image

To rotate an image by a specific rotation angle in degrees, use the `rotate_image` function:

```python
from optimaimg import rotate_image

input_path = 'path/to/your/image.jpg'
output_path = 'path/to/save/rotated_image.png'
degree = 45 # desired degree

# Rotate the image and save it
rotate_image(input_path, output_path, degree)
```

### Apply Blur

To apply a Gaussian blur to an image:

```python
from optimaimg import apply_blur

# Apply a blur with a sigma value of 2.0
apply_blur(input_path, output_path, sigma=2.0)
```

### Apply Sharpen

To sharpen an image:

```python
from optimaimg import apply_sharpen

# Sharpen the image
apply_sharpen(input_path, output_path)
```

### Apply Edge Detection

To apply edge detection to an image:

```python
from optimaimg import apply_edge_detection

# Detect edges in the image
apply_edge_detection(input_path, output_path)
```

### Apply Sepia Filter

To apply a sepia tone filter to an image:

```python
from optimaimg import apply_sepia

# Apply a sepia tone filter
apply_sepia(input_path, output_path)
```

### Adjust Saturation

To adjust the saturation of an image, thereby enhancing or muting its colors, use the `adjust_saturation` function:

```python
from optimaimg import adjust_saturation

input_path = 'path/to/your/image.jpg'
output_path = 'path/to/save/saturation_adjusted_image.png'
factor = 1.5  # factor > 1 increases saturation, factor < 1 decreases saturation

# Adjust the saturation of the image and save it
adjust_saturation(input_path, output_path, factor)
```

### Adjust Brightness

To adjust the brightness of an image:

```python
from optimaimg import adjust_brightness

input_path = 'path/to/your/image.jpg'
output_path = 'path/to/save/brightness_adjusted_image.png'
brightness = 1.2  # value > 1 increases brightness, value < 1 decreases brightness

# Adjust the brightness of the image and save it
adjust_brightness(input_path, output_path, brightness)
```

### Adjust Contrast

To adjust the contrast of an image:

```python
from optimaimg import adjust_contrast

input_path = 'path/to/your/image.jpg'
output_path = 'path/to/save/contrast_adjusted_image.png'
contrast = 1.2  # value > 1 increases contrast, value < 1 decreases contrast

# Adjust the contrast of the image and save it
adjust_contrast(input_path, output_path, contrast)
```

### Adjust Hue

To adjust the contrast of an image:

```python
from optimaimg import adjust_contrast

input_path = 'path/to/your/image.jpg'
output_path = 'path/to/save/contrast_adjusted_image.png'
hue = 1.2  # value > 1 increases contrast, value < 1 decreases contrast

# Adjust the hue of the image and save it
adjust_hue(input_path, output_path, hue)
```

## Benchmarks

Below is a performance comparison table for converting images to grayscale using OptimaImg, Pillow, and OpenCV. The times are measured in seconds and represent the average duration taken to convert a single image across multiple runs.

| Library   | Average Conversion Time (seconds) |
| --------- | --------------------------------- |
| Pillow    | ~0.20                             |
| OptimaImg | ~0.03                             |
| OpenCV    | ~0.03                             |

These benchmarks indicate that OptimaImg and OpenCV have comparable performance, with both significantly outperforming Pillow.

Please note that the actual performance can vary based on the system and the specific images processed.

## Contributing

Contributions are welcome! Please see `CONTRIBUTING.md` for details on how to contribute to the OptimaImg project.

## License

OptimaImg is distributed under the MIT license. See `LICENSE` for more information.

```

```

```

```

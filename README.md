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
    - [Batch Resize Images](#batch-resize-images)
    - [Convert Image Color Space](#convert-image-color-space)
    - [Overlay images](#overlay-images)
    - [CLI Usage Examples](#cli-usage-examples)
  - [Benchmark Results](#benchmark-results)
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
- Convert image color space.

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

### Batch Resize Images

Batch resize images to the specified dimensions:

```python
from optimaimg import batch_resize_images

input_images_path = ["path/to/image1", "path/to/image2"]
output_path = 'path/to/save/images/'
width = 20
height = 20

batch_resize_images(input_images_path, output_path, width, height)
```

### Convert Image Color Space

Convert images between different color spaces such as HSV or RGB.

```python
from optimaimg import convert_color_space

input_path = 'path/to/your/image.jpg'
output_path = 'path/to/save/converted_image.jpg'

# Convert the image color space from RGB to HSV
convert_color_space(input_path, output_path, 'rgb', 'hsv')
```

### Overlay images

Overlay an image onto another

```python
from optimaimg import overlay_images

input_path = 'path/to/your/image.jpg'
overlay_path = 'path/to/your/overlay-image.jpg
output_path = 'path/to/save/converted_image.jpg'
x = 100  # The x-coordinate of the overlay position on the base image.
y = 100 # The y-coordinate of the overlay position on the base image.
alpha = 0.2 # The transparency of the overlay (0.0 to 1.0).

overlay_images(inputh_path, overlay_path, output_path, x, y, alpha)
```

### CLI Usage Examples

- Convert an Image to Grayscale

```bash
optimaimg-cli grayscale path/to/input.jpg path/to/output.jpg
```

- Resize an Image

```bash
optimaimg-cli resize  path/to/input.jpg path/to/output.jpg 200 200
```

Options:

```bash
usage: optimaimg-cli [-h]
                     resize, grayscale, rotate, blur, sharpen, edge_detection, sepia, brightness, contrast, saturation, hue, batch_resize,
                     convert_color_space, overlay ...

optimaimg CLI

positional arguments:
  resize, grayscale, rotate, blur, sharpen, edge_detection, sepia, brightness, contrast, saturation, hue, batch_resize, convert_color_space, overlay
    resize              Resize an image
    grayscale           Convert an image to grayscale
    rotate              Rotate an image
    blur                Apply blur to an image
    sharpen             Sharpen an image
    edge_detection      Apply edge detection to an image
    sepia               Apply sepia tone to an image
    brightness          Adjust the brightness of an image
    contrast            Adjust the contrast of an image
    saturation          Adjust the saturation of an image
    hue                 Adjust the hue of an image
    batch_resize        Batch resize images
    convert_color_space
                        Convert image color space
    overlay             Overlay an image onto another

options:
  -h, --help            show this help message and exit
```

## Benchmark Results

| Function  | Grayscale | Blur    | Rotate  |
| --------- | --------- | ------- | ------- |
| OptimaImg | 0.0207    | 0.05771 | 0.03338 |
| OpenCV    | 0.04081   | 0.06736 | 0.07511 |
| Pillow    | 0.17259   | 0.66545 | 0.45785 |

These benchmark results demonstrate that OptimaImg outperforms both OpenCV and Pillow, with a notably higher speed advantage over Pillow.

> Please note that the actual performance can vary based on the system and the specific images processed.

## Contributing

Contributions are welcome! Please see `CONTRIBUTING.md` for details on how to contribute to the OptimaImg project.

## License

OptimaImg is distributed under the MIT license. See `LICENSE` for more information.

```

```

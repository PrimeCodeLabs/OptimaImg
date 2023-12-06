from typing import List
from optimaimg import (
    convert_to_grayscale as _convert_to_grayscale,
    resize_image as _resize_image,
    rotate_image as _rotate_image,
    apply_blur as _apply_blur,
    apply_sharpen as _apply_sharpen,
    apply_edge_detection as _apply_edge_detection,
    apply_sepia as _apply_sepia,
    adjust_brightness as _adjust_brightness,
    adjust_contrast as _adjust_contrast,
    adjust_saturation as _adjust_saturation,
    adjust_hue as _adjust_hue,
    batch_resize_images as _batch_resize_images,
    convert_color_space as _convert_color_space,
    overlay_images as _overlay_images,
)


def convert_to_grayscale(input_path: str, output_path: str) -> None:
    """
    Convert an image to grayscale.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the grayscale image will be saved.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _convert_to_grayscale(input_path, output_path)


def resize_image(input_path: str, output_path: str, width: int, height: int) -> None:
    """
    Resize an image to the specified dimensions.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the resized image will be saved.
    - width (int): The desired width of the image.
    - height (int): The desired height of the image.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _resize_image(input_path, output_path, width, height)


def rotate_image(input_path: str, output_path: str, degrees: float) -> None:
    """
    Rotate an image by the specified number of degrees.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the rotated image will be saved.
    - degrees (float): The angle in degrees to rotate the image.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _rotate_image(input_path, output_path, degrees)


def apply_blur(input_path: str, output_path: str, sigma: float) -> None:
    """
    Apply Gaussian blur to an image.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the blurred image will be saved.
    - sigma (float): The sigma parameter for the Gaussian function, determining the amount of blur.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _apply_blur(input_path, output_path, sigma)


def apply_sharpen(input_path: str, output_path: str) -> None:
    """
    Sharpen an image.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the sharpened image will be saved.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _apply_sharpen(input_path, output_path)


def apply_edge_detection(input_path: str, output_path: str) -> None:
    """
    Apply edge detection to an image.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the edge-detected image will be saved.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _apply_edge_detection(input_path, output_path)


def apply_sepia(input_path: str, output_path: str) -> None:
    """
    Apply a sepia tone to an image.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the sepia-toned image will be saved.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _apply_sepia(input_path, output_path)


def adjust_brightness(input_path: str, output_path: str, brightness: float) -> None:
    """
    Adjust the brightness of an image.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the adjusted image will be saved.
    - brightness (float): The amount to adjust the brightness by.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _adjust_brightness(input_path, output_path, brightness)


def adjust_contrast(input_path: str, output_path: str, contrast: float) -> None:
    """
    Adjust the contrast of an image.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the adjusted image will be saved.
    - contrast (float): The amount to adjust the contrast by.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _adjust_contrast(input_path, output_path, contrast)


def adjust_saturation(input_path: str, output_path: str, saturation: float) -> None:
    """
    Adjust the saturation of an image.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the adjusted image will be saved.
    - saturation (float): The amount to adjust the saturation by.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _adjust_saturation(input_path, output_path, saturation)


def adjust_hue(input_path: str, output_path: str, hue: float) -> None:
    """
    Adjust the hue of an image.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the adjusted image will be saved.
    - hue (float): The amount to adjust the hue by.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _adjust_hue(input_path, output_path, hue)


def batch_resize_images(
    input_paths: List[str], output_path: str, width: int, height: int
) -> None:
    """
    Batch resize images to the specified dimensions.

    Parameters:
    - input_paths (List[str]): A list of paths to the input images.
    - output_path (str): The path where the resized images will be saved.
    - width (int): The desired width of the images.
    - height (int): The desired height of the images.

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _batch_resize_images(input_paths, output_path, width, height)


def convert_color_space(
    input_path: str, output_path: str, input_space: str, output_space: str
) -> None:
    """
    Convert an image from one color space to another.

    Parameters:
    - input_path (str): The path to the input image.
    - output_path (str): The path where the converted image will be saved.
    - input_space (str): The color space of the input image (e.g., "rgb" or "hsv").
    - output_space (str): The desired color space for the output image (e.g., "rgb" or "hsv").

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _convert_color_space(input_path, output_path, input_space, output_space)


def overlay_images(
    base_path: str, overlay_path: str, output_path: str, x: int, y: int, alpha: float
) -> None:
    """
    Overlay an image onto another.

    Parameters:
    - base_path (str): The path to the base image.
    - overlay_path (str): The path to the overlay image.
    - output_path (str): The path where the composited image will be saved.
    - x (int): The x-coordinate of the overlay position on the base image.
    - y (int): The y-coordinate of the overlay position on the base image.
    - alpha (float): The transparency of the overlay (0.0 to 1.0).

    Returns:
    - None: The result of the Rust function, typically None if successful.
    """
    return _overlay_images(base_path, overlay_path, output_path, x, y, alpha)

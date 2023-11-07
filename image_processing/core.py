from optimaimg import (
    convert_to_grayscale as _convert_to_grayscale,
    resize_image as _resize_image,
    rotate_image as _rotate_image,
    apply_blur as _apply_blur,
    apply_sharpen as _apply_sharpen,
    apply_edge_detection as _apply_edge_detection,
    apply_sepia as _apply_sepia,
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

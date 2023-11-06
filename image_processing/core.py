from optimaimg import (
    convert_to_grayscale as _convert_to_grayscale,
    resize_image as _resize_image,
    rotate_image as _rotate_image,
    apply_blur as _apply_blur,
    apply_sharpen as _apply_sharpen,
    apply_edge_detection as _apply_edge_detection,
    apply_sepia as _apply_sepia,
)


def convert_to_grayscale(input_path, output_path):
    # You can add additional Python logic here if needed
    return _convert_to_grayscale(input_path, output_path)


def resize_image(input_path, output_path, width, height):
    # You can add additional Python logic here if needed
    return _resize_image(input_path, output_path, width, height)


def rotate_image(input_path, output_path, degrees):
    # You can add additional Python logic here if needed
    return _rotate_image(input_path, output_path, degrees)


def apply_blur(input_path, output_path):
    # You can add additional Python logic here if needed
    return _apply_blur(input_path, output_path)


def apply_sharpen(input_path, output_path):
    # You can add additional Python logic here if needed
    return _apply_sharpen(input_path, output_path)


def apply_edge_detection(input_path, output_path):
    # You can add additional Python logic here if needed
    return _apply_edge_detection(input_path, output_path)


def apply_sepia(input_path, output_path):
    # You can add additional Python logic here if needed
    return _apply_sepia(input_path, output_path)

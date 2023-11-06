# tests/python/test_image_processing.py
from pathlib import Path
import pytest
from PIL import Image
from optimaimg import convert_to_grayscale, resize_image, rotate_image


# Helper function to create a simple colored image for testing
def create_test_image(file_path, size=(100, 100), color=(255, 0, 0)):
    image = Image.new("RGB", size, color)
    image.save(file_path)


# Use pytest fixtures to manage test resources, like test images
@pytest.fixture
def input_image_path(tmp_path):
    # Create a real test input image file with a known color
    input_path = tmp_path / "input.jpg"
    create_test_image(input_path)
    return str(input_path)


@pytest.fixture
def output_image_path(tmp_path):
    # Define a path for the output image file
    return str(tmp_path / "output.jpg")


def test_convert_to_grayscale(input_image_path, output_image_path):
    # Call the convert_to_grayscale function with the test paths
    convert_to_grayscale(input_image_path, output_image_path)

    # Check if the output file has been created
    assert Path(output_image_path).is_file(), "Output image file was not created."

    # Open the output image to check its properties
    output_image = Image.open(output_image_path)
    # Check that the output image is in grayscale mode
    assert output_image.mode == "L", "Output image is not in grayscale mode."

    # Optionally, you can check the pixels of the output image to ensure it has been processed correctly


def test_resize_image(input_image_path, output_image_path):
    # Call the resize_image function with the test paths
    resize_image(input_image_path, output_image_path, 50, 50)

    # Check if the output file has been created
    assert Path(output_image_path).is_file(), "Output image file was not created."

    # Open the output image to check its properties
    output_image = Image.open(output_image_path)
    # Check that the output image has the expected dimensions
    assert output_image.size == (50, 50), "Output image has incorrect dimensions."


def test_rotate_image(input_image_path, output_image_path):
    # Call the rotate_image function with the test paths and a 90 degree rotation
    rotate_image(input_image_path, output_image_path, 90)

    # Check if the output file has been created
    assert Path(output_image_path).is_file(), "Output image file was not created."

    # Open the output image to check its properties
    output_image = Image.open(output_image_path)
    # Open the input image to compare
    input_image = Image.open(input_image_path)

    # Since we rotated by 90 degrees, the width of the input should match the height of the output and vice versa
    assert (
        input_image.size[0] == output_image.size[1]
    ), "Output image width does not match input image height after rotation."
    assert (
        input_image.size[1] == output_image.size[0]
    ), "Output image height does not match input image width after rotation."

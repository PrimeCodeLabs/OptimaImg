from PIL import Image, ImageChops
import numpy as np

some_threshold = 15


def create_gradient_image(width, height, output_path):
    """Create a gradient image to test filters."""
    array = np.zeros((height, width, 3), dtype=np.uint8)
    for i in range(height):
        for j in range(width):
            array[i, j] = [i % 256, j % 256, (i + j) % 256]
    img = Image.fromarray(array)
    img.save(output_path)


# Define a helper function to compare images
def rms_difference(image1, image2):
    """Calculate the root mean square difference between two images."""
    # Convert images to the same mode if different
    if image1.mode != image2.mode:
        if image1.mode == "RGB" and image2.mode == "RGBA":
            # Ignore alpha channel
            image2 = image2.convert("RGB")

    differences = ImageChops.difference(image1, image2)
    differences = differences.convert("L")  # convert to grayscale
    differences = np.array(differences)
    return np.sqrt(np.mean(np.square(differences)))


# Helper function to create a simple colored image for testing
def create_test_image(file_path, size=(100, 100), color=(255, 0, 0)):
    image = Image.new("RGB", size, color)
    image.save(file_path)

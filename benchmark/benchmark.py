import time
import cv2
from optimaimg import (
    convert_to_grayscale as optimaimg_to_grayscale,
    apply_blur as optimaimg_blur,
    rotate_image as optimaimg_rotate,
)
from PIL import Image, ImageFilter
from tabulate import tabulate

# Define the path for the input image and the output images
base = "."

# grayscale images
input_image_path = base + "/test.png"
output_image_pillow_grayscale = "./pillow_grayscale.png"
output_image_optimaimg_grayscale = "./optimaimg_grayscale.png"
output_image_cv2_grayscale = "./cv2_grayscale.png"

# blurred images
output_image_pillow_blur = "./pillow_blur.png"
output_image_optimaimg_blur = "./optimaimg_blur.png"
output_image_cv2_blur = "./cv2_blur.png"

# rotated images
output_image_pillow_rotate = "./pillow_rotate.png"
output_image_optimaimg_rotate = "./optimaimg_rotate.png"
output_image_cv2_rotate = "./cv2_rotate.png"


# Function to convert to grayscale using Pillow
def pillow_to_grayscale(input_path, output_path):
    image = Image.open(input_path).convert("L")
    image.save(output_path)


# Function to convert to grayscale using OpenCV
def cv2_to_grayscale(input_path, output_path):
    image = cv2.imread(input_path, cv2.IMREAD_GRAYSCALE)
    cv2.imwrite(output_path, image)


# Functions for Gaussian Blur
def pillow_blur(input_path, output_path):
    image = Image.open(input_path)
    blurred_image = image.filter(ImageFilter.GaussianBlur(radius=2))
    blurred_image.save(output_path)


def cv2_blur(input_path, output_path):
    image = cv2.imread(input_path)
    blurred_image = cv2.GaussianBlur(image, (5, 5), 2)
    cv2.imwrite(output_path, blurred_image)


def optimaimg_blur_wrapper(input_path, output_path):
    optimaimg_blur(input_path, output_path, 2.0)  # Assuming sigma value is 2.0


# Functions for Image Rotation
def pillow_rotate(input_path, output_path, degrees):
    image = Image.open(input_path)
    rotated_image = image.rotate(degrees)
    rotated_image.save(output_path)


def cv2_rotate(input_path, output_path, degrees):
    image = cv2.imread(input_path)
    height, width = image.shape[:2]
    center = (width / 2, height / 2)
    rotation_matrix = cv2.getRotationMatrix2D(center, degrees, 1)
    rotated_image = cv2.warpAffine(image, rotation_matrix, (width, height))
    cv2.imwrite(output_path, rotated_image)


def optimaimg_rotate_wrapper(input_path, output_path, degrees):
    optimaimg_rotate(input_path, output_path, degrees)


# Function to benchmark a function with provided arguments
def benchmark(function, *args):
    start_time = time.time()
    function(*args)
    end_time = time.time()
    return end_time - start_time


# Benchmark Pillow
pillow_grayscale_time = benchmark(
    pillow_to_grayscale, input_image_path, output_image_pillow_grayscale
)
pillow_blur_time = benchmark(pillow_blur, input_image_path, output_image_pillow_blur)
pillow_rotate_time = benchmark(
    pillow_rotate, input_image_path, output_image_pillow_rotate, 90
)

# Benchmark OptimaImg
optimaimg_grayscale_time = benchmark(
    optimaimg_to_grayscale, input_image_path, output_image_optimaimg_grayscale
)
optimaimg_blur_time = benchmark(
    optimaimg_blur_wrapper, input_image_path, output_image_optimaimg_blur
)
optimaimg_rotate_time = benchmark(
    optimaimg_rotate_wrapper, input_image_path, output_image_optimaimg_rotate, 90
)

# Benchmark OpenCV
cv2_grayscale_time = benchmark(
    cv2_to_grayscale, input_image_path, output_image_cv2_grayscale
)
cv2_blur_time = benchmark(cv2_blur, input_image_path, output_image_cv2_blur)
cv2_rotate_time = benchmark(cv2_rotate, input_image_path, output_image_cv2_rotate, 90)


# Function to get sorted results
def get_sorted_results(*args):
    results = sorted([(name, time) for name, time in args], key=lambda x: x[1])
    return [x[0] for x in results], [round(x[1], 4) for x in results]


# Sorting each operation's results
grayscale_sorted_names, grayscale_sorted_times = get_sorted_results(
    ("Pillow", pillow_grayscale_time),
    ("OptimaImg", optimaimg_grayscale_time),
    ("OpenCV", cv2_grayscale_time),
)

blur_sorted_names, blur_sorted_times = get_sorted_results(
    ("Pillow", pillow_blur_time),
    ("OptimaImg", optimaimg_blur_time),
    ("OpenCV", cv2_blur_time),
)

rotate_sorted_names, rotate_sorted_times = get_sorted_results(
    ("Pillow", pillow_rotate_time),
    ("OptimaImg", optimaimg_rotate_time),
    ("OpenCV", cv2_rotate_time),
)

# Example data for benchmark results
headers = ["Function", "Grayscale", "Blur", "Rotate"]

# Sorting the results
sorted_results = sorted(
    [
        ("Pillow", pillow_grayscale_time, pillow_blur_time, pillow_rotate_time),
        (
            "OptimaImg",
            optimaimg_grayscale_time,
            optimaimg_blur_time,
            optimaimg_rotate_time,
        ),
        ("OpenCV", cv2_grayscale_time, cv2_blur_time, cv2_rotate_time),
    ],
    key=lambda x: (
        x[1],
        x[2],
        x[3],
    ),  # Sorting based on Grayscale, Blur, and Rotate times
)

# Preparing data for the table
data = [
    [name, round(grayscale, 5), round(blur, 5), round(rotate, 5)]
    for name, grayscale, blur, rotate in sorted_results
]

# Convert data to markdown table format
markdown_table = tabulate(data, headers=headers, tablefmt="github")

print(markdown_table)

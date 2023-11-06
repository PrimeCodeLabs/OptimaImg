use image::{
    DynamicImage, ImageBuffer, ImageFormat, Luma, Rgba, open, imageops::FilterType,
};
use imageproc::{
    filter::{gaussian_blur_f32, sharpen3x3},
    geometric_transformations::{rotate_about_center, Interpolation},
    gradients::sobel_gradients,
    map::map_colors,
};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::path::Path;

const TRANSPARENT_COLOR: Rgba<u8> = Rgba([0, 0, 0, 0]);

// Generic function to save any image buffer with a specified format
fn save_image<T>(image: &ImageBuffer<T, Vec<u8>>, output_path: &str, image_format: ImageFormat) -> PyResult<()>
where
    T: image::Pixel<Subpixel = u8> + 'static + image::PixelWithColorType,
{
    image
        .save_with_format(output_path, image_format)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to save image: {}", e)))
}

// Generic function to open an image from a file path
fn open_image(path: &str) -> PyResult<DynamicImage> {
    open(Path::new(path))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to open image: {}", e)))
}

#[pyfunction]
fn resize_image(input_path: String, output_path: String, width: u32, height: u32) -> PyResult<()> {
    let img = open_image(&input_path)?;
    let resized_img = img.resize_exact(width, height, FilterType::Lanczos3);
    save_image(&resized_img.to_rgba8(), &output_path, ImageFormat::Png)
}

#[pyfunction]
fn convert_to_grayscale(input_path: String, output_path: String) -> PyResult<()> {
    let img = open_image(&input_path)?;
    save_image(&img.to_luma8(), &output_path, ImageFormat::Png)
}

#[pyfunction]
fn rotate_image(input_path: String, output_path: String, degrees: f32) -> PyResult<()> {
    let dyn_img = open_image(&input_path)?;
    let img_buffer = dyn_img.to_rgba8();
    let radians = degrees.to_radians();
    let rotated_img = rotate_about_center(&img_buffer, radians, Interpolation::Bilinear, TRANSPARENT_COLOR);
    save_image(&rotated_img, &output_path, ImageFormat::Png)
}

#[pyfunction]
fn apply_blur(input_path: String, output_path: String, sigma: f32) -> PyResult<()> {
    let img = open_image(&input_path)?;
    save_image(&gaussian_blur_f32(&img.to_rgba8(), sigma), &output_path, ImageFormat::Png)
}

#[pyfunction]
fn apply_sharpen(input_path: String, output_path: String) -> PyResult<()> {
    let img = open_image(&input_path)?;
    save_image(&sharpen3x3(&img.to_luma8()), &output_path, ImageFormat::Png)
}

#[pyfunction]
fn apply_edge_detection(input_path: String, output_path: String) -> PyResult<()> {
    let img = open_image(&input_path)?;
    let edge_detected_image_u16 = sobel_gradients(&img.to_luma8());

    let mut edge_detected_image_u8 = ImageBuffer::new(edge_detected_image_u16.width(), edge_detected_image_u16.height());
    for (x, y, pixel) in edge_detected_image_u16.enumerate_pixels() {
        let val = pixel[0] as u32;
        let scaled_val = (val * 255) / u16::MAX as u32;
        edge_detected_image_u8.put_pixel(x, y, Luma([scaled_val as u8]));
    }
    save_image(&edge_detected_image_u8, &output_path, ImageFormat::Png)
}

#[pyfunction]
fn apply_sepia(input_path: String, output_path: String) -> PyResult<()> {
    let img = open_image(&input_path)?;
    let sepia_image = map_colors(&img.to_rgba8(), |p| {
        let r = (p[0] as f32 * 0.393) + (p[1] as f32 * 0.769) + (p[2] as f32 * 0.189);
        let g = (p[0] as f32 * 0.349) + (p[1] as f32 * 0.686) + (p[2] as f32 * 0.168);
        let b = (p[0] as f32 * 0.272) + (p[1] as f32 * 0.534) + (p[2] as f32 * 0.131);
        Rgba([r.min(255.0) as u8, g.min(255.0) as u8, b.min(255.0) as u8, p[3]])
    });
    save_image(&sepia_image, &output_path, ImageFormat::Png)
}

// Add all the pyfunctions to the Python module
#[pymodule]
fn optimaimg(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(resize_image, m)?)?;
    m.add_function(wrap_pyfunction!(convert_to_grayscale, m)?)?;
    m.add_function(wrap_pyfunction!(rotate_image, m)?)?;
    m.add_function(wrap_pyfunction!(apply_blur, m)?)?;
    m.add_function(wrap_pyfunction!(apply_sharpen, m)?)?;
    m.add_function(wrap_pyfunction!(apply_edge_detection, m)?)?;
    m.add_function(wrap_pyfunction!(apply_sepia, m)?)?;
    Ok(())
}

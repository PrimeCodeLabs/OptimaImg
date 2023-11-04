use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use image::{open, ImageFormat, imageops::FilterType};

#[pyfunction]
fn resize_image(input_path: String, output_path: String, width: u32, height: u32) -> PyResult<()> {
    match open(&input_path) {
        Ok(img) => {
            // Resize the image to the specified dimensions using the Lanczos3 filter
            let resized_img = img.resize_exact(width, height, FilterType::Lanczos3);

            // Save the resized image to the output path
            match resized_img.save_with_format(&output_path, ImageFormat::Png) {
                Ok(_) => {
                    println!("Resized image and saved to {}", output_path);
                    Ok(())
                },
                Err(e) => {
                    println!("Failed to save resized image: {}", e);
                    Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to save resized image: {}", e)))
                }
            }
        },
        Err(e) => {
            println!("Failed to open image: {}", e);
            Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to open image: {}", e)))
        }
    }
}

#[pyfunction]
fn convert_to_grayscale(input_path: String, output_path: String) -> PyResult<()> {

    match open(&input_path) {
        Ok(img) => {
            let grayscale_img = img.to_luma8();

            match grayscale_img.save_with_format(&output_path, ImageFormat::Png) {
                Ok(_) => {
                    println!("Converted image to grayscale and saved to {}", output_path);
                    Ok(())
                },
                Err(e) => {
                    println!("Failed to save grayscale image: {}", e);
                    Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to save grayscale image: {}", e)))
                }
            }
        },
        Err(e) => {
            println!("Failed to open image: {}", e);
            Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to open image: {}", e)))
        }
    }
}

#[pymodule]
fn optimaimg(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_to_grayscale, m)?)?;
    m.add_function(wrap_pyfunction!(resize_image, m)?)?;
    
    Ok(())
}

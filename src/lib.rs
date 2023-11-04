use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use image::{open, ImageFormat};

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
    
    Ok(())
}

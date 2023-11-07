// Import necessary structs, traits, and functions from the image and imageproc crates
use image::{
    DynamicImage, ImageBuffer, ImageFormat, Luma, Rgba, RgbImage, Rgb, open, RgbaImage,
    imageops::FilterType,
    GenericImageView
};
use imageproc::{
    filter::gaussian_blur_f32,
    geometric_transformations::{rotate_about_center, Interpolation},
    gradients::sobel_gradients,
    map::map_colors,
};
use pyo3::{prelude::*, exceptions::PyIOError, exceptions::PyException, wrap_pyfunction};
use std::{path::Path, convert::TryInto, sync::mpsc};
use palette::{Hsv, Srgb, FromColor};

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
fn batch_resize_images(input_paths: Vec<String>, output_path: String, width: u32, height: u32) -> PyResult<()> {
    let (tx, rx) = mpsc::channel();

    // Use standard threading to process images in parallel
    input_paths.into_iter().enumerate().for_each(|(index, input_path)| {
        let tx = tx.clone();
        let output_path = output_path.clone(); // Clone output path for the move into the thread
        std::thread::spawn(move || {
            let result: Result<(), PyErr> = (|| {
                let img = open(Path::new(&input_path))
                    .map_err(|e| PyException::new_err(format!("Failed to open image: {}", e)))?;
                let resized_img = img.resize_exact(width, height, FilterType::Lanczos3);
                // Generate a unique filename for each output
                let file_name = Path::new(&input_path).file_stem()
                    .ok_or_else(|| PyException::new_err("Input path does not have a valid filename"))?
                    .to_str()
                    .ok_or_else(|| PyException::new_err("Failed to convert filename to string"))?;
                let output_file_path = Path::new(&output_path).join(format!("{}_{}.png", file_name, index));
                resized_img.save_with_format(output_file_path, ImageFormat::Png)
                    .map_err(|e| PyException::new_err(format!("Failed to save image: {}", e)))?;
                Ok(())
            })();
            tx.send(result).expect("Could not send over channel");
        });
    });

    // Drop the original transmitter so the receiver will close after all threads are done
    drop(tx);

    // Collect errors
    let mut errors: Vec<PyErr> = Vec::new();

    // Collect the results
    for received in rx {
        if let Err(e) = received {
            errors.push(e);
        }
    }

    // If there were any errors, return the first one
    if let Some(error) = errors.into_iter().next() {
        return Err(error);
    }

    Ok(())
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

fn sharpen_kernel() -> [[i32; 3]; 3] {
    [
        [0, -1, 0],
        [-1, 5, -1],
        [0, -1, 0]
    ]
}

fn apply_convolution(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let kernel = sharpen_kernel();
    let mut output_image: RgbImage = ImageBuffer::new(width, height);

    for y in 1..height as usize - 1 {
        for x in 1..width as usize - 1 {
            let mut r_total = 0;
            let mut g_total = 0;
            let mut b_total = 0;

            for ky in 0..3 {
                for kx in 0..3 {
                    let px = image.get_pixel((x + kx - 1) as u32, (y + ky - 1) as u32);
                    r_total += px[0] as i32 * kernel[ky][kx];
                    g_total += px[1] as i32 * kernel[ky][kx];
                    b_total += px[2] as i32 * kernel[ky][kx];
                }
            }

            let r = r_total.clamp(0, 255) as u8;
            let g = g_total.clamp(0, 255) as u8;
            let b = b_total.clamp(0, 255) as u8;
 
 
            // Inside the loop
            let x_u32: u32 = x.try_into().expect("Index out of u32 bounds");
            let y_u32: u32 = y.try_into().expect("Index out of u32 bounds");
            output_image.put_pixel(x_u32, y_u32, Rgb([r, g, b]));
        }
    }

    output_image
}

#[pyfunction]
fn apply_sharpen(input_path: String, output_path: String) -> PyResult<()> {
    let img = image::open(&input_path)
        .map_err(|e| PyErr::new::<PyIOError, _>(format!("Failed to open image: {}", e)))?
        .to_rgb8();
    let sharpened_img = apply_convolution(&img);

    sharpened_img.save(&output_path)
        .map_err(|e| PyErr::new::<PyIOError, _>(format!("Failed to save image: {}", e)))?;
    Ok(())
}

#[pyfunction]
fn apply_edge_detection(input_path: String, output_path: String) -> PyResult<()> {
    let img = open_image(&input_path)?;
    let edge_detected_image_u16 = sobel_gradients(&img.to_luma8());

    // Find the maximum gradient value in the image
    let max_val = edge_detected_image_u16.pixels().map(|p| p[0]).max().unwrap_or(0);

    let mut edge_detected_image_u8 = ImageBuffer::new(edge_detected_image_u16.width(), edge_detected_image_u16.height());
    for (x, y, pixel) in edge_detected_image_u16.enumerate_pixels() {
        // Scale based on the maximum gradient value instead of u16::MAX
        let scaled_val = if max_val > 0 {
            (pixel[0] as u32 * 255 / max_val as u32) as u8
        } else {
            0
        };
        edge_detected_image_u8.put_pixel(x, y, Luma([scaled_val]));
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

// Function to adjust the brightness of an image
// Increase brightness by adding to the pixel values
#[pyfunction]
fn adjust_brightness(input_path: String, output_path: String, value: i32) -> PyResult<()> {
    let img = open_image(&input_path)?;
    let adjusted_img = img.adjust_contrast(value as f32);
    save_image(&adjusted_img.to_rgba8(), &output_path, ImageFormat::Png)
}

// Function to change the contrast of an image
// Increase contrast by scaling pixel values to the power of the given factor
#[pyfunction]
fn adjust_contrast(input_path: String, output_path: String, factor: f32) -> PyResult<()> {
    let img = open_image(&input_path)?;
    let adjusted_img = img.adjust_contrast(factor);
    save_image(&adjusted_img.to_rgba8(), &output_path, ImageFormat::Png)
}

fn get_saturated_image(input_img: &DynamicImage, factor: f32) -> RgbaImage {
    let (width, height) = input_img.dimensions();
    let mut output_img = RgbaImage::new(width, height);

    for (x, y, image::Rgba(data)) in input_img.to_rgba8().enumerate_pixels() {
        // Convert u8 values to f32 for palette operations (normalize to 0..1 range)
        let srgb = Srgb::new(data[0] as f32 / 255.0, data[1] as f32 / 255.0, data[2] as f32 / 255.0);

        // Convert to HSV color space
        let hsv = Hsv::from_color(srgb);

        // Adjust the saturation
        let new_hsv = Hsv::new(hsv.hue, hsv.saturation * factor, hsv.value);

        // Convert back to sRGB color space
        let new_srgb = Srgb::from_color(new_hsv);

        // Denormalize from 0..1 range to 0..255 range
        let new_data = new_srgb.into_format::<u8>();


        // Put the new pixel in the output image
        output_img.put_pixel(x, y, Rgba([new_data.red, new_data.green as u8, new_data.blue as u8, data[3]]));
    }

    output_img
}

// Function to adjust the saturation of an image
// Increase saturation by scaling the color components relative to the luminance
#[pyfunction]
fn adjust_saturation(input_path: String, output_path: String, factor: f32) -> PyResult<()> {
    let img: DynamicImage = open_image(&input_path)?;
    let adjusted_img = get_saturated_image(&img, factor);
    save_image(&adjusted_img, &output_path, ImageFormat::Png)
}

// Function to adjust the hue of an image
// Adjust hue by rotating the hue value of each pixel by the given degrees
#[pyfunction]
fn adjust_hue(input_path: String, output_path: String, degrees: i32) -> PyResult<()> {
    let img = open_image(&input_path)?;
    let adjusted_img = img.huerotate(degrees);
    save_image(&adjusted_img.to_rgba8(), &output_path, ImageFormat::Png)
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
    m.add_function(wrap_pyfunction!(adjust_brightness, m)?)?;
    m.add_function(wrap_pyfunction!(adjust_contrast, m)?)?;
    m.add_function(wrap_pyfunction!(adjust_saturation, m)?)?;
    m.add_function(wrap_pyfunction!(adjust_hue, m)?)?;
    m.add_function(wrap_pyfunction!(batch_resize_images, m)?)?;

    Ok(())
}

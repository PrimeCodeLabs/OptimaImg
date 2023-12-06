use image::{
    DynamicImage, ImageBuffer, ImageFormat, Luma, Rgba, RgbImage, Rgb, open, RgbaImage,
    imageops::FilterType,
    GenericImageView, PixelWithColorType, Pixel
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
use image::io::Reader as ImageReader;
const TRANSPARENT_COLOR: Rgba<u8> = Rgba([0, 0, 0, 0]);

fn save_image<T>(image: &ImageBuffer<T, Vec<u8>>, output_path: &str, image_format: ImageFormat) -> PyResult<()>
where
    T: Pixel<Subpixel = u8> + PixelWithColorType + 'static,
{
    image
        .save_with_format(output_path, image_format)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to save image: {}", e)))
}

fn open_image(path: &str) -> PyResult<DynamicImage> {
    ImageReader::open(Path::new(path))?
        .decode()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to open image: {}", e)))
}
enum ColorSpace {
    Rgb,
    Hsv,
}

impl ColorSpace {
    fn from_str(s: &str) -> Result<ColorSpace, PyErr> {
        match s.to_lowercase().as_str() {
            "rgb" => Ok(ColorSpace::Rgb),
            "hsv" => Ok(ColorSpace::Hsv),
            _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid color space")),
        }
    }
}


fn convert_image_color_space(image: &RgbImage, input_space: ColorSpace, output_space: ColorSpace) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut output_img = RgbImage::new(width, height);

    for (x, y, rgb) in image.enumerate_pixels() {
        let srgb = match input_space {
            ColorSpace::Rgb => {
                Srgb::new(rgb[0] as f32 / 255.0, rgb[1] as f32 / 255.0, rgb[2] as f32 / 255.0)
            }
            ColorSpace::Hsv => {
                let hsv = Hsv::from_color(Srgb::new(rgb[0] as f32 / 255.0, rgb[1] as f32 / 255.0, rgb[2] as f32 / 255.0));
                Srgb::from_color(hsv)
            },
        };

        let final_rgb = match output_space {
            ColorSpace::Rgb => srgb,
            ColorSpace::Hsv => {
                let hsv: Hsv = Hsv::from_color(srgb);
                Srgb::from_color(hsv)
            },
        };

        // Convert back to image::Rgb
        let new_rgb = image::Rgb([
            (final_rgb.red * 255.0) as u8,
            (final_rgb.green * 255.0) as u8,
            (final_rgb.blue * 255.0) as u8
        ]);

        output_img.put_pixel(x, y, new_rgb);
    }

    output_img
}

/// Overlay an image onto another
/// - base_path: Path to the base image
/// - overlay_path: Path to the overlay image
/// - output_path: Path to save the composited image
/// - x: X-coordinate of the overlay position on the base image
/// - y: Y-coordinate of the overlay position on the base image
/// - alpha: Transparency of the overlay (0.0 to 1.0)
#[pyfunction]
fn overlay_images(base_path: String, overlay_path: String, output_path: String, x: u32, y: u32, alpha: f32) -> PyResult<()> {
    let mut base_img = open_image(&base_path)?.to_rgba8();
    let overlay_img = open_image(&overlay_path)?.to_rgba8();

    for (overlay_x, overlay_y, pixel) in overlay_img.enumerate_pixels() {
        let base_pixel = base_img.get_pixel_mut(x + overlay_x, y + overlay_y);
        let blended_pixel = blend_pixels(*base_pixel, *pixel, alpha);
        *base_pixel = blended_pixel;
    }

    save_image(&base_img, &output_path, ImageFormat::Png)
}

fn blend_pixels(base: Rgba<u8>, overlay: Rgba<u8>, alpha: f32) -> Rgba<u8> {
    // Blend two pixels based on alpha
    let blend = |base, overlay| ((1.0 - alpha) * base as f32 + alpha * overlay as f32) as u8;
    Rgba([
        blend(base[0], overlay[0]),
        blend(base[1], overlay[1]),
        blend(base[2], overlay[2]),
        255, // Assuming the alpha channel of the base image is always 255 (fully opaque)
    ])
}

/// Convert an image from one color space to another
/// - input_path: The path to the input image
/// - output_path: The path to save the converted image
/// - input_space: The color space of the input image
/// - output_space: The color space to convert the image to
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: convert_color_space("input.png", "output.png", "rgb", "hsv")
#[pyfunction]
fn convert_color_space(input_path: String, output_path: String, input_space: String, output_space: String) -> PyResult<()> {
    let img = open_image(&input_path)?.to_rgb8();
    let input_space = ColorSpace::from_str(&input_space)?;
    let output_space = ColorSpace::from_str(&output_space)?;

    let converted_img = convert_image_color_space(&img, input_space, output_space);
    save_image(&converted_img, &output_path, ImageFormat::Png)
}

/// Resize an image to the given width and height
/// - input_path: The path to the input image
/// - output_path: The path to save the resized image
/// - width: The width to resize the image to
/// - height: The height to resize the image to
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: resize_image("input.png", "output.png", 100, 100)
#[pyfunction]
fn resize_image(input_path: String, output_path: String, width: u32, height: u32) -> PyResult<()> {
    let img = open_image(&input_path)?;
    let resized_img = img.resize_exact(width, height, FilterType::Lanczos3);
    save_image(&resized_img.to_rgba8(), &output_path, ImageFormat::Png)
}

/// Resize a batch of images to the given width and height
/// - input_paths: A list of paths to the input images
/// - output_path: The path to save the resized images
/// - width: The width to resize the images to
/// - height: The height to resize the images to
/// - Returns: None
/// - Raises: IOError if an input image could not be opened or an output image could not be saved
/// - Example: batch_resize_images(["input1.png", "input2.png"], "output.png", 100, 100)
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

    drop(tx);

    let mut errors: Vec<PyErr> = Vec::new();

    for received in rx {
        if let Err(e) = received {
            errors.push(e);
        }
    }

    if let Some(error) = errors.into_iter().next() {
        return Err(error);
    }

    Ok(())
}

/// Convert an image to grayscale
/// - input_path: The path to the input image
/// - output_path: The path to save the grayscale image
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: convert_to_grayscale("input.png", "output.png")
#[pyfunction]
fn convert_to_grayscale(input_path: String, output_path: String) -> PyResult<()> {
    let img = open_image(&input_path)?;
    save_image(&img.to_luma8(), &output_path, ImageFormat::Png)
}

/// Rotate an image by the given number of degrees
/// - input_path: The path to the input image
/// - output_path: The path to save the rotated image
/// - degrees: The number of degrees to rotate the image by
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: rotate_image("input.png", "output.png", 90)
#[pyfunction]
fn rotate_image(input_path: String, output_path: String, degrees: f32) -> PyResult<()> {
    let dyn_img = open_image(&input_path)?;
    let img_buffer = dyn_img.to_rgba8();
    let radians = degrees.to_radians();
    let rotated_img = rotate_about_center(&img_buffer, radians, Interpolation::Bilinear, TRANSPARENT_COLOR);
    save_image(&rotated_img, &output_path, ImageFormat::Png)
}

/// Apply a Gaussian blur to an image
/// - input_path: The path to the input image
/// - output_path: The path to save the blurred image
/// - sigma: The standard deviation of the Gaussian blur
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: apply_blur("input.png", "output.png", 1.0)
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
 
            let x_u32: u32 = x.try_into().expect("Index out of u32 bounds");
            let y_u32: u32 = y.try_into().expect("Index out of u32 bounds");
            output_image.put_pixel(x_u32, y_u32, Rgb([r, g, b]));
        }
    }

    output_image
}

/// Apply a sharpening filter to an image
/// - input_path: The path to the input image
/// - output_path: The path to save the sharpened image
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: apply_sharpen("input.png", "output.png")
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

/// Apply an edge detection filter to an image
/// - input_path: The path to the input image
/// - output_path: The path to save the edge detected image
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: apply_edge_detection("input.png", "output.png")
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

/// Apply a sepia filter to an image
/// - input_path: The path to the input image
/// - output_path: The path to save the sepia image
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: apply_sepia("input.png", "output.png")
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

/// Apply a brightness adjustment to an image
/// - input_path: The path to the input image
/// - output_path: The path to save the adjusted image
/// - value: The amount to adjust the brightness by
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: adjust_brightness("input.png", "output.png", 50)
#[pyfunction]
fn adjust_brightness(input_path: String, output_path: String, value: i32) -> PyResult<()> {
    let img = open_image(&input_path)?;
    let adjusted_img = img.adjust_contrast(value as f32);
    save_image(&adjusted_img.to_rgba8(), &output_path, ImageFormat::Png)
}

/// Apply a contrast adjustment to an image
/// - input_path: The path to the input image
/// - output_path: The path to save the adjusted image
/// - factor: The amount to adjust the contrast by
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: adjust_contrast("input.png", "output.png", 1.5)
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

/// Apply a saturation adjustment to an image
/// - input_path: The path to the input image
/// - output_path: The path to save the adjusted image
/// - factor: The amount to adjust the saturation by
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: adjust_saturation("input.png", "output.png", 1.5)
#[pyfunction]
fn adjust_saturation(input_path: String, output_path: String, factor: f32) -> PyResult<()> {
    let img: DynamicImage = open_image(&input_path)?;
    let adjusted_img = get_saturated_image(&img, factor);
    save_image(&adjusted_img, &output_path, ImageFormat::Png)
}

/// Apply a hue adjustment to an image
/// - input_path: The path to the input image
/// - output_path: The path to save the adjusted image
/// - degrees: The number of degrees to adjust the hue by
/// - Returns: None
/// - Raises: IOError if the input image could not be opened or the output image could not be saved
/// - Example: adjust_hue("input.png", "output.png", 90)
#[pyfunction]
fn adjust_hue(input_path: String, output_path: String, degrees: i32) -> PyResult<()> {
    let img = open_image(&input_path)?;
    let adjusted_img = img.huerotate(degrees);
    save_image(&adjusted_img.to_rgba8(), &output_path, ImageFormat::Png)
}

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
    m.add_function(wrap_pyfunction!(convert_color_space, m)?)?;
    m.add_function(wrap_pyfunction!(overlay_images, m)?)?;
    Ok(())
}

import argparse
from image_processing import (
    convert_to_grayscale,
    resize_image,
    rotate_image,
    apply_blur,
    apply_sharpen,
    apply_edge_detection,
    apply_sepia,
    adjust_brightness,
    adjust_contrast,
    adjust_saturation,
    adjust_hue,
    batch_resize_images,
    convert_color_space,
    overlay_images,
)


class CustomHelpFormatter(argparse.HelpFormatter):
    def _metavar_formatter(self, action, default_metavar):
        if action.choices:
            # Return a function that formats the list of choices as a single string.
            def format_metavar(choice):
                return (", ".join(action.choices),)

            return format_metavar
        else:
            return super()._metavar_formatter(action, default_metavar)


def main():
    parser = argparse.ArgumentParser(
        description="optimaimg CLI", formatter_class=CustomHelpFormatter
    )
    subparsers = parser.add_subparsers(dest="command")

    # Resize command
    resize_parser = subparsers.add_parser("resize", help="Resize an image")
    resize_parser.add_argument("input_path", type=str, help="Input image path")
    resize_parser.add_argument("output_path", type=str, help="Output image path")
    resize_parser.add_argument("width", type=int, help="New width")
    resize_parser.add_argument("height", type=int, help="New height")

    # Convert to grayscale command
    grayscale_parser = subparsers.add_parser(
        "grayscale", help="Convert an image to grayscale"
    )
    grayscale_parser.add_argument("input_path", type=str, help="Input image path")
    grayscale_parser.add_argument("output_path", type=str, help="Output image path")

    # Rotate image command
    rotate_parser = subparsers.add_parser("rotate", help="Rotate an image")
    rotate_parser.add_argument("input_path", type=str, help="Input image path")
    rotate_parser.add_argument("output_path", type=str, help="Output image path")
    rotate_parser.add_argument(
        "degrees", type=float, help="Degrees to rotate the image"
    )

    # Blur command
    blur_parser = subparsers.add_parser("blur", help="Apply blur to an image")
    blur_parser.add_argument("input_path", type=str, help="Input image path")
    blur_parser.add_argument("output_path", type=str, help="Output image path")
    blur_parser.add_argument("sigma", type=float, help="Sigma for blur")

    # Sharpen command
    sharpen_parser = subparsers.add_parser("sharpen", help="Sharpen an image")
    sharpen_parser.add_argument("input_path", type=str, help="Input image path")
    sharpen_parser.add_argument("output_path", type=str, help="Output image path")

    # Edge detection command
    edge_detection_parser = subparsers.add_parser(
        "edge_detection", help="Apply edge detection to an image"
    )
    edge_detection_parser.add_argument("input_path", type=str, help="Input image path")
    edge_detection_parser.add_argument(
        "output_path", type=str, help="Output image path"
    )

    # Sepia command
    sepia_parser = subparsers.add_parser("sepia", help="Apply sepia tone to an image")
    sepia_parser.add_argument("input_path", type=str, help="Input image path")
    sepia_parser.add_argument("output_path", type=str, help="Output image path")

    # Brightness command
    brightness_parser = subparsers.add_parser(
        "brightness", help="Adjust the brightness of an image"
    )
    brightness_parser.add_argument("input_path", type=str, help="Input image path")
    brightness_parser.add_argument("output_path", type=str, help="Output image path")
    brightness_parser.add_argument(
        "brightness", type=float, help="Brightness adjustment level"
    )

    # Contrast command
    contrast_parser = subparsers.add_parser(
        "contrast", help="Adjust the contrast of an image"
    )
    contrast_parser.add_argument("input_path", type=str, help="Input image path")
    contrast_parser.add_argument("output_path", type=str, help="Output image path")
    contrast_parser.add_argument(
        "contrast", type=float, help="Contrast adjustment level"
    )

    # Saturation command
    saturation_parser = subparsers.add_parser(
        "saturation", help="Adjust the saturation of an image"
    )
    saturation_parser.add_argument("input_path", type=str, help="Input image path")
    saturation_parser.add_argument("output_path", type=str, help="Output image path")
    saturation_parser.add_argument(
        "saturation", type=float, help="Saturation adjustment level"
    )

    # Hue command
    hue_parser = subparsers.add_parser("hue", help="Adjust the hue of an image")
    hue_parser.add_argument("input_path", type=str, help="Input image path")
    hue_parser.add_argument("output_path", type=str, help="Output image path")
    hue_parser.add_argument("hue", type=float, help="Hue adjustment level")

    # Batch resize command
    batch_resize_parser = subparsers.add_parser(
        "batch_resize", help="Batch resize images"
    )
    batch_resize_parser.add_argument(
        "input_paths", type=str, nargs="+", help="List of input image paths"
    )
    batch_resize_parser.add_argument(
        "output_path", type=str, help="Output directory path"
    )
    batch_resize_parser.add_argument("width", type=int, help="Width to resize to")
    batch_resize_parser.add_argument("height", type=int, help="Height to resize to")

    # Color space conversion command
    color_space_parser = subparsers.add_parser(
        "convert_color_space", help="Convert image color space"
    )
    color_space_parser.add_argument("input_path", type=str, help="Input image path")
    color_space_parser.add_argument("output_path", type=str, help="Output image path")
    color_space_parser.add_argument(
        "input_space", type=str, help="Input color space (e.g., 'rgb', 'hsv')"
    )
    color_space_parser.add_argument(
        "output_space", type=str, help="Output color space (e.g., 'rgb', 'hsv')"
    )

    # Overlay images command
    overlay_parser = subparsers.add_parser(
        "overlay", help="Overlay an image onto another"
    )
    overlay_parser.add_argument("base_path", type=str, help="Base image path")
    overlay_parser.add_argument("overlay_path", type=str, help="Overlay image path")
    overlay_parser.add_argument("output_path", type=str, help="Output image path")
    overlay_parser.add_argument("x", type=int, help="X-coordinate of overlay position")
    overlay_parser.add_argument("y", type=int, help="Y-coordinate of overlay position")
    overlay_parser.add_argument("alpha", type=float, help="Transparency of the overlay")

    args = parser.parse_args()

    # Process commands
    if args.command == "resize":
        resize_image(args.input_path, args.output_path, args.width, args.height)
    elif args.command == "grayscale":
        convert_to_grayscale(args.input_path, args.output_path)
    elif args.command == "rotate":
        rotate_image(args.input_path, args.output_path, args.degrees)
    elif args.command == "blur":
        apply_blur(args.input_path, args.output_path, args.sigma)
    elif args.command == "sharpen":
        apply_sharpen(args.input_path, args.output_path)
    elif args.command == "edge_detection":
        apply_edge_detection(args.input_path, args.output_path)
    elif args.command == "sepia":
        apply_sepia(args.input_path, args.output_path)
    elif args.command == "brightness":
        adjust_brightness(args.input_path, args.output_path, args.brightness)
    elif args.command == "contrast":
        adjust_contrast(args.input_path, args.output_path, args.contrast)
    elif args.command == "saturation":
        adjust_saturation(args.input_path, args.output_path, args.saturation)
    elif args.command == "hue":
        adjust_hue(args.input_path, args.output_path, args.hue)
    elif args.command == "batch_resize":
        batch_resize_images(args.input_paths, args.output_path, args.width, args.height)
    elif args.command == "convert_color_space":
        convert_color_space(
            args.input_path, args.output_path, args.input_space, args.output_space
        )
    elif args.command == "overlay":
        overlay_images(
            args.base_path,
            args.overlay_path,
            args.output_path,
            args.x,
            args.y,
            args.alpha,
        )
    elif args.command is None:
        parser.print_help()
        return

    print(f"Command '{args.command}' executed successfully.")


if __name__ == "__main__":
    main()

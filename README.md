# Ray Tracing in Rust

This is a simple ray tracer implemented in Rust, following the structure and concepts from Peter Shirley's book "[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)".

## Description

The program generates a PPM image file of a scene containing several spheres with different materials (Lambertian and Metal). It demonstrates basic ray tracing principles such as ray-sphere intersection, material handling, reflections, and anti-aliasing through random sampling.

## How to Build and Run

You will need to have the Rust toolchain (including `cargo`) installed.

1.  **Build the project:**
    ```bash
    cargo build
    ```

2.  **Run the project and generate the image:**
    The program writes the image data in PPM format to standard output. To save it as a file, you can redirect the output:
    ```bash
    cargo run --release > image.ppm
    ```
    Running in `--release` mode is highly recommended for performance. The rendering process will show a progress indicator on the standard error stream.

3.  **View the image:**
    The output file `image.ppm` can be viewed with an image viewer that supports the PPM format, such as GIMP, ImageMagick, or other online viewers.

## Project Structure

The source code is organized into several modules within the `src/` directory:

-   `main.rs`: The main application entry point, handles scene setup and rendering loop.
-   `vec3.rs`: Defines a 3D vector struct (`Vec3`) used for points, colors, and directions.
-   `ray.rs`: Defines the `Ray` struct.
-   `hittable.rs` & `hittable_list.rs`: Define the `Hittable` trait for objects that can be intersected by rays, and a list to hold them.
-   `sphere.rs`: Implements the `Sphere` object as a `Hittable`.
-   `material.rs`: Defines the `Material` trait and concrete materials like `Lambertian` (diffuse) and `Metal` (reflective).
-   `camera.rs`: Implements a camera to generate rays for the scene.
-   `color.rs`: Contains utility functions for writing color values to the output image.

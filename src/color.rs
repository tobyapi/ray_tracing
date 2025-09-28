use crate::vec3::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;
    // gamma = 2.0
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as i32,
        (256.0 * g.clamp(0.0, 0.999)) as i32,
        (256.0 * b.clamp(0.0, 0.999)) as i32
    )
}

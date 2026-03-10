use std::io::Write;
use crate::vec3::Vec3;
use crate::Common;
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.get_x();
    let mut g = pixel_color.get_y();
    let mut b = pixel_color.get_z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    writeln!(
        out,
        "{} {} {}",
        (256.0 * Common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * Common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * Common::clamp(b, 0.0, 0.999)) as i32,
    )
        .expect("writing color");
}
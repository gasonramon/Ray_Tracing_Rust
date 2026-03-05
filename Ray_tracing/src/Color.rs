use std::io::Write;
use crate::vec3::Vec3;
pub type color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: color) {
    let r = (255.999 * pixel_color.get_x()) as i32;
    let g = (255.999 * pixel_color.get_y()) as i32;
    let b = (255.999 * pixel_color.get_z()) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
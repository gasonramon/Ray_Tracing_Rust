mod color;
mod ray;
mod vec3;
mod Hitteble;

use std::io;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.get_origin() - *center;
    let a = vec3::dot(ray.get_direction(), ray.get_direction());
    let b = 2.0 * vec3::dot(oc, ray.get_direction());
    let c = vec3::dot(oc,oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - f64::sqrt(discriminant)) / (2.0 * a)
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = vec3::unit_vector(r.get_position(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.get_x() + 1.0, n.get_y() + 1.0, n.get_z() + 1.0);
    }
    let unit_direction = vec3::unit_vector(r.get_direction());
    let t = 0.5 * (unit_direction.get_y() + 1.0 );
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}



fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0,0.0,focal_length);

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", i);
        for j in (0..IMAGE_WIDTH) {
            let u = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = i as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(origin,
                             lower_left_corner + u * horizontal + v * vertical - origin);

            let pixel_color = ray_color(&r);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }
    eprint!("\nDone\n");
}

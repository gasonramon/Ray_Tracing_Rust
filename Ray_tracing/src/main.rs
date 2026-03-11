mod color;
mod camera;
mod ray;
mod vec3;
mod Hitteble;
mod sphere;
mod Hitteble_list;
mod Common;
mod material;

use std::io;
use std::rc::Rc;


use camera::Camera;
use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};
use material::{Lambertian, Metal, Dielectric, };
use Hitteble::{Hitrecord, Hittable};
use Hitteble_list::HittableList;
use sphere::Sphere;

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

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec = Hitrecord::new();
    if world.hit(r, 0.001, Common::INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        if rec.mat.as_ref().unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = vec3::unit_vector(r.get_direction());
    let t = 0.5 * (unit_direction.get_y() + 1.0 );
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)

}



fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    //World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    //Camera
    let position = Point3::new(-2.0, 2.0, 1.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(position,lookat, vup, 20.0, ASPECT_RATIO);

    //render
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
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (j as f64 + Common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (i as f64 + Common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone\n");
}

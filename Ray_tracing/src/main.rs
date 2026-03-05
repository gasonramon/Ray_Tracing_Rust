mod Color;
mod vec3;

use std::io;

use Color::color;

fn main() {

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", i);
        for j in (0..IMAGE_WIDTH) {
            let r = i as f64 / IMAGE_WIDTH as f64;
            let g = j as f64 / IMAGE_HEIGHT as f64;
            let b = 0.25;

            let pixel_color = color::new(r, g, b);
            Color::write_color(&mut io::stdout(), pixel_color);
        }
    }
    eprint!("\nDone\n");
}

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use raytracer::*;
use vec3::{Vec3, Vec3F32};


fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;
    let image_height_temp = (image_width as f64 / aspect_ratio) as i32;
    let image_height = match image_height_temp {
        i32::MIN..=1_i32 => 1,
        2_i32..=i32::MAX => image_height_temp,

    };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::with(0.0, 0.0, 0.0);

    let viewport_u = Vec3::with(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::with(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u.div(image_width as f32);
    let pixel_delta_v = viewport_v.div(image_height as f32);

    let viewport_upper_left = camera_center
                                        .sub(Vec3::with(0.0, 0.0, focal_length))
                                        .sub(viewport_u.div(2.0))
                                        .sub(viewport_v.div(2.0));
    let pixel_00_loc = viewport_upper_left.add((pixel_delta_u.add(pixel_delta_v)).div(2.0));

    let config = Config {
        image_width, 
        image_height,
        pixel_delta_u,
        pixel_delta_v,
        pixel_00_loc,
        camera_center,
    };

    let mut stream = BufWriter::new(
        OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("src/image.ppm")
        .unwrap()
    );

    let content = format!("P3\n{} {}\n{}\n", image_width, image_height, 255);
    stream.write(content.as_bytes()).unwrap();

    render(&mut stream, config);

    stream.flush().unwrap();
}



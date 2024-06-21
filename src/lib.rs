use std::fs::File;
use log::info;
use ray::Ray;
use vec3::{Vec3, Vec3F32};
use std::io::BufWriter;

use color::OColor;
mod ray;

pub struct Config {
    pub image_width: i32,
    pub image_height: i32,
    pub pixel_delta_u: Vec3<f32>,
    pub pixel_delta_v: Vec3<f32>,
    pub pixel_00_loc: Vec3<f32>,
    pub camera_center: Vec3<f32>,
} 

pub fn render(stream: &mut BufWriter<File>, config: Config) {
    for j in 0..config.image_height {
        info!("Scanlines remaining: {}", config.image_height - j);
        for i in 0..config.image_width {
            let pixel_center = config.pixel_00_loc
                                .add(config.pixel_delta_u.mul(i as f32))
                                .add(config.pixel_delta_v.mul(j as f32));
            let ray_direction = pixel_center.sub(config.camera_center);
            let r = Ray::with(config.camera_center, ray_direction);

            let pixel_color = ray_color(r);
            pixel_color.write_color(stream);
        }
    }
    info!("Done.");
}

fn hit_sphere(center: Vec3<f32>, radius: f32, ray: &Ray) -> bool {
    let oc = center.sub(ray.orig);

    let a = ray.dir.dot_product(ray.dir);
    let b = -2.0 * ray.dir.dot_product(oc);
    let c = oc.dot_product(oc) - radius*radius;
    
    let discriminant = b*b - 4.0*a*c;
    discriminant >= 0.0
}

fn ray_color(ray: Ray) -> OColor {
    if hit_sphere(Vec3::with(0.0, 0.0, -1.0), 0.5, &ray) {
        return OColor::with(255, 0, 0);
    }

    let unit_direction = ray.dir.unit_vec();
    let a = (unit_direction.e[1] + 1.0) * 0.5;   

    let rgb = (1.0 * (1.0 - a) * 255.999) as u8;
    let c1 = OColor::with(rgb, rgb, rgb);

    let r = (0.5 * a * 255.999) as u8;
    let g = (0.7 * a * 255.999) as u8;
    let b = (1.0 * a * 255.999) as u8;
    let c2 = OColor::with(r, g, b);
    c1.add(c2)
}
use rand::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod vector;
use vector::Vec3;

mod camera;
use camera::Camera;

mod ray;
use ray::{color, Ray};

mod hittable;
use hittable::{HitRecord, Hittable, HittableList};

mod materials;
use materials::{Dielectic, Lambertian, Material, Metal};

mod shapes;
use shapes::Sphere;

use std::f32;

mod scenes;
use scenes::gen_cover;

mod utils;
use utils::*;

fn main() {
    let path = Path::new("image.ppm");
    let nx = 1280;
    let ny = 720;
    let ns = 10;
    let mut arr = vec![format!("P3\n{} {}\n255", nx, ny)];

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );

    let mut rng = thread_rng();

    let world = gen_cover();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u: f32 = (i as f32 + rand(&mut rng)) / nx as f32;
                let v: f32 = (j as f32 + rand(&mut rng)) / ny as f32;
                let r = camera.get_ray(u, v);

                col += color(r, &world, 0);
            }
            col /= ns as f32;
            let ir = (255.99 * col.x.sqrt()) as u32;
            let ig = (255.99 * col.y.sqrt()) as u32;
            let ib = (255.99 * col.z.sqrt()) as u32;
            arr.push(format!("{} {} {}", ir, ig, ib));
        }
    }

    let output = arr.join("\n");
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why.description()),
        Ok(file) => file,
    };

    match file.write_all(output.as_bytes()) {
        Err(why) => panic!("Could not write bytes {}", why.description()),
        Ok(_) => println!("Success!"),
    }
}

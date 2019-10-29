use crate::Vec3;
use crate::{Hittable, HittableList};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
    pub fn new() -> Ray {
        Ray {
            origin: Vec3::zero(),
            direction: Vec3::zero(),
        }
    }
}

pub fn color(r: Ray, world: &HittableList, depth: i32) -> Vec3 {
    match world.hit(&r, 0.001, std::f32::MAX) {
        Some(res) => {
            let mut scattered = Ray::new();
            let mut attenuation = Vec3::zero();

            if depth < 50
                && res
                    .material
                    .scatter(&r, &res, &mut attenuation, &mut scattered)
            {
                attenuation * color(scattered, world, depth + 1)
            } else {
                Vec3::zero()
            }
        }
        None => {
            let unit_direction = Vec3::unit_vector(r.direction);
            let t = 0.5 * (unit_direction.y) + 1.0;
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            } * (1.0 - t)
                + Vec3 {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                } * t
        }
    }
}

use rand::prelude::*;
use crate::Vec3;

pub fn rand(r: &mut ThreadRng) -> f32 {
    r.gen()
}

pub fn nrand() -> f32 {
    thread_rng().gen()
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = thread_rng();
    loop {
        p = Vec3{
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        } * 2.0 - Vec3::from_one(1.0);


        if !(p.norm() >= 1.0) {
            break;
        }        
    }
    p
}
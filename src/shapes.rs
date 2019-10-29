use crate::Material;
use crate::Vec3;
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

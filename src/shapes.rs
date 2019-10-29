use crate::Vec3;
use std::sync::Arc;
use crate::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

use crate::Ray;
use crate::Vec3;
use crate::Sphere;
use crate::Material;
use std::sync::Arc;

pub struct HitableList {
    objects: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list_size: usize) -> Self {
        HitableList {
            objects: Vec::with_capacity(list_size),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hitable>) {
        self.objects.push(obj)
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for i in &self.objects {
            let temp = i.hit(r, t_min, closest_so_far);
            match temp {
                Some(rec) => {
                    closest_so_far = rec.t;
                    hit_anything = Some(rec);
                },
                None => {},
            }
        }
        hit_anything
    }
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = ( -b - (b*b - a*c).sqrt() ) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord{t: temp, p: p, normal: (p - self.center) / self.radius, material: Arc::clone(&self.material)});
            }
            let temp = (-b + (b*b - a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord{t: temp, p: p, normal: (p - self.center) / self.radius, material: Arc::clone(&self.material)});
            }
        }
        return None;
    }
}

use crate::utils::{nrand, random_in_unit_sphere};
use crate::HitRecord;
use crate::Ray;
use crate::Vec3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray {
            origin: rec.p,
            direction: target - rec.p,
        };
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub roughness: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, r: f32) -> Self {
        Metal {
            albedo,
            roughness: f32::min(r, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(Vec3::unit_vector(r_in.direction), rec.normal);
        *scattered = Ray {
            origin: rec.p,
            direction: reflected + random_in_unit_sphere() * self.roughness,
        };
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction, &rec.normal) > 0.0
    }
}

pub struct Dielectic {
    pub ref_idx: f32,
}

impl Dielectic {
    pub fn new(index: f32) -> Dielectic {
        Dielectic { ref_idx: index }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * Vec3::dot(&v, &n) * 2.0
}
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = Vec3::unit_vector(*v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = (uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt();
        return true;
    }
    false
}

impl Material for Dielectic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal: Vec3;
        let reflected = reflect(r_in.direction, rec.normal);
        let ni_over_nt: f32;
        let reflect_prob: f32;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted = Vec3::zero();
        let cosine = if Vec3::dot(&r_in.direction, &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            self.ref_idx * Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.length()
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            -Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.length()
        };
        if refract(&r_in.direction, &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            *scattered = Ray {
                origin: rec.p,
                direction: reflected,
            };
            reflect_prob = 1.0;
        }

        if nrand() < reflect_prob {
            *scattered = Ray {
                origin: rec.p,
                direction: reflected,
            };
        } else {
            *scattered = Ray {
                origin: rec.p,
                direction: refracted,
            };
        }
        true
    }
}

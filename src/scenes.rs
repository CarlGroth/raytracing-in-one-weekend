use crate::nrand;
use crate::HittableList;
use crate::Vec3;
use crate::{Dielectic, Lambertian, Metal, Sphere};
use std::sync::Arc;

pub fn gen_cover() -> HittableList {
    let n = 500;
    let mut list = HittableList::new(n + 1);

    list.add(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    }));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = nrand();
            let center = Vec3::new(a as f32 + 0.9 * nrand(), 0.2, b as f32 + 0.9 * nrand());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    list.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Lambertian::new(Vec3::new(nrand(), nrand(), nrand()))),
                    }));
                } else if choose_mat < 0.95 {
                    list.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + nrand()),
                                0.5 * (1.0 + nrand()),
                                0.5 * (1.0 + nrand()),
                            ),
                            0.5 * nrand(),
                        )),
                    }));
                } else {
                    list.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectic::new(1.5)),
                    }));
                }
            }
        }
    }
    list.add(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectic::new(1.5)),
    }));
    list.add(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    }));
    list.add(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    }));
    list
}

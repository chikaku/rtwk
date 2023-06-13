use std::rc::Rc;

use crate::{hit::Hittable, hit::Record, material::Material, vec3::Point3, vec3::Vec3};

pub struct Sphere {
    pub center: Point3,             // 中心点
    pub radius: f64,                // 半径
    pub material: Rc<dyn Material>, // 材质
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

unsafe impl Sync for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, min: f64, max: f64) -> Option<Record> {
        let oc = r.origin() - &self.center;

        let a = r.direction().length_square();
        let half_b = Vec3::dot(&oc, r.direction());
        let c = oc.length_square() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < min || max < root {
            root = (-half_b + sqrtd) / a;
            if root < min || max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = Vec3::dot(r.direction(), &outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };

        Some(Record {
            p,
            t: root,
            normal,
            front_face,
            material: self.material.clone(),
        })
    }
}

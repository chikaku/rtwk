use std::rc::Rc;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Record {
    pub p: Point3, // 撞击点
    pub t: f64,    // 光源到到撞击对象的距离

    pub normal: Vec3,     // 法线
    pub front_face: bool, // true 表示法线向外 false 表示法线向内

    pub material: Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<Record>;
}

pub struct HittableList<T> {
    objects: Vec<T>,
}

impl<T> HittableList<T> {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object)
    }
}

impl<T> Default for HittableList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, min: f64, max: f64) -> Option<Record> {
        let mut closest = max;
        let mut rec = None;

        for object in &self.objects {
            if let Some(hit_rec) = object.hit(ray, min, closest) {
                closest = hit_rec.t;
                rec = Some(hit_rec);
            }
        }

        rec
    }
}

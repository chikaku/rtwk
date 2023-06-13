use crate::vec3::Point3;

pub struct Ray {
    pub origin: Point3,
    pub dir: Point3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Point3) -> Self {
        Self { origin, dir }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Point3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }
}

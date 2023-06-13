use rand::Rng;

use crate::{
    hit::Record,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Material {
    // 材质的散射
    fn scatter(&self, rin: &Ray, rec: &Record) -> Option<(Ray, Color)>;
}

// 漫反射材质
pub struct Lambertian {
    albedo: Color, // 反射率
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &Record) -> Option<(Ray, Color)> {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        let scatter = Ray::new(rec.p, scatter_dir);
        Some((scatter, self.albedo))
    }
}

pub struct Metal {
    albedo: Color, // 反射率
    fuzz: f64,     // 模糊度
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, rin: &Ray, rec: &Record) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&rin.direction().unit(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if Vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

// 电介质
pub struct Dielectric {
    ir: f64, // 折射率
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    // Schlick's approximation
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, rin: &Ray, rec: &Record) -> Option<(Ray, Color)> {
        let refracttion_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let dir = rin.direction().unit();
        let cos_theta = Vec3::dot(&-dir, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // 当光线处于折射率较高的介质, 可能无法产生折射, 这种情况直接反射
        let cannot_refract = refracttion_ratio * sin_theta > 1.0;

        // 不是很懂...
        let rn = rand::thread_rng().gen_range(0.0..1.0);
        let rn = self.reflectance(cos_theta, refracttion_ratio) > rn;

        let dir = if cannot_refract || rn {
            Vec3::reflect(&dir, &rec.normal)
        } else {
            Vec3::refract(&dir, &rec.normal, refracttion_ratio)
        };

        let scattered = Ray::new(rec.p, dir);
        let attenuation = Color::new(1.0, 1.0, 1.0);
        Some((scattered, attenuation))
    }
}

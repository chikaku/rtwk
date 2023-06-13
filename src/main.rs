#![feature(portable_simd)]
use std::rc::Rc;

use anyhow::Result;
use rand::Rng;
// use rayon::prelude::*;
// use std::simd::Simd;

use rtwk::{
    camera::Camera,
    hit::Hittable,
    hit::HittableList,
    image::Image,
    material::{Dielectric, Lambertian, Material, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3::Point3,
    vec3::{Color, Vec3},
};

fn main() -> Result<()> {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let mut img = Image::open("output.ppm", image_width, image_height)?;

    let cam = new_camera(aspect_ratio);
    let world = random_scene();

    // 光线反射深度
    let max_depth = 50;

    // 多重采样抗锯齿
    let samples_per_pixel = 500;

    img.write_color_with(samples_per_pixel, |i, j| {
        let mut r = rand::thread_rng();
        (0..samples_per_pixel).fold(Color::new(0.0, 0.0, 0.0), |acc, _| {
            let u = (i as f64 + r.gen_range(0.0..1.0)) / (image_width - 1) as f64;
            let v = (j as f64 + r.gen_range(0.0..1.0)) / (image_height - 1) as f64;
            let r = cam.ray(u, v);
            acc + ray_color(&r, &world, max_depth)
        })

        // FIXME: illegal hardware instruction
        // let color = (0..samples_per_pixel)
        //     .into_par_iter()
        //     .map(|_| {
        //         let mut r = rand::thread_rng();
        //         let u = (i as f64 + r.gen_range(0.0..1.0)) / (image_width - 1) as f64;
        //         let v = (j as f64 + r.gen_range(0.0..1.0)) / (image_height - 1) as f64;
        //         let r = cam.ray(u, v);
        //         let color = ray_color(&r, &world, max_depth);
        //         Simd::from_array([color.x(), color.y(), color.z(), 0.0])
        //     })
        //     .sum::<Simd<f64, 4>>()
        //     .to_array();

        // Color::new(color[0], color[1], color[2])
    })
}

fn ray_color<H: Hittable>(r: &Ray, world: &H, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // 命中物体后在对应材质上散射
    if let Some(rec) = world.hit(r, 0.001, f64::MAX) {
        return match rec.material.as_ref().scatter(r, &rec) {
            Some((scattered, attenuation)) => attenuation * ray_color(&scattered, world, depth - 1),
            None => Color::new(0.0, 0.0, 0.0),
        };
    }

    let unit = r.direction().unit();
    let t = 0.5 * (unit.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn new_camera(aspect_ratio: f64) -> Camera {
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let dist_to_focus = 10.0;

    Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    )
}

fn random_scene() -> HittableList<Sphere> {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(ground);

    let mat1 = Rc::new(Dielectric::new(1.5));
    let sp1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    world.add(sp1);

    let mat2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let sp2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    world.add(sp2);

    let mat3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sp3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);
    world.add(sp3);

    let mut rng = rand::thread_rng();
    let mut rand_double = || rng.gen_range(0.0..1.0);

    for a in -11..11 {
        for b in -11..11 {
            let (a, b) = (a as f64, b as f64);

            let center = Point3::new(a + 0.9 * rand_double(), 0.2, b + 0.9 * rand_double());

            let p = Point3::new(4.0, 0.2, 0.0);
            if (center - p).length() > 0.9 {
                let material: Rc<dyn Material> = match rand_double() {
                    // 漫反射材质
                    n if (0.0..0.60).contains(&n) => {
                        let c1 = Color::random();
                        let c2 = Color::random();
                        let albedo = c1 * c2;
                        Rc::new(Lambertian::new(albedo))
                    }
                    // 金属材质
                    n if (0.60..0.85).contains(&n) => {
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = rand_double() / 2.0;
                        Rc::new(Metal::new(albedo, fuzz))
                    }
                    // 玻璃材质
                    n if (0.85..1.0).contains(&n) => Rc::new(Dielectric::new(1.5)),
                    _ => unreachable!(),
                };

                let radius = 0.2;
                world.add(Sphere::new(center, radius, material));
            }
        }
    }

    world
}

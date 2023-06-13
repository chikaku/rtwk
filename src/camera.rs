use std::f64::consts::PI;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    origin: Point3,            // 原点
    lower_left_corner: Point3, // 视口左下角坐标
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64, // 透镜半径

    u: Vec3,
    v: Vec3,
    #[allow(dead_code)]
    w: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,  // 相机位置
        lookat: Point3,    // 相机观察点
        vup: Vec3,         // 相机垂直向上向量 view-up vector
        vfov: f64,         // 表示垂直视角角度
        aspect_ratio: f64, // 长宽比
        aperture: f64,     // 光圈
        focus_dist: f64,   //对焦距离
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        // 视口高度 [-1.0, +1.0]
        let viewport_height = 2.0 * h;

        // 视口宽度, 用高度和长宽比计算
        let viewport_weight = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(&vup, &w).unit();
        let v = Vec3::cross(&w, &u);

        let lens_radius = aperture / 2.0;

        // 相机原点
        let origin = lookfrom;

        // 视口的水平宽度向量, 相当于左下角到右下角
        let horizontal = focus_dist * viewport_weight * u;

        // 视口的垂直宽度向量, 相当于左下角到左上角
        let vertical = focus_dist * viewport_height * v;

        // 左下角向量
        let v = (horizontal / 2.0) + (vertical / 2.0);
        let lower_left_corner = origin - v - focus_dist * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    // u 是投射点水平方向宽度比值
    // v 是投射点垂直方向高度比值
    // 用左下角向量跟水平与垂直方向上对应向量叠加加上远点就可以得到从原点到投射点的向量
    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_uint_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let v = s * self.horizontal + t * self.vertical;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + v - self.origin - offset,
        )
    }
}

// 角度转弧度
fn degrees_to_radians(degress: f64) -> f64 {
    degress * PI / 180.0
}

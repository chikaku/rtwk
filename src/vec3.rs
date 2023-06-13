use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    // 向量点积
    pub fn dot(a: &Self, b: &Self) -> f64 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
    }

    // 向量叉积
    pub fn cross(a: &Self, b: &Self) -> Self {
        Self::new(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0,
        )
    }

    // 反射 v 表示入射向量 n 表示法向量
    pub fn reflect(v: &Self, n: &Self) -> Self {
        *v - 2.0 * Self::dot(v, n) * (*n)
    }

    // 通过斯涅尔定律计算折射光
    // 折射 uv 表示入射光向量 n 表示法向量 etai_over_etat 是两介质的折射率之比
    pub fn refract(uv: &Self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = Self::dot(&-uv, n).min(1.0);
        let prep = etai_over_etat * (*uv + cos_theta * n);
        let parallel = -(1.0 - prep.length_square()).abs().sqrt() * n;
        prep + parallel
    }

    pub fn random() -> Self {
        Self::random_range(0.0, 1.0)
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);
        let z = rng.gen_range(min..max);
        Self(x, y, z)
    }

    pub fn random_in_unit_sphere() -> Self {
        // loop {
        //     let p = Self::random_range(-1.0, 1.0);
        //     if p.length_square() < 1.0 {
        //         break p;
        //     }
        // }
        for _ in 0..1000 {
            let p = Self::random_range(-1.0, 1.0);
            if p.length_square() < 1.0 {
                return p;
            }
        }

        panic!("too many loop");
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_in_hemishpere(normal: &Self) -> Self {
        let unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(normal, &unit_sphere) > 0.0 {
            unit_sphere
        } else {
            -unit_sphere
        }
    }

    // 随机 Z 平面上半径在单位长度以内的圆盘
    pub fn random_in_uint_disk() -> Self {
        let mut rng = rand::thread_rng();
        // loop {
        //     let p = Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.0);
        //     if p.length_square() < 1.0 {
        //         break p;
        //     }
        // }
        for _ in 0..1000 {
            let p = Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.0);
            if p.length_square() < 1.0 {
                return p;
            }
        }

        panic!("too many loop");
    }
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_square(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_square().sqrt()
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs * (-1.0)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        *self - *rhs
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += rhs * (-1.0);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

pub type Color = Vec3;

pub type Point3 = Vec3;

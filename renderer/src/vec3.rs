use crate::{rand_f64, rand_range_f64};

pub type Point3 = Vec3;

#[derive(Default, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn random() -> Self {
        Self {
            e: [rand_f64(), rand_f64(), rand_f64()],
        }
    }
    pub fn random_rng(min: f64, max: f64) -> Self {
        Self {
            e: [
                rand_range_f64(min, max),
                rand_range_f64(min, max),
                rand_range_f64(min, max),
            ],
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self[0].abs() < s) && (self[1].abs() < s) && (self[2].abs() < s)
    }

    pub fn change(&mut self, x: f64, y: f64, z: f64) {
        self[0] = x;
        self[1] = y;
        self[2] = z;
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[3])
    }
}
impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            e: [-self[0], -self[1], -self[2]],
        }
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self[0], -self[1], -self[2]],
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}
impl std::ops::Add<&Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Self {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl std::ops::Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}
impl std::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}
impl std::ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl std::ops::Sub<&Self> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        Self {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}
impl std::ops::Mul<&Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self::Output {
        Self {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl std::ops::Mul<Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl std::ops::Mul for &Vec3{
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}
impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl std::ops::Mul<&f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: &f64) -> Self::Output {
        Self {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl std::ops::Mul<&f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &f64) -> Self::Output {
        Vec3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [rhs[0] * self, rhs[1] * self, rhs[2] * self],
        }
    }
}
impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            e: [rhs[0] * self, rhs[1] * self, rhs[2] * self],
        }
    }
}

impl std::ops::Mul<Vec3> for &f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [rhs[0] * self, rhs[1] * self, rhs[2] * self],
        }
    }
}

impl std::ops::Mul<&Vec3> for &f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            e: [rhs[0] * self, rhs[1] * self, rhs[2] * self],
        }
    }
}

impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            e: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}
impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}
impl std::ops::Div<&f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: &f64) -> Self::Output {
        Vec3 {
            e: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}

impl std::ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self[0] /= rhs[0];
        self[1] /= rhs[1];
        self[2] /= rhs[2];
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

#[inline]
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

#[inline]
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    )
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.len()
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_rng(-1_f64, 1_f64);
        let lensq = p.len_squared();
        if lensq <= 1_f64 {
            return p / lensq.sqrt();
        }
    }
}

#[inline]
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand_range_f64(-1.0, 1.0), rand_range_f64(-1.0, 1.0), 0.0);
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

#[inline]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

#[inline]
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: &f64) -> Vec3 {
    let cos_theta = dot(&-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (n * cos_theta + uv);
    let r_out_parallel = -((1.0 - r_out_perp.len_squared()).abs()).sqrt() * n;
    r_out_perp + r_out_parallel
}

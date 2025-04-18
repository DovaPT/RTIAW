use crate::{
    color::Color,
    hittable::HitRecord,
    rand_f64,
    ray::Ray,
    vec3::{dot, random_unit_vector, reflect, refract, unit_vector},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertain {
    albedo: Color,
}

impl Lambertain {
    pub fn new(albedo: Color) -> Self{
        Self { albedo }
    }
}

impl Material for Lambertain {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = &(&rec.normal + random_unit_vector());

        if scatter_direction.near_zero() {
            scatter_direction = &rec.normal;
        }

        scattered.change(&rec.p, &scatter_direction);
        attenuation.change(self.albedo.x(), self.albedo.y(), self.albedo.z());

        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self{
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.dir, &rec.normal);
        let reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
        scattered.change(&rec.p, &reflected);
        attenuation.change(self.albedo.x(), self.albedo.y(), self.albedo.z());

        dot(scattered.direction(), &rec.normal) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric{
    pub fn new(refraction_index: f64) -> Self{
        Self { refraction_index }
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        attenuation.change(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = unit_vector(r_in.direction());

        let cos_theta = dot(&-&unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > rand_f64() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, &ri)
        };

        scattered.change(&rec.p, &direction);

        true
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

use core::f64;

use crate::{
    color::Color,
    hittable::HitRecord,
    rand_f64,
    ray::Ray,
    vec3::{dot, random_unit_vector, reflect, refract, unit_vector},
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertain { color: Color },
    Metal { color: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertain {
            color: Color::default(),
        }
    }
}

impl Material {
    pub fn scatter(
        self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertain { color: albedo } => {
                scatter_lambertian(&albedo, rec, attenuation, scattered)
            }
            Material::Metal {
                color: albedo,
                mut fuzz,
            } => scatter_metal(&albedo, &mut fuzz, r_in, rec, attenuation, scattered),
            Material::Dielectric { refraction_index } => {
                scatter_dielectric(&refraction_index, r_in, attenuation, rec, scattered)
            }
        }
    }
}

fn scatter_lambertian(
    albedo: &Color,
    rec: &HitRecord,
    attenuation: &mut Color,
    scattered: &mut Ray,
) -> bool {
    let mut scatter_direction = rec.normal + random_unit_vector();

    if scatter_direction.near_zero() {
        scatter_direction = rec.normal;
    }

    scattered.change(rec.p, scatter_direction);
    attenuation.change(*albedo);

    true
}

fn scatter_metal(
    albedo: &Color,
    fuzz: &mut f64,
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Color,
    scattered: &mut Ray,
) -> bool {
    if *fuzz > 1.0 {
        *fuzz = 1.0;
    }
    let reflected = reflect(&r_in.dir, &rec.normal);
    let reflected = unit_vector(&reflected) + (*fuzz * random_unit_vector());
    scattered.change(rec.p, reflected);
    attenuation.change(*albedo);

    dot(scattered.direction(), &rec.normal) > 0.0
}

fn scatter_dielectric(
    refraction_index: &f64,
    r_in: &Ray,
    attenuation: &mut Color,
    rec: &HitRecord,
    scattered: &mut Ray,
) -> bool {
    attenuation.change(Color { e: [1.0, 1.0, 1.0] });
    let ri = if rec.front_face {
        1.0 / *refraction_index
    } else {
        *refraction_index
    };
    let unit_direction = unit_vector(r_in.direction());

    let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
    let sin_theta = 1.0 - cos_theta * cos_theta;

    let cannot_refract = ri * sin_theta > 1.0;
    let direction = if cannot_refract || reflectance(cos_theta, ri) > rand_f64() {
        refract(&unit_direction, &rec.normal, &1.0)
    } else {
        refract(&unit_direction, &rec.normal, &ri)
    };

    scattered.change(rec.p, direction);

    true
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

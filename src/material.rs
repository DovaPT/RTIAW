use crate::{
    color::Color,
    hittable::HitRecord,
    rand_f64,
    ray::Ray,
    vec3::{
        dot,
        random_unit_vector,
        reflect,
        refract,
        unit_vector,
    },
};

#[derive(Clone, Copy)]
pub enum Mat {
    Metal { albedo: Color, fuzz: f64 },
    Lambertain { albedo: Color },
    Dielectric { refraction_index: f64 },
}

impl Mat {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Self::Lambertain { albedo } => {
                lambertain_scatter(albedo, r_in, rec, attenuation, scattered)
            }
            Self::Metal { albedo, fuzz } => {
                metal_scatter(albedo, *fuzz, r_in, rec, attenuation, scattered)
            }
            Self::Dielectric { refraction_index } => {
                dielectric_scatter(*refraction_index, r_in, rec, attenuation, scattered)
            }
        }
    }
}
fn lambertain_scatter(
    albedo: &Color,
    _r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Color,
    scattered: &mut Ray,
) -> bool {
    let mut scatter_direction = &(rec.normal + random_unit_vector());

    if scatter_direction.near_zero() {
        scatter_direction = &rec.normal;
    }

    scattered.change(&rec.p, scatter_direction);
    attenuation.change(albedo.x(), albedo.y(), albedo.z());

    true
}
fn metal_scatter(
    albedo: &Color,
    fuzz: f64,
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Color,
    scattered: &mut Ray,
) -> bool {
    let reflected = reflect(&r_in.dir, &rec.normal);
    let reflected = unit_vector(&reflected) + (fuzz * random_unit_vector());
    scattered.change(&rec.p, &reflected);
    attenuation.change(albedo.x(), albedo.y(), albedo.z());

    dot(scattered.direction(), &rec.normal) > 0.0
}

fn dielectric_scatter(
    refraction_index: f64,
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Color,
    scattered: &mut Ray,
) -> bool {
    attenuation.change(1.0, 1.0, 1.0);
    let ri = if rec.front_face {
        1.0 / refraction_index
    } else {
        refraction_index
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

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    (1.0 - r0).mul_add((1.0 - cosine).powi(5), r0)
}

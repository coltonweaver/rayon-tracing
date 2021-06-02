use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::{random_in_unit_sphere, random_unit_vector};
use crate::vec3::{Color, Vec3};

use rand::random;

pub trait Material {
    fn scatter(&self, ray_in: &mut Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &mut Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = &rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        let scattered = Ray {
            orig: rec.p.clone(),
            dir: scatter_direction,
        };

        Some((scattered, self.albedo.clone()))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &mut Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&ray_in.dir.unit_vector(), &rec.normal);
        let scattered = Ray {
            orig: rec.p.clone(),
            dir: reflected + (random_in_unit_sphere() * self.fuzz),
        };

        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some((scattered, self.albedo.clone()))
        } else {
            None
        }
    }
}

pub struct Dialectric {
    pub ir: f32, // Index of Refraction
}

impl Material for Dialectric {
    fn scatter(&self, ray_in: &mut Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio: f32;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        } else {
            refraction_ratio = self.ir;
        }

        let unit_direction: Vec3 = ray_in.dir.unit_vector();
        let cos_theta: f32 = (-&unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta: f32 = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract: bool = (refraction_ratio * sin_theta) > 1.0;

        let direction: Vec3;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        let res_ray: Ray = Ray {
            orig: rec.p.clone(),
            dir: direction,
        };

        Some((res_ray, Color::ones()))
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - (n * (v.dot(&n)) * 2.0)
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta: f32 = (-uv).dot(&n).min(1.0);
    let r_out_perp: Vec3 = (uv + (n * cos_theta)) * etai_over_etat;
    let r_out_parallel: Vec3 = n * (-1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt());

    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance
    let r0_sqrt: f32 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0_sqrt * r0_sqrt;

    let one_min_cosine = 1.0 - cosine;
    let one_min_cosine_fifth =
        one_min_cosine * one_min_cosine * one_min_cosine * one_min_cosine * one_min_cosine;

    r0 + (1.0 - r0) * one_min_cosine_fifth
}

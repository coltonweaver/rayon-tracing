use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Color, Vec3};
use crate::utils::{random_unit_vector, random_in_unit_sphere};

use rand::random;

pub trait Material {
    fn scatter(&self, ray_in: &mut Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &mut Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray { orig: rec.p, dir: scatter_direction };

        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo:  Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &mut Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(ray_in.dir.unit_vector(), rec.normal);
        let scattered = Ray { orig: rec.p, dir: reflected + (random_in_unit_sphere() * self.fuzz) };
        
        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dialectric {
    pub ir: f64, // Index of Refraction
}

impl Material for Dialectric {
    fn scatter(&self, ray_in: &mut Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio: f64;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        } else {
            refraction_ratio = self.ir;
        }

        let unit_direction: Vec3 = ray_in.dir.unit_vector();
        let cos_theta: f64 = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract: bool = (refraction_ratio * sin_theta) > 1.0;

        let direction: Vec3;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
            direction = reflect(unit_direction, rec.normal);
        } else {
            direction = refract(unit_direction, rec.normal, refraction_ratio);
        }

        let res_ray: Ray = Ray { orig: rec.p, dir: direction };

        Some((res_ray, Color::ones()))
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (n * (v.dot(&n)) * 2.0)
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta: f64 = (-uv).dot(&n).min(1.0);
    let r_out_perp: Vec3 = (uv + (n * cos_theta)) * etai_over_etat;
    let r_out_parallel: Vec3 = n * (-1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt());

    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let r0: f64 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
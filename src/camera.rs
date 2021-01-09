use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::utils::{degrees_to_radians, random_in_unit_disk};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub w: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta: f64 = degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        
        let w: Vec3 = (lookfrom - lookat).unit_vector();
        let u: Vec3 = vup.cross(w).unit_vector();
        let v: Vec3 = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - (w * focus_dist);

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            w: w,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = random_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = (self.u * rd.x) + (self.v * rd.y);

        Ray {
            orig: self.origin + offset, 
            dir: self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.origin - offset
        }
    }
}

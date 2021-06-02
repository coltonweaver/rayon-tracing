use crate::ray::Ray;
use crate::utils::{degrees_to_radians, random_in_unit_disk};
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub w: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: &Point3,
        lookat: &Point3,
        vup: &Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = &u * viewport_width * focus_dist;
        let vertical = &v * viewport_height * focus_dist;
        let lower_left_corner =
            origin - (&horizontal / 2.0) - (&vertical / 2.0) - (&w * focus_dist);

        Camera {
            origin: origin.clone(),
            horizontal,
            vertical,
            lower_left_corner,
            w,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = (&self.u * rd.x) + (&self.v * rd.y);

        Ray {
            orig: &self.origin + &offset,
            dir: &self.lower_left_corner + (&self.horizontal * s) + (&self.vertical * t)
                - &self.origin
                - offset,
        }
    }
}

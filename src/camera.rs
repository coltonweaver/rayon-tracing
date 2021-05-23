use crate::ray::Ray;
use crate::utils::{degrees_to_radians, random_in_unit_disk};
use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

pub struct Camera {
    pub origin: Arc<Point3>,
    pub lower_left_corner: Arc<Point3>,
    pub horizontal: Arc<Vec3>,
    pub vertical: Arc<Vec3>,
    pub w: Arc<Vec3>,
    pub u: Arc<Vec3>,
    pub v: Arc<Vec3>,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Arc<Point3>,
        lookat: Arc<Point3>,
        vup: Arc<Vec3>,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Arc::new((lookfrom.as_ref() - lookat.as_ref()).unit_vector());
        let u = Arc::new(vup.cross(w.as_ref()).unit_vector());
        let v = Arc::new(w.cross(u.as_ref()));

        let origin = lookfrom;
        let horizontal = Arc::new(u.as_ref() * viewport_width * focus_dist);
        let vertical = Arc::new(v.as_ref() * viewport_height * focus_dist);
        let lower_left_corner = Arc::new(
            origin.as_ref()
                - (horizontal.as_ref() / 2.0f32)
                - (vertical.as_ref() / 2.0f32)
                - (w.as_ref() * focus_dist),
        );

        Camera {
            origin,
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
        let offset = Arc::new((self.u.as_ref() * rd.x) + (self.v.as_ref() * rd.y));

        Ray {
            orig: Arc::new(self.origin.as_ref() + offset.as_ref()),
            dir: Arc::new(
                self.lower_left_corner.as_ref()
                    + (self.horizontal.as_ref() * s)
                    + (self.vertical.as_ref() * t)
                    - self.origin.as_ref()
                    - offset.as_ref(),
            ),
        }
    }
}

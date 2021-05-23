use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use std::{sync::Arc, vec::Vec};

pub struct HitRecord {
    pub p: Arc<Point3>,
    pub normal: Arc<Vec3>,
    pub t: f32,
    pub front_face: bool,
    pub mat: Arc<dyn Material + Send + Sync>,
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(&outward_normal) < 0.0;
        if self.front_face {
            self.normal = Arc::new(outward_normal)
        } else {
            self.normal = Arc::new(-outward_normal)
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_option: Option<HitRecord> = None;
        let mut closest_so_far: f32 = t_max;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_option = Some(hit)
            }
        }

        hit_option
    }
}

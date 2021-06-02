use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub m: Arc<dyn Material + Sync + Send>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = &r.orig - &self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = half_b.powi(2) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find nearest root that lies in the acceptible range
        let mut root = (-half_b - sqrtd) / a;
        if (root < t_min) || (t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (t_max < root) {
                return None;
            }
        }

        let point = &r.at(root);
        let mut record = HitRecord {
            t: root,
            p: point.clone(),
            normal: Vec3::zeroes(),
            front_face: false,
            mat: self.m.as_ref(),
        };
        let outward_normal = (point - &self.center) / self.radius;
        record.set_face_normal(r, outward_normal);

        Some(record)
    }
}

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

pub struct Sphere {
    pub center: Arc<Point3>,
    pub radius: f32,
    pub m: Arc<dyn Material + Send + Sync>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = r.orig.as_ref() - self.center.as_ref();
        let a: f32 = r.dir.length_squared();
        let half_b: f32 = oc.dot(&r.dir);
        let c: f32 = oc.length_squared() - (self.radius * self.radius);

        let discriminant: f32 = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd: f32 = discriminant.sqrt();

        // Find nearest root that lies in the acceptible range
        let mut root: f32 = (-half_b - sqrtd) / a;
        if (root < t_min) || (t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (t_max < root) {
                return None;
            }
        }

        let point = Arc::new(r.at(root));
        let mut record: HitRecord = HitRecord {
            t: root,
            p: point.clone(),
            normal: Arc::new(Vec3::zeroes()),
            front_face: false,
            mat: self.m.clone(),
        };
        let outward_normal: Vec3 = (point.as_ref() - self.center.as_ref()) / self.radius;
        record.set_face_normal(r, outward_normal);

        Some(record)
    }
}

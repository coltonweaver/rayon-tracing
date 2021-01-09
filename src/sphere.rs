use crate::vec3::{Point3, Vec3};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub m: &'static (dyn Material + Sync),
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.orig - self.center;
        let a: f64 = r.dir.length_squared();
        let half_b: f64 = oc.dot(&r.dir);
        let c: f64 = oc.length_squared() - self.radius.powi(2);
        
        let discriminant: f64 = half_b.powi(2) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd: f64 = discriminant.sqrt();

        // Find nearest root that lies in the acceptible range
        let mut root: f64 = (-half_b - sqrtd) / a;
        if (root < t_min) || (t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (t_max < root) {
                return None;
            }
        }

        let point = r.at(root);
        let mut record: HitRecord = HitRecord { t: root, p: point, normal: Vec3::zeroes(), front_face: false, mat: self.m }; 
        let outward_normal: Vec3 = (point - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);
        
        Some(record)
    }
}

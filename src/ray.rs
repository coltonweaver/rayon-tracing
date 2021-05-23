use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

pub struct Ray {
    pub orig: Arc<Point3>,
    pub dir: Arc<Vec3>,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.orig.as_ref() + (self.dir.as_ref() * t)
    }
}

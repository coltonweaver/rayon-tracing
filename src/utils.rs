use rand::{Rng, thread_rng};

use crate::ray::Ray;
use crate::vec3::{Vec3, Color, Point3};
use crate::hittable::{Hittable, HittableList};
use crate::material::{Lambertian, Metal, Dialectric};
use crate::sphere::Sphere;

pub fn ray_color(r: &mut Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zeroes();
    }

    if let Some(hit) = world.hit(&r, 0.001, f64::INFINITY) {
        if let Some(mut result) = hit.mat.scatter(r, &hit) {
            return ray_color(&mut result.0, world, depth - 1) * result.1;
        }
        
        return Color::zeroes();
    }

    let unit_direction: Vec3 = r.dir.unit_vector();
    let t: f64 = 0.5 * (unit_direction.y + 1.0);
    (Color::ones() * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}

pub fn write_color(pixel: &Color, samples_per_pixel: f64) {
    let mut r: f64 = pixel.x;
    let mut g: f64 = pixel.y;
    let mut b: f64 = pixel.z;

    // Divide the color by the number of samples.
    let scale: f64 = 1.0 / samples_per_pixel;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // Write out the translated [0,255] value of each color component.
    print!("{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.0)
}

// Diffuse Renderers

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let mut rng = thread_rng();
        let p: Vec3 = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p
        }
    }
}

// World

pub fn random_scene(world: &mut HittableList) {
    let ground_material = Box::new(Lambertian { albedo: Color::new(0.5, 0.5, 0.5)});
    let ground_sphere: Box<Sphere> = Box::new(Sphere { center: Point3::new(0.0, -1000.0, 0.0), radius: 1000.0, m: Box::leak(ground_material) });
    world.objects.push(ground_sphere);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rand::random();
            let rand_1: f64 = rand::random();
            let rand_2: f64 = rand::random();
            let center: Point3 = Point3::new(a as f64 + 0.9 * rand_1, 0.2, b as f64 + 0.9 * rand_2);

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Box::new(Lambertian { albedo: albedo });
                    let sphere = Box::new(Sphere { center: center, radius: 0.2, m: Box::leak(sphere_material) });
                    world.objects.push(sphere);
                } else if choose_mat < 0.95 {
                    let mut rng = thread_rng();
                    let albedo = Color::random_range(0.5,1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let sphere_material = Box::new(Metal { albedo: albedo, fuzz: fuzz });
                    let sphere = Box::new(Sphere { center: center, radius: 0.2, m: Box::leak(sphere_material) });
                    world.objects.push(sphere);
                } else {
                    let sphere_material = Box::new(Dialectric { ir: 1.5 });
                    let sphere = Box::new(Sphere { center: center, radius: 0.2, m: Box::leak(sphere_material) });
                    world.objects.push(sphere);
                }
            }
        }
    }

    let material_1 = Box::new(Dialectric { ir: 1.5 });
    world.objects.push(Box::new(Sphere { center: Point3::new(0.0, 1.0, 0.0), radius: 1.0, m: Box::leak(material_1) }));

    let material_2 = Box::new(Lambertian { albedo: Color::new(0.4, 0.2, 0.1) });
    world.objects.push(Box::new(Sphere { center: Point3::new(-4.0, 1.0, 0.0), radius: 1.0, m: Box::leak(material_2) }));

    let material_3 = Box::new(Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 });
    world.objects.push(Box::new(Sphere { center: Point3::new(4.0, 1.0, 0.0), radius: 1.0, m: Box::leak(material_3) }));
}
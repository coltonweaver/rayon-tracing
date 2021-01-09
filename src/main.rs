extern crate rand;

use rayon::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

use vec3::{Color, Point3, Vec3};
use ray::Ray;
use hittable::HittableList;
use camera::Camera;

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;
mod utils;

// Image

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const IMAGE_SIZE: i32 = IMAGE_WIDTH * IMAGE_HEIGHT;
const SAMPLES_PER_PIXEL: i32 = 500;
const MAX_DEPTH: i32 = 50;

fn main() {
    // Local World
    let mut world: HittableList = HittableList { objects: Vec::new() };
    utils::random_scene(&mut world);

    // Camera
    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::zeroes();
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let aperture: f64 = 0.1;
    let dist_to_focus: f64 = 10.0;
    
    let cam = &Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    // Render

    let image_vec: Vec<Vec<Color>> = vec![vec![Color::zeroes(); IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize];
    let synchronized_image_vec = Arc::new(Mutex::new(image_vec));

    eprintln!("Rendering image with resolution of {}x{}:", IMAGE_WIDTH, IMAGE_HEIGHT);
    let progress = AtomicI32::new(0);
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    (0..IMAGE_HEIGHT).into_par_iter().rev().for_each(|j| {
        (0..IMAGE_WIDTH).into_par_iter().for_each(|i| {
            // For each sample per pixel, calculate the color, and then finally fold together into sum for one pixel.
            let mut pixel_color = Color::zeroes();
            (0..SAMPLES_PER_PIXEL).into_iter().for_each(|_| {
                let mut r: Ray = cam.get_ray(
                    ((i as f64) + rand::random::<f64>()) / (IMAGE_WIDTH as f64),
                    ((j as f64) + rand::random::<f64>()) / (IMAGE_HEIGHT as f64)
                );
                pixel_color += &utils::ray_color(&mut r, &world, MAX_DEPTH);
            });
            // Write pixel rgb values to syncrhonized_image_vec, which is converted to the image at the end.
            synchronized_image_vec.lock().unwrap()[j as usize][i as usize] = pixel_color;
            progress.fetch_add(1, Ordering::SeqCst);
            print_progress(progress.load(Ordering::SeqCst) as f32, IMAGE_SIZE as f32);
        });
    });

    eprintln!("\nDone rendering, writing to stdout now:");
    let mut count = 0;
    (0..IMAGE_HEIGHT).into_iter().rev().for_each(|j| {
        (0..IMAGE_WIDTH).into_iter().for_each(|i| {
            utils::write_color(&synchronized_image_vec.lock().unwrap()[j as usize][i as usize], SAMPLES_PER_PIXEL as f64);
            count += 1;
            print_progress(count as f32, IMAGE_SIZE as f32);
        });
    });

    eprint!("\nDone!\n");
}

fn print_progress(current_count: f32, total_count: f32) {
    eprint!("\r               ");
    eprint!("\r{:.3}% complete...", ((current_count/total_count) * 100.0));
}

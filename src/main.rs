extern crate rand;

use parking_lot::RwLock;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use camera::Camera;
use hittable::HittableList;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

// Image

const ASPECT_RATIO: f32 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 500;
const MAX_DEPTH: i32 = 50;

fn main() {
    // Local World
    let mut world: HittableList = HittableList {
        objects: Vec::new(),
    };
    utils::random_scene(&mut world);

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::zeroes();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture: f32 = 0.1;
    let dist_to_focus: f32 = 10.0;

    let cam = &Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render

    let image_vec: Vec<Vec<Color>> =
        vec![vec![Color::zeroes(); IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize];
    let synchronized_image_vec = RwLock::new(image_vec);

    eprintln!(
        "Rendering image with resolution of {}x{}:",
        IMAGE_WIDTH, IMAGE_HEIGHT
    );
    (0..IMAGE_HEIGHT).into_par_iter().rev().for_each(|j| {
        (0..IMAGE_WIDTH).into_par_iter().for_each(|i| {
            // For each sample per pixel, calculate the color, and then finally fold together into sum for one pixel.
            let mut pixel_color = Color::zeroes();
            (0..SAMPLES_PER_PIXEL).into_iter().for_each(|_| {
                let mut r: Ray = cam.get_ray(
                    ((i as f32) + rand::random::<f32>()) / (IMAGE_WIDTH as f32),
                    ((j as f32) + rand::random::<f32>()) / (IMAGE_HEIGHT as f32),
                );
                pixel_color += &utils::ray_color(&mut r, &world, MAX_DEPTH);
            });
            // Write pixel rgb values to syncrhonized_image_vec, which is converted to the image at the end.
            synchronized_image_vec.write()[j as usize][i as usize] = pixel_color;
        });
    });

    eprintln!("\nDone rendering, writing to file now:");
    let output_file = File::create("result.ppm").expect("Unable to open result.ppm file!");
    let mut buf_writer = BufWriter::new(output_file);
    buf_writer
        .write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT))
        .expect("Unable to write resulting image!");
    let mut count = 0;
    (0..IMAGE_HEIGHT).into_iter().rev().for_each(|j| {
        (0..IMAGE_WIDTH).into_iter().for_each(|i| {
            utils::write_color(
                &mut buf_writer,
                &synchronized_image_vec.read()[j as usize][i as usize],
                SAMPLES_PER_PIXEL as f32,
            );
            count += 1;
        });
    });

    eprint!("\nDone!\n");
}

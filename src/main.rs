extern crate rand;

use camera::Camera;
use hittable::HittableList;
use ray::Ray;
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    sync::Arc,
};
use tokio::sync::mpsc::channel;
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

#[derive(Debug)]
struct ColorWrapper {
    color: Color,
    indeces: (usize, usize),
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Local World
    let mut world: HittableList = HittableList {
        objects: Vec::new(),
    };
    utils::random_scene(&mut world);

    let world = Arc::new(world);

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::zeroes();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture: f32 = 0.1;
    let dist_to_focus: f32 = 10.0;

    let cam = Arc::new(Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    ));

    // Render

    let (sender, mut receiver) = channel((IMAGE_HEIGHT * IMAGE_WIDTH) as usize);

    let mut image_vec = vec![vec![Color::zeroes(); IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize];

    let join_handle = tokio::spawn(async move {
        while let Some(ColorWrapper { color, indeces }) = receiver.recv().await {
            let (j, i) = indeces;

            image_vec[j][i] = color;
        }

        let path = "result.ppm";
        let output_file = File::create(path).expect("Unable to open result.ppm file!");
        let mut buf_writer = BufWriter::new(output_file);
        buf_writer
            .write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT))
            .expect("Unable to write resulting image!");
        let mut count = 0;
        (0..IMAGE_HEIGHT).into_iter().rev().for_each(|j| {
            (0..IMAGE_WIDTH).into_iter().for_each(|i| {
                utils::write_color(
                    &mut buf_writer,
                    &image_vec[j as usize][i as usize],
                    SAMPLES_PER_PIXEL as f32,
                );
                count += 1;
            });
        });
    });

    eprintln!(
        "Rendering image with resolution of {}x{}:",
        IMAGE_WIDTH, IMAGE_HEIGHT
    );
    (0..(IMAGE_HEIGHT)).into_iter().rev().for_each(|j| {
        let sender = sender.clone();
        let world = world.clone();
        let cam = cam.clone();
        tokio::spawn(async move {
            for i in 0..IMAGE_WIDTH {
                // For each sample per pixel, calculate the color, and then finally fold together into sum for one pixel.
                let mut pixel = Color::zeroes();
                (0..SAMPLES_PER_PIXEL).into_iter().for_each(|_| {
                    let random_f32: f32 = rand::random();
                    let mut r: Ray = cam.get_ray(
                        ((i as f32) + random_f32) / (IMAGE_WIDTH as f32),
                        ((j as f32) + random_f32) / (IMAGE_HEIGHT as f32),
                    );

                    // Write pixel rgb values to syncrhonized_image_vec, which is converted to the image at the end.
                    pixel += &utils::ray_color(&mut r, &world, MAX_DEPTH);
                });
                sender
                    .send(ColorWrapper {
                        color: pixel,
                        indeces: (j as usize, i as usize),
                    })
                    .await
                    .unwrap();
            }

            tokio::task::yield_now().await;
        });
    });

    drop(sender);

    let err = join_handle.await.unwrap_err();
    assert!(err.is_panic());
    Ok(())
}

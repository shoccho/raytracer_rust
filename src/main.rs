#![allow(dead_code)]
extern crate sdl2;

mod camera;
mod hit_record;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use core::f64;
use hittable_list::HittableList;
use interval::Interval;
use material::Material;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sphere::Sphere;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::Arc;
use std::time::Duration;
use vec3::Vec3;

const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: usize = 900;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

fn print_image(buff_data: &[Vec<Vec3>]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).open("out.ppm")?;
    file.write_all(b"P3\n")?;
    file.write_all(
        format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT)
            .to_string()
            .as_bytes(),
    )?;
    file.write_all(b"255\n")?;

    for (j, row) in buff_data.iter().enumerate() {
        eprintln!("Lines remaining {}", IMAGE_HEIGHT - j);
        for data in row.iter() {
            let ir = (255.999 * data.x) as isize;
            let ig = (255.999 * data.y) as isize;
            let ib = (255.999 * data.z) as isize;
            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }
    Ok(())
}

fn main() {
    let output = std::env::args().nth(1).unwrap_or_else(|| "sdl".to_string());
    let range: i32 = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "2".to_string())
        .parse()
        .unwrap();

    let mut world = HittableList::new();

    let ground_material = Material::Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };
    world.push(Box::new(Sphere::new(
        &Vec3::new(0.0, -1000., -1.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::thread_rng();
    for a in -range..range {
        for b in -range..range {
            let mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if Vec3::sub(&center, &Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let material: Material = if mat < 0.8 {
                    let albedo = Vec3::mul_vec(&Vec3::new_rand(), &Vec3::new_rand());
                    Material::Lambertian { albedo }
                } else if mat < 0.95 {
                    let albedo = Vec3::new_rand_ranged(0.5, 1.);
                    let fuzz = 0. + (0.5 * rng.gen::<f64>());
                    Material::Metal { albedo, fuzz }
                } else {
                    Material::Dielectric {
                        refraction_index: 1.5,
                    }
                };
                world.push(Box::new(Sphere::new(&center, 0.2, material)));
            }
        }
    }
    let material1 = Material::Dielectric {
        refraction_index: 0.5,
    };
    world.push(Box::new(Sphere::new(
        &Vec3::new(0.0, 1., 0.),
        1.,
        material1,
    )));

    let material2 = Material::Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    };
    world.push(Box::new(Sphere::new(
        &Vec3::new(-4.0, 1., 0.),
        1.,
        material2,
    )));

    let material3 = Material::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    world.push(Box::new(Sphere::new(
        &Vec3::new(4.0, 1., 0.),
        1.,
        material3,
    )));

    let camera: Camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        Vec3::new(13., 2., 3.),
        Vec3::new(0., 0., 0.),
        20.,
        0.6,
        10.,
    );
    let buff_data = Camera::render(Arc::new(camera), world);

    if output == "ppm" {
        print_image(&buff_data).unwrap();
        return;
    }

    let intensity = Interval::new_with_values(0.0000, 0.9999);

    // sdl things
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo", IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        for (j, row) in buff_data.iter().enumerate() {
            for (i, data) in row.iter().enumerate() {
                canvas.set_draw_color(Color::RGB(
                    (256.0 * intensity.clamp(data.x)) as u8,
                    (256.0 * intensity.clamp(data.y)) as u8,
                    (256.0 * intensity.clamp(data.z)) as u8,
                ));
                let _ = canvas.draw_point(Point::new(i as i32, j as i32));
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

#![allow(dead_code)]
extern crate sdl2;

mod ray;
mod vec3;

use ray::Ray;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use vec3::Vec3;

use std::time::Duration;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 900;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

fn print_head() {
    println!("P3");
    println!("{IMAGE_HEIGHT} {IMAGE_WIDTH}");
    println!("255")
}

fn print_image(buff_data: &mut [Vec<Vec3>]) {
    for (j, row) in buff_data.iter().enumerate() {
        eprintln!("Lines remaining {}", IMAGE_HEIGHT - j);
        for data in row.iter() {
            let ir = (255.999 * data.x) as isize;
            let ig = (255.999 * data.y) as isize;
            let ib = (255.999 * data.z) as isize;
            println!("{ir} {ig} {ib}");
        }
    }
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = Vec3::sub(center, &ray.origin);
    let a = ray.direction.length_squared();
    let h = Vec3::dot(&ray.direction, &oc);
    let c = oc.length_squared() - radius*radius;
    let d = h*h - a*c;
    if d < 0.0 {
        return -1.0;
    } else {
        return (h-d.sqrt()) / a;
    }
}

fn ray_color(ray: &Ray) -> Vec3 {
    let hit = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if hit > 0.0 {
        let n = Vec3::sub(&ray.at(hit), &Vec3::new(0.0,0.0,-1.0));
        return Vec3::mul(&Vec3::add(&n, &Vec3::new(1.0, 1.0, 1.0)), 0.5)
    }
    

    let unit_dir = Vec3::unit(&ray.direction);

    let a = 0.5 * (unit_dir.y + 1.0);

    Vec3::add(
        &Vec3::mul(&Vec3::new(1.0, 1.0, 1.0), 1.0 - a),
        &Vec3::mul(&Vec3::new(0.5, 0.7, 1.0), a),
    )
}

fn main() {
    // print_head();

    let mut buff_data = vec![vec![Vec3::default(); IMAGE_WIDTH]; IMAGE_HEIGHT];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let camera_center = Vec3::new(0f64, 0f64, 0f64);
    let viewport_width = viewport_height * ((IMAGE_WIDTH) as f64 / IMAGE_HEIGHT as f64);
    let viewport_u = Vec3::new(viewport_width, 0f64, 0f64);
    let viewport_v = Vec3::new(0f64, -viewport_height, 0f64);

    let pixel_delta_u = Vec3::div(&viewport_u, IMAGE_WIDTH as f64);
    let pixel_delta_v = Vec3::div(&viewport_v, IMAGE_HEIGHT as f64);

    let viewport_upper_left = Vec3::sub(
        &Vec3::sub(&camera_center, &Vec3::new(0.0, 0.0, focal_length)),
        &Vec3::add(&Vec3::div(&viewport_u, 2.0), &Vec3::div(&viewport_v, 2.0)),
    );

    let pixel00_loc = Vec3::add(
        &viewport_upper_left,
        &Vec3::mul(&Vec3::add(&pixel_delta_u, &pixel_delta_v), 0.5),
    );

    for (j, row) in buff_data.iter_mut().enumerate() {
        for (i, data) in row.iter_mut().enumerate() {
            let pixel_center = Vec3::add(
                &pixel00_loc,
                &Vec3::add(
                    &Vec3::mul(&pixel_delta_u, i as f64),
                    &Vec3::mul(&pixel_delta_v, j as f64),
                ),
            );
            let ray_direction = Vec3::sub(&pixel_center, &camera_center);
            let ray = Ray {
                origin: camera_center.clone(),
                direction: ray_direction.clone(),
            };
            let color = ray_color(&ray);
            data.x = color.x;
            data.y = color.y;
            data.z = color.z;
        }
    }

    // print_image(&mut buff_data);

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
                    (255.999 * data.x) as u8,
                    (255.999 * data.y) as u8,
                    (255.999 * data.z) as u8,
                ));
                let _ = canvas.draw_point(Point::new(i as i32, j as i32));
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

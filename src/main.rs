#![allow(dead_code)]
extern crate sdl2;

mod hit_record;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;
mod camera;

use camera::Camera;
use hittable_list::HittableList;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sphere::Sphere;
use vec3::Vec3;

use core::f64;
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


fn main() {
    // print_head();
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(&Vec3::new(0.0, -101.0, -1.0), 100.0)));
    let mut buff_data = vec![vec![Vec3::default(); IMAGE_WIDTH]; IMAGE_HEIGHT];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH);
    camera.render(&world, &mut buff_data);

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

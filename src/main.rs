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
use hittable_list::HittableList;
use interval::Interval;
use material::{Dielectric, Lambertian, Material, Metal};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sphere::Sphere;
use vec3::Vec3;
use rand::Rng;
use core::f64;
use std::rc::Rc;
use std::time::Duration;

const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: usize = 900;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

fn print_head() {
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255")
}

fn print_image(buff_data: &[Vec<Vec3>]) {
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
    let mut world = HittableList::new();

    let left_material = Rc::new(Dielectric::new(1.5));
    let bubble_material = Rc::new(Dielectric::new(1. / 1.5));
    let right_material = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));
    let center_material = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        &Vec3::new(0.0, -1000., -1.0),
        1000.0,
        ground_material,
    )));
    
    let mut rng = rand::thread_rng();
    for a in -11..11{
        for b in -11..11{
            let mat = rng.gen::<f64>();
            let center = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if Vec3::sub(&center, &Vec3::new(4.,0.2, 0.)).length() > 0.9 {
                let material: Rc<dyn Material> = if mat < 0.8 {
                    let albedo = Vec3::mul_vec(&Vec3::new_rand() , &Vec3::new_rand());
                    Rc::new(Lambertian::new(albedo))
                }else if mat < 0.95{
                    let albedo = Vec3::new_rand_ranged(0.5, 1.);
                    let fuzz = 0. + (0.5 * rng.gen::<f64>());
                    Rc::new(Metal::new(albedo, fuzz))
                }else {
                    Rc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(&center, 0.2, material)));
            }
        }
    }
    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(&Vec3::new(0.0, 1., 0.), 1., material1)));
    
    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(&Vec3::new(-4.0, 1., 0.), 1., material2)));

    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(&Vec3::new(4.0, 1., 0.), 1., material3)));

    let mut buff_data = vec![vec![Vec3::default(); IMAGE_WIDTH]; IMAGE_HEIGHT];

    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        Vec3::new(13., 2., 3.),
        Vec3::new(0., 0., 0.),
        20.,
        0.6,
        10.,
    );
    camera.render(&world, &mut buff_data);

    // print_head();
    // print_image(&buff_data);

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

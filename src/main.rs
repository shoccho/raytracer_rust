extern crate sdl2;

mod ray;
mod vec3;

use ray::ray::Ray;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use vec3::vec3::Vec3;

use std::time::Duration;

const image_height: usize = 256;
const image_width: usize = 256;

fn print_head() {
    println!("P3");
    println!("{image_height} {image_width}");
    println!("255")
}

fn print_image(buffData: &mut Vec<Vec<Vec3>>) {
    for j in 0..image_height {
        eprintln!("Lines remaining {}", image_height - j);
        for i in 0..image_width {
            let ir = (255.999 * buffData[j][i].x) as isize;
            let ig = (255.999 * buffData[j][i].y) as isize;
            let ib = (255.999 * buffData[j][i].z) as isize;
            println!("{ir} {ig} {ib}");
        }
    }
}

fn ray_color(ray: &Ray) -> Vec3 {
    // Vec3::default()
    // vec3 unit_direction = unit_vector(r.direction());
    let unit_dir = Vec3::unit(&ray.direction);

    let a = 0.5 * (unit_dir.y + 1.0);
    // return (1.0-a)*color(1.0, 1.0, 1.0) + a*color(0.5, 0.7, 1.0);
    Vec3::add(
        &Vec3::mul(
            &Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            1.0 - a,
        ),
        &Vec3::mul(
            &Vec3 {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            },
            a,
        ),
    )
}

fn main() {
    // print_head();
    
    let mut buffData = vec![vec![Vec3::default(); image_width]; image_height];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", image_width as u32, image_width as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let camera_center = Vec3 {
        x: 0f64,
        y: 0f64,
        z: 0f64,
    };
    let viewport_width = viewport_height * ((image_width) as f64 / image_height as f64);
    let viewport_u = Vec3 {
        x: viewport_width,
        y: 0f64,
        z: 0f64,
    };
    let viewport_v = Vec3 {
        x: 0f64,
        y: -viewport_height,
        z: 0f64,
    };

    let pixel_delta_u = Vec3::div(&viewport_u, image_width as f64);
    let pixel_delta_v = Vec3::div(&viewport_v, image_height as f64);

    let viewport_upper_left = Vec3::sub(
        &camera_center,
        &Vec3::sub(
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            },
            &Vec3::sub(&Vec3::div(&viewport_u, 2.0), &Vec3::div(&viewport_v, 2.0)),
        ),
    );
    let pixel00_loc = Vec3::add(
        &viewport_upper_left,
        &Vec3::mul(&Vec3::add(&pixel_delta_u, &pixel_delta_v), 0.5),
    );

    for j in 0..image_height {
        for i in 0..image_width {
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
            let pixel_color = ray_color(&ray);
            buffData[j][i] = pixel_color;
        }
    }

    // print_image(&mut buffData);

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
        for j in 0..image_height {
            for i in 0..image_width {
                canvas.set_draw_color(Color::RGB(
                    (255.999 * buffData[j][i].x) as u8,
                    (255.999 * buffData[j][i].y) as u8,
                    (255.999 * buffData[j][i].z) as u8,
                ));
                let _ = canvas.draw_point(Point::new(i as i32, j as i32));
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

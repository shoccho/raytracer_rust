extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
const image_height: usize = 256;
const image_width: usize = 256;

fn print_head(){
    println!("P3");
    println!("{image_height} {image_width}");
    println!("255")
}
fn print_image( buffData: &mut Vec<Vec<Vec3>>){
    for j in 0..image_height{
        eprintln!("Lines remaining {}",image_height-j);
        for i in 0..image_width{
            let r :f64  = i as f64 / (image_width-1) as f64;
            let g :f64  = j as f64 / (image_height-1) as f64;
            let b = 0.0f64;
            
            // let ir = (255.999 * r) as isize;
            // let ig = (255.999 * g) as isize;
            // let ib = (255.999 * b) as isize;
            
            let x = (255.999 * r);
            let y = (255.999 * g);
            let z = (255.999 * b);
            // buffData.get_mut(j).get_or_insert(Vec3{x, y, z});
            buffData[j][i] = Vec3{x, y, z}
        }
    }
}

#[derive(Clone)]
struct Vec3{
    x: f64,
    y: f64,
    z: f64
}
impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            z: 0f64
        }
    }
}

fn main() {
    print_head();
    // print_image();
    let mut buffData = vec![vec![Vec3::default(); image_width]; image_height];

    print_image(&mut buffData);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", image_width as u32, image_width as u32)
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
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        for j in 0..image_height{
            for i in 0..image_width{
                canvas.set_draw_color( Color::RGB(buffData[j][i].x as u8, buffData[j][i].y as u8, buffData[j][i].z as u8));
                let _ = canvas.draw_point(Point::new(i as i32, j as i32));
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

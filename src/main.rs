extern crate sdl2;

use std::env::args_os;
use std::process::exit;
use std::io::{Write, stderr};

use sdl2::pixels;
use sdl2::render;
use sdl2::rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn usage() -> ! {
    let programname = args_os().nth(0).unwrap();
    let message = format!("Usage: {} HEIGHT WIDTH MEGABYTES\n", programname.to_string_lossy());
    let _ = stderr().write_all(message.as_bytes());
    exit(1);
}

fn arg_to_int(arg: u32) -> u32 {
    match args_os().nth(arg as usize).unwrap().to_string_lossy().parse::<u32>() {
        Ok(w) => w,
        Err(_) => usage(),
    }
}

fn main() {
    if args_os().count() != 4 {
        usage();
    }

    let mut current: isize = 0;

    let width = arg_to_int(1);
    let height = arg_to_int(2);
    let num_buffers = (arg_to_int(3) * 1024 * 1024) / (width * height * 4);

    println!("{}", num_buffers);

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("window", width, height).build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut renderer = window.renderer().build().unwrap();

    let mut txt: Vec<render::Texture> = Vec::new();

    for i in 0..num_buffers {
        println!("Now initializing buffer number {} of {}", i, num_buffers);

        txt.push(renderer.create_texture(
            pixels::PixelFormatEnum::ARGB8888,
            render::TextureAccess::Streaming,
            width,
            height).unwrap());
    }

    'running: loop {
        renderer.clear();
        renderer.copy(&txt[current as usize], None, None);
        renderer.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Escape) => {
                            break 'running
                        },
                        Some(Keycode::Right) => {
                            current += 1;
                        },
                        Some(Keycode::Left) => {
                            current -= 1;
                        },
                        Some(Keycode::Up) => {
                            current += 10;
                        },
                        Some(Keycode::Down) => {
                            current -= 10;
                        },
                        Some(Keycode::Space) => {
                            let path = "test.bmp";
                            let (w, h) = renderer.output_size().unwrap();
                            println!("Saving frame {}", current);
                            let mask = pixels::PixelMasks {
                                bpp: 32,
                                rmask: 0x00ff,
                                gmask: 0x0000ff,
                                bmask: 0x000000ff,
                                amask: 0xff,
                            };
                            let format = pixels::PixelFormatEnum::from_masks(mask);
                            // let rect = rect::Rect::new(0, 0, w, h);
                            let pixels = renderer.read_pixels(None, format);
                        },
                        _ => {}
                    }
                    current = current % num_buffers as isize;
                    if current < 0 {
                        current = current as isize + num_buffers as isize;
                    }
                    println!("Now displaying buffer {} of {}", current, num_buffers);
                },
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}

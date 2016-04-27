extern crate sdl2;
extern crate image;

use std::env::args_os;
use std::process::exit;
use std::io::{Write, stderr};
use std::fs;

use sdl2::{pixels, render};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use image::png;
use image::Pixel;

fn usage() -> ! {
    let programname = args_os().nth(0).unwrap();
    let message = format!("Usage: {} [-v] HEIGHT WIDTH MEGABYTES\n", programname.to_string_lossy());
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
    if args_os().count() != 4 && args_os().count() != 5 {
        usage();
    }

    let verbose = if args_os().nth(1).unwrap().to_string_lossy() == "-v" ||
        args_os().nth(1).unwrap().to_string_lossy() == "--verbose" {
        true
    } else {
        false
    };

    let mut current: isize = 0;

    let (width, height) = if ! verbose {
        (arg_to_int(1), arg_to_int(2))
    } else {
        (arg_to_int(2), arg_to_int(3))
    };

    let num_buffers = if ! verbose {
        (arg_to_int(3) * 1024 * 1024) / (width * height * 4)
    } else {
        (arg_to_int(4) * 1024 * 1024) / (width * height * 4)
    };

    if verbose {
        println!("{}", num_buffers);
    }

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("window", width, height).build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut renderer = window.renderer().build().unwrap();

    let mut txt: Vec<render::Texture> = Vec::new();

    for i in 0..num_buffers {
        if verbose {
            println!("Now initializing buffer number {} of {}", i, num_buffers);
        }

        txt.push(renderer.create_texture(
            pixels::PixelFormatEnum::RGBA8888,
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
                        Some(Keycode::Escape) | Some(Keycode::Q) => {
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
                            let path = format!("frame_{}.png", current);
                            let file = fs::File::create(path).unwrap();

                            let (w, h) = renderer.output_size().unwrap();

                            let format = pixels::PixelFormatEnum::RGBA8888;

                            let pixels = renderer.read_pixels(None, format).unwrap();

                            let mut image = image::ImageBuffer::from_raw(w, h, pixels).unwrap();
                            for pixel in image.pixels_mut() {
                                let mut new_pixel = image::Rgba::from_channels(0,0,0,0xff);
                                new_pixel.blend(pixel);
                                *pixel = new_pixel;
                            }

                            let encoder = png::PNGEncoder::new(file);

                            let _ = encoder.encode(&image, w, h, image::ColorType::RGBA(8));

                            println!("Saved frame {}", current);
                        },
                        _ => {}
                    }
                    current = current % num_buffers as isize;
                    if current < 0 {
                        current = current as isize + num_buffers as isize;
                    }
                    if verbose {
                        println!("Now displaying buffer {} of {}", current, num_buffers);
                    }
                },
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}

extern crate image;
#[macro_use] extern crate itertools;

use std::fs::{self, File};
use std::io::Write;

use image::{ImageBuffer, ImageRgb8};
use itertools::Itertools;

#[derive(Debug)]
struct Rgb(u8, u8, u8);

trait Generate {
    fn generate(&self, w: u8, h: u8, n: u8, x: u8, y: u8) -> Rgb;
    fn name(&self) -> &str;
    fn steps(&self, _w: u8, _h: u8) -> u8 {
        1
    }
}

struct HorizWave;
impl Generate for HorizWave {
    fn generate(&self, _w: u8, _h: u8, n: u8, x: u8, _y: u8) -> Rgb {
        let value = match x == n {
            true => 255,
            false => 0,
        };
        Rgb(value, value, value)
    }

    fn name(&self) -> &str {
        "horiz_wave"
    }

    fn steps(&self, w: u8, _h: u8) -> u8 {
        w
    }
}

trait Output {
    fn output(&self, animations: &Vec<Animation>, w: u8, h: u8);
}

struct Printer;
impl Output for Printer {
    fn output(&self, animations: &Vec<Animation>, _w: u8, _h: u8) {
        println!("{:?}", animations);
    }
}
struct ImageOutput;
impl Output for ImageOutput {
    fn output(&self, animations: &Vec<Animation>, w: u8, h: u8) {
        animations.iter().foreach(|anim| {
            fs::remove_dir_all(&anim.name).unwrap();
            fs::create_dir(&anim.name).unwrap();

            anim.images.iter().enumerate().foreach(|(i, image)| {
                let mut imgbuf = ImageBuffer::new(w as u32, h as u32);

                for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                    let Rgb(r, g, b) = image[(x as u8 * h + y as u8) as usize];
                    *pixel = image::Rgb([r, g, b]);
                }

                let ref mut fout = File::create(format!("{}/{}.png", anim.name, i)).unwrap();
                ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();
            });

            let ref mut list_file = File::create(format!("{}/list", anim.name)).unwrap();
            write!(
                list_file,
                "{}\n",
                (0..anim.images.len()).map(|n| format!("{}.png", n)).join("\n")
            ).unwrap();
        })
    }
}

#[derive(Debug)]
struct Animation {
    images: Vec<Vec<Rgb>>,
    name: String,
}
impl Animation {
    fn new(images: Vec<Vec<Rgb>>, name: &str) -> Animation {
        Animation {
            images: images,
            name: name.to_string()
        }
    }
}

fn main() {
    let w = 9;
    let h = 9;
    let generators: Vec<Box<Generate>> = vec![Box::new(HorizWave)];
    let outputters: Vec<Box<Output>> = vec![Box::new(Printer), Box::new(ImageOutput)];

    let animations = generators.iter().map(|generator| {
        Animation::new((0..generator.steps(w, h)).map(|n| iproduct!(0..w, 0..h).map(
            |(x, y)| generator.generate(w, h, n, x, y)
        ).collect()).collect(), generator.name())
    }).collect();

    outputters.iter().foreach(|outputter| outputter.output(&animations, w, h));
}

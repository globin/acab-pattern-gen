use std::fs::{self, File};
use std::io::Write;

use image::{self, ImageBuffer, ImageRgb8};
use itertools::Itertools;

use super::{Animation, Rgb};

pub trait Output {
    fn output(&self, animations: &Vec<Animation>, w: u8, h: u8);
}

pub struct Printer;
impl Output for Printer {
    fn output(&self, animations: &Vec<Animation>, _w: u8, _h: u8) {
        println!("{:?}", animations);
    }
}

pub struct ImageOutput;
impl Output for ImageOutput {
    fn output(&self, animations: &Vec<Animation>, w: u8, h: u8) {
        animations.iter().foreach(|anim| {
            if fs::metadata(&anim.name).is_ok() {
                fs::remove_dir_all(&anim.name).unwrap();
            }
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

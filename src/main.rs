extern crate image;
#[macro_use] extern crate itertools;

use itertools::Itertools;
use output::{Printer, ImageOutput, Output};

mod output;

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

struct VertWave;
impl Generate for VertWave {
    fn generate(&self, _w: u8, _h: u8, n: u8, _x: u8, y: u8) -> Rgb {
        let value = match y == n {
            true => 255,
            false => 0,
        };
        Rgb(value, value, value)
    }

    fn name(&self) -> &str {
        "vert_wave"
    }

    fn steps(&self, _w: u8, h: u8) -> u8 {
        h
    }
}

fn main() {
    let w = 9;
    let h = 9;
    let generators: Vec<Box<Generate>> = vec![
        Box::new(HorizWave),
        Box::new(VertWave),
    ];
    let outputters: Vec<Box<Output>> = vec![Box::new(Printer), Box::new(ImageOutput)];

    let animations = generators.iter().map(|generator| {
        Animation::new((0..generator.steps(w, h)).map(|n| iproduct!(0..w, 0..h).map(
            |(x, y)| generator.generate(w, h, n, x, y)
        ).collect()).collect(), generator.name())
    }).collect();

    outputters.iter().foreach(|outputter| outputter.output(&animations, w, h));
}

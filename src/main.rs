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
    fn generate(&self, w: u8, h: u8, n: u8, x: u8, _y: u8) -> Rgb {
        let half_steps = (self.steps(w, h) / 2) as i8;
        let value = match -(n as i8 - half_steps).abs() + half_steps as i8 == x as i8 {
            true => 255,
            false => 0,
        };
        Rgb(value, value, value)
    }

    fn name(&self) -> &str {
        "horiz_wave"
    }

    fn steps(&self, w: u8, _h: u8) -> u8 {
        (w-1) * 2
    }
}

struct HorizDblWave;
impl Generate for HorizDblWave {
    fn generate(&self, w: u8, _h: u8, n: u8, x: u8, _y: u8) -> Rgb {
        let value = match x == n || w - x == n + 1 {
            true => 255,
            false => 0,
        };
        Rgb(value, value, value)
    }

    fn name(&self) -> &str {
        "horiz_dbl_wave"
    }

    fn steps(&self, w: u8, _h: u8) -> u8 {
        w - 1
    }
}

struct VertWave;
impl Generate for VertWave {
    fn generate(&self, w: u8, h: u8, n: u8, _x: u8, y: u8) -> Rgb {
        let half_steps = (self.steps(w, h) / 2) as i8;
        let value = match -(n as i8 - half_steps).abs() + half_steps as i8 == y as i8 {
            true => 255,
            false => 0,
        };
        Rgb(value, value, value)
    }

    fn name(&self) -> &str {
        "vert_wave"
    }

    fn steps(&self, _w: u8, h: u8) -> u8 {
        (h-1) * 2
    }
}

struct VertDblWave;
impl Generate for VertDblWave {
    fn generate(&self, _w: u8, h: u8, n: u8, _x: u8, y: u8) -> Rgb {
        let value = match y == n || h - y == n + 1 {
            true => 255,
            false => 0,
        };
        Rgb(value, value, value)
    }

    fn name(&self) -> &str {
        "vert_dbl_wave"
    }

    fn steps(&self, _w: u8, h: u8) -> u8 {
        h - 1
    }
}


const ANIM2015: [[u8; 9]; 9] = [
    [0, 1, 1, 0, 0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0, 1, 1, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 1, 1, 0],
    [0, 0, 1, 0, 0, 1, 1, 1, 0],
];
struct Anim2015;
impl Generate for Anim2015 {
    fn generate(&self, w: u8, h: u8, n: u8, x: u8, y: u8) -> Rgb {
        let half_steps = (self.steps(w, h) / 2) as i8;
        let value = match ANIM2015[y as usize][x as usize] == 1 {
            true => (255f64 * ((-(n as i8 - half_steps).abs() + half_steps) as f64 / half_steps as f64)) as u8,
            false => 0,
        };
        Rgb(value, value, value)
    }

    fn name(&self) -> &str {
        "2015"
    }

    fn steps(&self, _w: u8, _h: u8) -> u8 {
        16
    }
}


fn main() {
    let w = 9;
    let h = 9;
    let generators: Vec<Box<Generate>> = vec![
        Box::new(HorizWave),
        Box::new(HorizDblWave),
        Box::new(VertWave),
        Box::new(VertDblWave),
        Box::new(Anim2015),
    ];
    let outputters: Vec<Box<Output>> = vec![Box::new(ImageOutput)];

    let animations = generators.iter().map(|generator| {
        Animation::new((0..generator.steps(w, h)).map(|n| iproduct!(0..w, 0..h).map(
            |(x, y)| generator.generate(w, h, n, x, y)
        ).collect()).collect(), generator.name())
    }).collect();

    outputters.iter().foreach(|outputter| outputter.output(&animations, w, h));
}

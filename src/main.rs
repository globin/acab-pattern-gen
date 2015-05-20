#![feature(core)]

extern crate core;
extern crate image;
#[macro_use] extern crate itertools;

use core::array::FixedSizeArray;
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


const CHESS: [[u8; 9]; 9] = [
    [1, 0, 1, 0, 1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1, 0, 1, 0, 1],
];
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
struct Fade {
    pattern: Vec<Vec<u8>>,
    name: String,
}
impl Fade {
    fn new(pattern: Vec<Vec<u8>>, name: &str) -> Fade {
        Fade {
            pattern: pattern,
            name: name.to_string()
        }
    }
}
impl Generate for Fade {
    fn generate(&self, w: u8, h: u8, n: u8, x: u8, y: u8) -> Rgb {
        let half_steps = (self.steps(w, h) / 2) as i8;
        let value = match self.pattern[y as usize][x as usize] == 1 {
            true => (255f64 * ((-(n as i8 - half_steps).abs() + half_steps) as f64 / half_steps as f64)) as u8,
            false => 0,
        };
        Rgb(value, value, value)
    }

    fn name(&self) -> &str {
        &self.name[..]
    }

    fn steps(&self, _w: u8, _h: u8) -> u8 {
        16
    }
}

const UNITY_SCROLL: [[u8; 31]; 9] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const ANIM2015SCROLL: [[u8; 22]; 9] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];
struct Scroll {
    pattern: Vec<Vec<u8>>,
    name: String,
}
impl Scroll {
    fn new(pattern: Vec<Vec<u8>>, name: &str) -> Scroll {
        Scroll {
            pattern: pattern,
            name: name.to_string()
        }
    }
}
impl Generate for Scroll {
    fn generate(&self, w: u8, h: u8, n: u8, x: u8, y: u8) -> Rgb {
        let value = match self.pattern[y as usize][((x + n) % self.steps(w, h)) as usize] == 1 {
            true => 255,
            false => 0,
        };
        Rgb(value, value, value)
    }

    fn name(&self) -> &str {
        &self.name[..]
    }

    fn steps(&self, _w: u8, _h: u8) -> u8 {
        self.pattern[0].len() as u8
    }
}

struct GameOfLife;
impl Generate for GameOfLife {
    fn generate(&self, w: u8, h: u8, n: u8, x: u8, y: u8) -> Rgb {
        Rgb(0, 0, 0)
    }

    fn name(&self) -> &str {
        "game_of_life"
    }

    fn steps(&self, _w: u8, _h: u8) -> u8 {
        16
    }
}

fn main() {
    let anim2015scroll: Vec<Vec<u8>> = ANIM2015SCROLL.to_owned().iter().map(
        |inner| inner.as_slice().to_owned()).collect();
    let unity_scroll: Vec<Vec<u8>> = UNITY_SCROLL.to_owned().iter().map(
        |inner| inner.as_slice().to_owned()).collect();
    let anim2015fade: Vec<Vec<u8>> = ANIM2015.to_owned().iter().map(
        |inner| inner.as_slice().to_owned()).collect();
    let chess_fade: Vec<Vec<u8>> = CHESS.to_owned().iter().map(
        |inner| inner.as_slice().to_owned()).collect();

    let w = 9;
    let h = 9;
    let generators: Vec<Box<Generate>> = vec![
        Box::new(HorizWave),
        Box::new(HorizDblWave),
        Box::new(VertWave),
        Box::new(VertDblWave),
        Box::new(Fade::new(anim2015fade, "2015")),
        Box::new(Fade::new(chess_fade, "chess")),
        Box::new(Scroll::new(anim2015scroll, "2015_scroll")),
        Box::new(Scroll::new(unity_scroll, "unity_scroll")),
    ];
    let outputters: Vec<Box<Output>> = vec![Box::new(ImageOutput)];

    let animations = generators.iter().map(|generator| {
        Animation::new((0..generator.steps(w, h)).map(|n| iproduct!(0..w, 0..h).map(
            |(x, y)| generator.generate(w, h, n, x, y)
        ).collect()).collect(), generator.name())
    }).collect();

    outputters.iter().foreach(|outputter| outputter.output(&animations, w, h));
}

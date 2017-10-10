extern crate image;

mod interpreter;
mod colors;
mod ops;

use std::path::Path;
use image::{GenericImage, DynamicImage};
use image::Rgba;

use interpreter::Interpreter;

pub fn run(filename: &str) {
    let path = Path::new(filename);
    let image = loadfile(&path);
    let mut interp = Interpreter::new(image);

    interp.run();
}

fn loadfile(path: &Path) -> DynamicImage {
    image::open(path).expect("Could not open image file")
}
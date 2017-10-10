extern crate rustpiet;

use std::env;

fn main() {
    let file = env::args().last().unwrap();
    rustpiet::run(&file);
}

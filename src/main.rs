use std::fs;
use std::env;
mod interp;

use crate::interp::interp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Error! File not found.");

    interp(&contents);
}

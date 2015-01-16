#![feature(core)]
#![feature(libc)]

extern crate ncurses;
extern crate rand;

mod matrix;
mod rain;

fn main() {
    let mut m = matrix::Matrix::new();
    m.run();
}

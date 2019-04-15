use std::env;

mod viewer;

#[macro_use]
extern crate stdweb;

fn main() {
    viewer::show();
}
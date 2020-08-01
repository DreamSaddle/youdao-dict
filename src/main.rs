#![allow(non_snake_case, non_camel_case_types)]

extern crate reqwest;
extern crate futures;
extern crate serde;
extern crate regex;


#[macro_use]
mod utils;
mod structs;
mod gui;

use crate::gui::startQt::StartQt;


#[tokio::main]
async fn main() {
    StartQt::start();
}

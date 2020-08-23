#![allow(non_snake_case, non_camel_case_types)]

extern crate reqwest;
extern crate futures;
extern crate serde;
extern crate regex;
extern crate inputbot;
extern crate notify_rust;

#[macro_use]
mod utils;
mod structs;
mod gui;

use std::env;
use notify_rust::Notification;

use crate::gui::startQt::StartQt;
use crate::gui::clip::clipboard::ClipBoard;


#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        //参数启动
        match args.get(1).unwrap().as_str() {
            "clipboard" => {
                ClipBoard::show();
            },
            _ => {
                Notification::new()
                    .summary("错误的启动方式")
                    .body("你试图通过参数启动, 但参数未知.")
                    .show().unwrap();
            },
        }
    } else {
        //正常启动
        StartQt::start();
    }
}

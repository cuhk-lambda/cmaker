#[macro_use]
extern crate lazy_static;

use crate::global::*;
use crate::parsing::parse;

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod abstraction;
mod utils;
mod global;
mod parsing;
mod cli;

fn main() {
    let res = match parse(*WORK_DIR, *PATH) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("failed to parse the file, error: {}", e);
            std::process::exit(1)
        }
    };

    match res.store_encoded(*OUTPUT) {
        Ok(_) => println!("stored to {}", *OUTPUT),
        Err(e) => {
            ("failed to store data, error: {}", e);
            std::process::exit(1)
        }
    }
}
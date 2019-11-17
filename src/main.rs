extern crate jemallocator;

use crate::global::{PATH, WORK_DIR};
use crate::parsing::{Collection, parse};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod abstraction;
mod utils;
mod global;
mod parsing;

fn main() {
//    let res = parse(WORK_DIR, PATH).unwrap();
//    res.store_encoded("/tmp/test").unwrap();
    let res = Collection::load("/tmp/test");
    println!("{:?}", res.unwrap());
}
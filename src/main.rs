#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate lz4;

pub mod client;
pub mod fs;

fn main() {
    client::routes::startup();
}

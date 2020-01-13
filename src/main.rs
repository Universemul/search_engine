#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

pub mod client;

fn main() {
    client::routes::startup()
}

#![feature(proc_macro)]
#![feature(use_extern_macros)]
#[macro_use] extern crate include_dir_macro;

use std::path::Path;

fn main() {
    let hashmap = include_dir_macro::include_dir!("examples/poems");
    for key in hashmap.keys() {
        println!("{}", key.to_string_lossy());
    }
    let nightingale = hashmap.get(Path::new("keats/ode-to-a-nightingale.txt"))
        .and_then(|entry| ::std::str::from_utf8(*entry).ok()).unwrap();
    println!("{}", nightingale);
}

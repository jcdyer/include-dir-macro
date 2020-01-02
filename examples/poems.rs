#![feature(proc_macro_hygiene)]

//! This uses the `include_dir!()` procedural macro.  When run, it prints out the
//! names of all the files in examples/poems, and then prints the contents of
//! one of the files in the directory.
//!
//! Usage:
//!
//!     cargo run --example poems

use std::path::Path;
use std::str::from_utf8;

use include_dir_macro::include_dir;

fn main() {
    let hashmap = include_dir!("examples/static/poems");
    for key in hashmap.keys() {
        println!("{}", key.to_string_lossy());
    }
    let nightingale = Path::new("keats/ode-to-a-nightingale.txt");
    println!("{}", nightingale.to_string_lossy());
    let nightingale_text = hashmap.get(nightingale)
        .and_then(|entry| from_utf8(*entry).ok()).unwrap();
    println!("{}", nightingale_text);
}

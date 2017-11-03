#![feature(proc_macro)]
#![feature(use_extern_macros)]
#[macro_use] extern crate include_dir_macro;

use std::path::PathBuf;

fn main() {
    let hashmap = include_dir_macro::include_dir!("examples/poems");
    for key in hashmap.keys() {
        println!("{}", key.to_string_lossy());
    }
    println!("{}", hashmap.get(&PathBuf::from("keats/ode-to-a-nightingale.txt")).unwrap());
}

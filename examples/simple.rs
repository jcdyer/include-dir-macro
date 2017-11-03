//! This example uses the custom-derive for the `IncludeDir` trait.
//! 
//! When run, it prints the entire contents of the `examples/static` directory
//! as a string.
//! 
//! Usage:
//! 
//!    cargo run --example simple

extern crate include_dir; #[macro_use] extern crate include_dir_derive;

use include_dir::IncludeDir;

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(IncludeDir)]
#[dir = "examples/static"]
struct StaticFiles {
    pub str_files: HashMap<PathBuf, &'static str>,
    pub bytes_files: HashMap<PathBuf, &'static [u8]>,
}

impl StaticFiles {
    fn new() -> StaticFiles {
        let mut staticfiles = StaticFiles { 
            str_files: HashMap::new(),
            bytes_files: HashMap::new(),
        };
        let files = staticfiles.construct_str_hash();
        staticfiles.str_files = files;
        staticfiles
    }
}

fn main() {
    let staticfiles = StaticFiles::new();
    println!("{:?}", staticfiles.str_files);
}

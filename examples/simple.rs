extern crate include_dir;
#[macro_use] extern crate include_dir_derive;

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
        let mut foo = StaticFiles { 
            str_files: HashMap::new(),
            bytes_files: HashMap::new(),
        };
        let files = foo.construct_str_hash();
        foo.str_files = files;
        foo
    }
}

fn main() {
    let staticfiles = StaticFiles::new();
    println!("{:?}", staticfiles.str_files);
}

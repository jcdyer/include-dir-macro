#[macro_use] extern crate include_dir_derive;

use std::collections::HashMap;

trait IncludeDir {
    fn construct_str_hash(&mut self) -> HashMap<&'static str, &'static str>;
    fn construct_bytes_hash(&mut self) -> HashMap<&'static str, &'static [u8]>;
}

#[derive(IncludeDir)]
//#[dir = "static"]
#[dir = "/home/cliff/work/opencraft/docker/edx-platform/lms/djangoapps/completion"]
struct Foo {
    str_files: HashMap<&'static str, &'static str>,
    bytes_files: HashMap<&'static str, &'static [u8]>,
}

impl Foo {
    fn new() -> Foo {
        let mut foo = Foo { 
            str_files: HashMap::new(),
            bytes_files: HashMap::new(),
        };
        let files = foo.construct_str_hash();
        foo.str_files = files;
        foo
    }
}

fn main() {
    let mut f = Foo::new();
    println!("{:?}", f.str_files);
}

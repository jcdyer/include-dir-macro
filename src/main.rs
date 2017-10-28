#[macro_use] extern crate include_dir_derive;

use std::collections::HashMap;

trait IncludeDir {
    fn construct_hash(&mut self) -> HashMap<&'static str, &'static str>;
}

#[derive(IncludeDir)]
#[dir = "/home/cliff/work/opencraft/docker/edx-platform/lms/djangoapps/completion"]
struct Foo {
    files: HashMap<&'static str, &'static str>,
}

impl Foo {
    fn new() -> Foo {
        let mut foo = Foo { files: HashMap::new() };
        let files = foo.construct_hash();
        foo.files = files;
        foo
    }
}

fn main() {
    let mut f = Foo::new();
    println!("{:?}", f.files);
}

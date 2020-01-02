#![feature(proc_macro_hygiene, decl_macro)]


use std::collections::HashMap;
use std::path::{Path, PathBuf};

use include_dir_macro::include_dir;  // proc-macro
use rocket::{Response, State};
use rocket::http::{ContentType, Status};


struct StaticFiles {
    files: HashMap<&'static Path, &'static [u8]>
}

fn expected_type(mimetype: &str, input: &[u8]) -> Option<String> {
    if tree_magic::match_u8(mimetype, input) {
        Some(mimetype.to_owned())
    } else {
        None
    }
}

#[derive(Debug)]
struct InvalidFile(PathBuf);

impl StaticFiles {
    pub fn new(files: HashMap<&'static Path, &'static [u8]>) -> StaticFiles {
        StaticFiles { files: files }
    }

    pub fn get_raw(&self, path: &Path) -> Option<&'static [u8]>{
        self.files.get(&path).map(|x| *x)
    }

    pub fn get_response<'r>(&'r self, path: &Path) -> Option<Result<rocket::Response<'r>, InvalidFile>> {
        match self.get_raw(path) {
            None => None,
            Some(raw) => {
                let extension = path.extension().and_then(|ext| ext.to_str());
                let filetype = match extension {
                    Some("png") => expected_type("image/png", raw),
                    Some("jpg") | Some("jpeg") => expected_type("image/jpeg", raw),
                    Some("gif") => expected_type("image/gif", raw),
                    Some("js") => expected_type("application/javascript", raw),
                    Some("json") => expected_type("text/json", raw),
                    Some("html") => expected_type("text/html", raw),
                    _ => Some(tree_magic::from_u8(raw)),
                }.ok_or_else(|| InvalidFile(path.to_owned()));
                Some(
                    filetype.map(|filetype| {
                        rocket::Response::build()
                            .status(Status::Ok)
                            .header(filetype.parse::<ContentType>().expect("valid mimetype"))
                            .sized_body(::std::io::Cursor::new(raw.to_owned()))
                            .finalize()
                    })
                )
            },
        }
    }
}

use rocket::{get, routes};

#[get("/")]
fn hello() -> &'static str {
    "Hello world"
}

#[get("/static/<path..>")]
fn staticfiles<'r>(path: PathBuf, store: State<'r, StaticFiles>) -> Option<Result<Response<'r>, InvalidFile>> {
    store.inner().get_response(&path)
} 

#[get("/raw/<path..>")]
fn rawfiles(path: PathBuf, store: State<StaticFiles>) -> Option<&'static str> {
    store.get_raw(&path).map(|data| ::std::str::from_utf8(data).unwrap())
}

fn main() {
    rocket::ignite()
        .manage(StaticFiles::new(include_dir!("examples/static/web")))
        .mount("/", routes![
               hello,
               staticfiles,
               rawfiles
        ])
        .launch();
}

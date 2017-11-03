use std::collections::HashMap;
use std::path::{PathBuf};

pub trait IncludeDir {
    fn construct_str_hash(&mut self) -> HashMap<PathBuf, &'static str>;
    fn construct_bytes_hash(&mut self) -> HashMap<PathBuf, &'static [u8]>;
}

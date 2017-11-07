# include-dir-macro

## Quick start:

Using a nightly toolchain, add this to your Cargo.toml:

    [dependencies]
    include-dir-macro = "0.1"

Then at the root of your crate (in `main.rs` or `lib.rs`), add the following:

    #[feature(proc_macro)]
    extern crate include-dir-macro;

Finally, you can call the macro as:

    include_dir("path/to/directory");

If the path is a relative path, it will be interpreted relative to the
directory from which `cargo` or `rustc` was invoked 

## Description

Provides an `include_dir!()` macro, which returns a `HashMap<PathBuf, &'static
[u8]>` mapping files within a given directory to the contents of the file,
stored within the built executable as static byte arrays.  Given a crate with a
directory structure like this:

    /root
    +-Cargo.toml
    +-src/
    | \-main.rs
    |   
    |       #[feature(proc_macro)]
    |       extern crate include_dir_macro;
    |       fn main() { 
    |           let stat = include_dir!("static");
    |       }
    |   
    \-static/
      +-this
      |
      |     ABC
      |
      +-that.html
      |
      |     <p>123</p>
      |
      \-path/
        \-to/
          \-theother.txt

                Do 
                re
                mi.

The value of stat will be the same as if main.rs included:

    use std::collections::HashMap;
    use std::path::PathBuf

    fn main() {
        let stat = HashMap::new()
        stat.insert(PathBuf::from("this"), b"ABC\n");
        stat.insert(PathBuf::from("that.html"), b"<p>123</p>\n");
        stat.insert(PathBuf::from("path/to/theother.txt"), b"Do\nre\nmi.\n");
    }
       
## Rationale

Out of the box, Rust provides a macro called `include_bytes!()`, which loads
the contents of a file into a static byte array at compile time.  The contents
then live in the binary, requiring no further access to the filesystem.  I
thought this could be useful for bundling up entire directories, to provide a
way to make, for example, whole websites bundled as a static binary, containing
all associated images, stylesheets, and scripts.  

One shortcoming of of `include_bytes!()` for this use case is that it only
operates on paths specified as str literals, so to use it for an entire
directory, one would have to add each file by hand.  I saw two ways around
this:

One is to include code-generation in a crate's `build.rs` script that crawls
the included directory, and adds an include_str for each file.  This method is
supported by another crate called "include_dir", but I wanted an approach with
the ergonomic simplicity of the `include_bytes!()` macro.  This required using
procedural macros, which in turn means that it currently (as of Rust 1.21)
rustonly works on nightly releases.

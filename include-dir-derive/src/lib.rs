#![recursion_limit = "128" ]
extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use std::path::{Path, PathBuf};
use std::str;
use proc_macro::TokenStream;
use syn::{Token, Lit, StrStyle};

#[proc_macro_derive(IncludeDir, attributes(dir))]
pub fn derive_include_dir(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_include_dir(&ast);
    gen.parse().unwrap()
}


fn get_files<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut files = vec![];
    let listing: Vec<_> = ::std::fs::read_dir(dir).expect("could not read directory")
        .map(|entry| entry.unwrap().path()).collect();
    for path in listing {
        if path.is_file() {
            files.push(path)
        } else if path.is_dir() {
            for file in get_files(&path) {
                files.push(file)
            }
        }
    }
    files
}

fn get_path_from_attr(attr: &syn::Attribute) -> Option<&Path> {
    let dirvalue = match attr.value {
        syn::MetaItem::NameValue(_, ref value) => value,
        _ => panic!("attribute must be of form #[dir = \"path/to/dir\""),
    };
    let maybepath: Option<&str> = match *dirvalue {
        Lit::Str(ref val, _) => Some(val),
        Lit::ByteStr(ref val, _) => str::from_utf8(val).ok(),
        _ => None
    };
    maybepath.map(Path::new)
}


fn path_to_str_literal<P: AsRef<Path>>(path: P) -> Token {
    Token::Literal(Lit::Str(path.as_ref().to_str().unwrap().to_owned(), StrStyle::Cooked))
}


fn impl_include_dir(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let dir = if let Some(ref dirattr) = ast.attrs.iter().filter(|&attr| attr.name() == "dir").next() {
        get_path_from_attr(dirattr).unwrap()
    } else {
        panic!("ONO");
    };
    let paths: Vec<_> = get_files(dir);

    let keys: Vec<_> = paths.iter()
        .map(|path| path.strip_prefix(dir).unwrap())
        .map(path_to_str_literal)
        .collect();

    let vals: Vec<_> = paths.iter()
        .map(|path| ::std::fs::canonicalize(path).expect("found"))
        .map(path_to_str_literal)
        .collect();
    
    let keys2 = keys.clone();
    let vals2 = vals.clone();

    quote! {
        impl IncludeDir for #name {
            fn construct_str_hash(&mut self) -> ::std::collections::HashMap<::std::path::PathBuf, &'static str> {
                let mut hashmap = ::std::collections::HashMap::new();
                #( hashmap.insert(::std::path::PathBuf::from(#keys), include_str!(#vals)); )*
                hashmap
            }

            fn construct_bytes_hash(&mut self) -> ::std::collections::HashMap<::std::path::PathBuf, &'static [u8]> {
                let mut hashmap = ::std::collections::HashMap::new();
                #( hashmap.insert(::std::path::PathBuf::from(#keys2), &include_bytes!(#vals2)[..]); )*
                hashmap
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

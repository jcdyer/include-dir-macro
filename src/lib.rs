#![recursion_limit = "128"]
extern crate proc_macro;

#[macro_use]
extern crate quote;

use std::path::{Path, PathBuf};
use std::str;
use proc_macro::TokenStream;

use syn::{Lit, StrStyle, Token, TokenTree, parse_token_trees};


#[proc_macro]
pub fn include_dir(input: TokenStream) -> TokenStream {
    let foo = input.to_string();
    let args = parse_token_trees(&foo).unwrap();
    let gen = impl_include_dir(args).unwrap();
    gen.parse().unwrap()
}


fn get_files<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut files = vec![];
    let listing: Vec<_> = ::std::fs::read_dir(dir)
        .expect("could not read directory")
        .map(|entry| entry.unwrap().path())
        .collect();
    for path in listing {
        if path.is_file() {
            files.push(path)
        } else if path.is_dir() {
            for file in get_files(&path) {
                files.push(file.into())
            }
        }
    }
    files
}


fn path_to_str_literal<P: AsRef<Path>>(path: P) -> Token {
    Token::Literal(Lit::Str(
        path.as_ref().to_str().unwrap().to_owned(),
        StrStyle::Cooked,
    ))
}

fn get_path_from_args(args: Vec<TokenTree>) -> Result<PathBuf, &'static str> {
    match args.len() {
        0 => Err("empty"),
        1 => {
            let nexttree = args.into_iter().next().unwrap();
            match nexttree {
                TokenTree::Token(Token::Literal(Lit::Str(ref val, ..))) => Ok(val.into()),
                _ => Err("not str"),
            }
        }
        _ => Err("multiple trees"),
    }
}


fn impl_include_dir(args: Vec<TokenTree>) -> Result<quote::Tokens, &'static str> {
    let dir = get_path_from_args(args)?;
    let paths: Vec<_> = get_files(&dir);

    let keys: Vec<_> = paths
        .iter()
        .map(|path| path.strip_prefix(&dir).unwrap())
        .map(path_to_str_literal)
        .collect();

    let vals: Vec<_> = paths
        .iter()
        .map(|path| ::std::fs::canonicalize(path).expect("found"))
        .map(path_to_str_literal)
        .collect();

    Ok(quote! {
        {
            let mut __include_dir_hashmap = ::std::collections::HashMap::new();
            #( __include_dir_hashmap.insert(::std::path::Path::new(#keys), &include_bytes!(#vals)[..]); )*
            __include_dir_hashmap
        }
    })
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

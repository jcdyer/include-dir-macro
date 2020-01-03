#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use] extern crate quote;

use std::path::{Path, PathBuf};
use std::str;
use proc_macro2::{Span, TokenStream, TokenTree, Literal};

use syn::{Lit, StrStyle, token::Token};
use syn::{parse2, parse::ParseStream};



#[proc_macro]
pub fn include_dir(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    proc_macro::TokenStream::from(impl_include_dir(input).unwrap())
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


fn path_to_str_literal<P: AsRef<Path>>(path: P) -> Lit {
    Lit::Str(syn::LitStr::new(
        path.as_ref().to_str().unwrap(),
        Span::call_site(),
    ))
}

fn get_path_from_args(args: Vec<TokenTree>) -> syn::parse::Result<PathBuf> {
    let path = parse2::<Literal>(nexttree)?.into();
    
    Ok(path)
    match args.len() {
        0 => Err("empty".into()),
        1 => {
            let nexttree = args.into_iter().next().unwrap();
            let lit: Literal = parse2(nexttree)?;
            lit.into()
            /*
            match nexttree {
                TokenTree::Literal(Lit::Str(ref val, ..)) => Ok(val.into()),
                _ => Err("not str".into()),
            }
            */
        }
        _ => Err("multiple trees".into()),
    }
}


fn impl_include_dir(args: TokenStream) -> Result<TokenStream, &'static str> {
    let dir = get_path_from_args(args.into_iter().collect())?;
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

use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::LitStr;

pub(super) struct StaticFilesDirInput {
    pub(super) path: String,
}

impl Parse for StaticFilesDirInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse::<LitStr>()?.value();
        Ok(Self { path })
    }
}

pub(crate) fn static_files_dir(base: &str, inner: &str) -> TokenStream {
    let entries = get_files(base, inner);

    quote!(static_files!(
        #(
            #entries,
        )*
    ))
}

fn get_files(base: &str, inner: &str) -> Vec<String> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let path = Path::new(&manifest_dir).join(base).join(inner);

    #[cfg(feature = "nightly")]
    {
        let path_str = path.to_str().expect("path is not valid UTF-8");
        proc_macro::tracked_path::path(path_str);
    }

    if !path.exists() {
        panic!("Directory does not exist: {}", path.display());
    }

    let mut entries = vec![];
    for entry in std::fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    entries.push(
                        Path::new(inner)
                            .join(file_name_str)
                            .to_string_lossy()
                            .into_owned(),
                    );
                }
            }
        }
    }

    entries
}

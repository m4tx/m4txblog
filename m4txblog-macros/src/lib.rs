#![cfg_attr(feature = "nightly", feature(track_path))]

use proc_macro::TokenStream;

use crate::md_pages::MdPageInput;
use crate::static_files_dir::StaticFilesDirInput;

mod md_pages;
mod static_files_dir;

#[proc_macro]
pub fn md_page(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let MdPageInput { link } = syn::parse2(input).unwrap();

    let md_page = md_pages::parse_md_page(&link);
    md_pages::quote_md_page(&md_page).into()
}

#[proc_macro]
pub fn static_files_dir(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let StaticFilesDirInput { path } = syn::parse2(input).unwrap();

    static_files_dir::static_files_dir("static/", &path).into()
}

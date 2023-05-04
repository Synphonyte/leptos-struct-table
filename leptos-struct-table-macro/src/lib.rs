mod models;
mod table;

use darling::FromDeriveInput;
use models::TableComponentDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(TableComponent, attributes(table))]
pub fn derive_table_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let data = TableComponentDeriveInput::from_derive_input(&input).expect("Wrong options");
    let stream = quote!(#data);
    stream.into()
}

mod record;
mod table;
use crate::record::RecordDeriveInput;
use crate::table::TableComponentDeriveInput;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(Record)]
pub fn derive_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let data = RecordDeriveInput::from_derive_input(&input).expect("Wrong options");
    let stream = quote!(#data);
    stream.into()
}

#[proc_macro_derive(TableComponent, attributes(table))]
pub fn derive_table_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let data = TableComponentDeriveInput::from_derive_input(&input).expect("Wrong options");
    let stream = quote!(#data);
    stream.into()
}

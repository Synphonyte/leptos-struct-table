#![allow(non_snake_case)]

mod class_providers;
mod components;
mod data_provider;

pub use class_providers::*;
pub use components::*;
pub use data_provider::*;
pub use leptos_struct_table_macro::TableComponent;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColumnSort {
    Ascending,
    Descending,
    None,
}

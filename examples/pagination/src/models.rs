use crate::tailwind::TailwindClassesPreset;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

#[derive(TableRow, Serialize, Deserialize, Clone, Debug)]
#[table(sortable, classes_provider = "TailwindClassesPreset")]
pub struct Brewery {
    #[table(skip)]
    pub id: String,

    pub name: String,

    pub brewery_type: String,

    pub city: String,

    pub country: String,
}

#[derive(Deserialize, Debug)]
pub struct MetaResponse {
    pub total: String,
    pub page: String,
    pub per_page: String,
}
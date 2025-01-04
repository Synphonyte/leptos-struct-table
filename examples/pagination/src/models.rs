use crate::tailwind::TailwindClassesPreset;
use leptos::IntoView;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
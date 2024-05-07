#![allow(unused_variables)]
#![doc(cfg(feature = "chrono"))]

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use leptos::*;

#[derive(Default)]
pub struct RenderChronoOptions {
    pub string: Option<String>,
}

impl crate::cell_value::CellValue for NaiveDate {
    type RenderOptions = RenderChronoOptions;
    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.string.as_ref() {
            self.format(&value).to_string()
        } else {
            self.to_string()
        }
    }
}
impl crate::cell_value::CellValue for NaiveDateTime {
    type RenderOptions = RenderChronoOptions;
    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.string.as_ref() {
            self.format(&value).to_string()
        } else {
            self.to_string()
        }
    }
}

impl crate::cell_value::CellValue for NaiveTime {
    type RenderOptions = RenderChronoOptions;
    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.string.as_ref() {
            self.format(&value).to_string()
        } else {
            self.to_string()
        }
    }
}
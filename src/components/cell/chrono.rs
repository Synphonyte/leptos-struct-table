#![allow(unused_variables)]
#![doc(cfg(feature = "chrono"))]

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use leptos::*;

impl crate::CellValue for NaiveDate {
    fn render_value(self, options: &crate::RenderOptions) -> impl IntoView {
        if let Some(value) = options.format_string.as_ref() {
            view! {
                <>{self.format(&value).to_string()}</>
            }
        } else {
            view! {
                <>{self.to_string()}</>
            }
        }
    }
}
impl crate::CellValue for NaiveDateTime {
    fn render_value(self, options: &crate::RenderOptions) -> impl IntoView {
        if let Some(value) = options.format_string.as_ref() {
            view! {
                <>{self.format(&value).to_string()}</>
            }
        } else {
            view! {
                <>{self.to_string()}</>
            }
        }
    }
}

impl crate::CellValue for NaiveTime {
    fn render_value(self, options: &crate::RenderOptions) -> impl IntoView {
        if let Some(value) = options.format_string.as_ref() {
            view! {
                <>{self.format(&value).to_string()}</>
            }
        } else {
            view! {
                <>{self.to_string()}</>
            }
        }
    }
}

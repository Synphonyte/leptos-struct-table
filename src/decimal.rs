#![allow(unused_variables)]
#![doc(cfg(feature = "decimal"))]
use rust_decimal::Decimal;
use leptos::*;
#[derive(Default)]
pub struct DecimalNumberRenderOptions {
    pub precision: Option<usize>,
}

impl crate::cell_value::CellValue for Decimal {
    type RenderOptions = DecimalNumberRenderOptions;
    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.precision.as_ref() {
            view! {
                <>{format!("{:.value$}", self)}</>
            }
        }
        else {
            view! {
                <>{self.to_string()}</>
            }
        }
    }
}
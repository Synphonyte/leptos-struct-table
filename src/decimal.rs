#![allow(unused_variables)]
#![doc(cfg(feature = "decimal"))]
use rust_decimal::Decimal;
use leptos::*;
#[derive(Default)]
pub struct DecimalNumberRenderOptions {
    pub precision: Option<usize>,
}
/// CellValue implementation for uuid for uuid to work with the TableRow derive
/// ``` 
/// # use leptos_struct_table::*;
/// # use leptos::*;
/// # use rust_decimal::Decimal;
/// 
/// #[derive(TableRow, Clone)]
/// #[table]
/// struct SomeStruct {
///     #[table(format(precision = 2usize))]
///     my_field: Decimal
/// }
/// ```
impl crate::cell_value::CellValue for Decimal {
    type RenderOptions = DecimalNumberRenderOptions;
    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.precision.as_ref() {
            format!("{:.value$}", self)
        }
        else {
           self.to_string()
        }
    }
}
//! Support for [::rust_decimal] crate.
use crate::*;
use ::rust_decimal::Decimal;
use leptos::*;

#[derive(Default)]
pub struct DecimalNumberRenderOptions {
    /// Specifies the number of digits to display after the decimal point
    pub precision: Option<usize>,
}
/// Implementation for [`Decimal`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
/// ```
/// # use leptos_struct_table::*;
/// # use leptos::*;
/// # use ::rust_decimal::Decimal;
/// #[derive(TableRow, Clone)]
/// #[table]
/// struct SomeStruct {
///     #[table(format(precision = 2usize))]
///     my_field: Decimal
/// }
/// ```
impl CellValue for Decimal {
    type RenderOptions = DecimalNumberRenderOptions;
    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.precision.as_ref() {
            format!("{:.value$}", self)
        } else {
            self.to_string()
        }
    }
}

//! Support for [::time] crate.

use crate::*;
use ::time::format_description;
use ::time::{Date, OffsetDateTime, PrimitiveDateTime, Time};
use leptos::*;

#[derive(Default)]
pub struct RenderTimeOptions {
    /// Specifies a format string see [the time book](https://time-rs.github.io/book/api/format-description.html).
    pub string: Option<String>,
}

/// Implementation for [`Date`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
/// ```
/// # use leptos_struct_table::*;
/// # use leptos::*;
/// # use ::time::Date;
/// #[derive(TableRow, Clone)]
/// #[table]
/// struct SomeStruct {
///     #[table(format(string = "[year]-[month]-[day]"))]
///     my_field: Date
/// }
/// ```
impl CellValue for Date {
    type RenderOptions = RenderTimeOptions;

    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.string.as_ref() {
            let format = format_description::parse(value)
                .expect("Unable to construct a format description given the format string");
            self.format(&format)
                .expect("Unable to format given the format description")
        } else {
            self.to_string()
        }
    }
}
/// Implementation for [`Time`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
/// ```
/// # use leptos_struct_table::*;
/// # use leptos::*;
/// # use ::time::Time;
/// #[derive(TableRow, Clone)]
/// #[table]
/// struct SomeStruct {
///     #[table(format(string = "[hour]:[minute]:[second]"))]
///     my_field: Time
/// }
/// ```
impl CellValue for Time {
    type RenderOptions = RenderTimeOptions;

    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.string.as_ref() {
            let format = format_description::parse(value)
                .expect("Unable to construct a format description given the format string");
            self.format(&format)
                .expect("Unable to format given the format description")
        } else {
            self.to_string()
        }
    }
}

/// Implementation for [`PrimitiveDateTime`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
/// ```
/// # use leptos_struct_table::*;
/// # use leptos::*;
/// # use ::time::PrimitiveDateTime;
/// #[derive(TableRow, Clone)]
/// #[table]
/// struct SomeStruct {
///     #[table(format(string = "[year]-[month]-[day] [hour]:[minute]:[second]"))]
///     my_field: PrimitiveDateTime
/// }
/// ```
impl CellValue for PrimitiveDateTime {
    type RenderOptions = RenderTimeOptions;

    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.string.as_ref() {
            let format = format_description::parse(value)
                .expect("Unable to construct a format description given the format string");
            self.format(&format)
                .expect("Unable to format given the format description")
        } else {
            self.to_string()
        }
    }
}

/// Implementation for [`OffsetDateTime`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
/// ```
/// # use leptos_struct_table::*;
/// # use leptos::*;
/// # use ::time::OffsetDateTime;
/// #[derive(TableRow, Clone)]
/// #[table]
/// struct SomeStruct {
///     #[table(format(string = "[year]-[month]-[day] [hour]:[minute]:[second] Z[offset_hour]"))]
///     my_field: OffsetDateTime
/// }
/// ```
impl CellValue for OffsetDateTime {
    type RenderOptions = RenderTimeOptions;

    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.string.as_ref() {
            let format = format_description::parse(value)
                .expect("Unable to construct a format description given the format string");
            self.format(&format)
                .expect("Unable to format given the format description")
        } else {
            self.to_string()
        }
    }
}

//! Support for [::chrono] crate.

use crate::*;
use ::chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use leptos::*;

#[derive(Default)]
pub struct RenderChronoOptions {
    /// Specifies a format string, See [`::chrono::format::strftime`] for more information.
    pub string: Option<String>,
}

macro_rules! chrono_cell_value_impl {
    (
        $(#[$outer:meta])*
        $ty:ty
    ) => {
        $(#[$outer])*
        impl CellValue for $ty {
            type RenderOptions = RenderChronoOptions;

            fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
                if let Some(value) = options.string.as_ref() {
                    self.format(&value).to_string()
                } else {
                    self.to_string()
                }
            }
        }
    };
}

chrono_cell_value_impl!(
    /// Implementation for [`NaiveDate`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
    /// ```
    /// # use leptos_struct_table::*;
    /// # use leptos::*;
    /// # use ::chrono::NaiveDate;
    /// #[derive(TableRow, Clone)]
    /// #[table]
    /// struct SomeStruct {
    ///     #[table(format(string = "%Y-%m-%d"))]
    ///     my_field: NaiveDate
    /// }
    /// ```
    NaiveDate
);

chrono_cell_value_impl!(
    /// Implementation for [`NaiveDateTime`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
    /// ```
    /// # use leptos_struct_table::*;
    /// # use leptos::*;
    /// # use ::chrono::NaiveDateTime;
    /// #[derive(TableRow, Clone)]
    /// #[table]
    /// struct SomeStruct {
    ///     #[table(format(string = "%Y-%m-%d %H:%M:%S"))]
    ///     my_field: NaiveDateTime
    /// }
    /// ```
    NaiveDateTime
);

chrono_cell_value_impl!(
    /// Implementation for [`NaiveTime`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
    /// ```
    /// # use leptos_struct_table::*;
    /// # use leptos::*;
    /// # use ::chrono::NaiveTime;
    /// #[derive(TableRow, Clone)]
    /// #[table]
    /// struct SomeStruct {
    ///     #[table(format(string = "%H:%M:%S"))]
    ///     my_field: NaiveTime
    /// }
    /// ```
    NaiveTime
);

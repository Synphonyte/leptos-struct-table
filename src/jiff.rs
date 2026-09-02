//! Support for [::jiff] crate.

use crate::*;
use ::jiff::{
    Timestamp, Zoned,
    civil::{Date, DateTime, Time},
};
use leptos::prelude::*;

#[derive(Clone, Default)]
pub struct RenderJiffOptions {
    /// Specifies a format string, See [`::jiff::fmt::strtime`] for more information.
    pub string: Option<String>,
}

macro_rules! jiff_cell_value_impl {
    (
        $(#[$outer:meta])*
        $ty:ty
    ) => {
        $(#[$outer])*
        impl CellValue<$ty> for $ty {
            type RenderOptions = RenderJiffOptions;

            fn render_value(self, options: Self::RenderOptions) -> impl IntoView {
                if let Some(value) = options.string.as_ref() {
                    self.strftime(value).to_string()
                } else {
                    self.to_string()
                }
            }
        }
    };
}

jiff_cell_value_impl!(
    /// Implementation for [`Date`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
    /// ```
    /// # use leptos_struct_table::*;
    /// # use leptos::prelude::*;
    /// # use ::jiff::civil::Date;
    /// #[derive(TableRow, Clone)]
    /// #[table]
    /// struct SomeStruct {
    ///     #[table(format(string = "%Y-%m-%d"))]
    ///     my_field: Date
    /// }
    /// ```
    Date
);

jiff_cell_value_impl!(
    /// Implementation for [`Time`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
    /// ```
    /// # use leptos_struct_table::*;
    /// # use leptos::prelude::*;
    /// # use ::jiff::civil::Time;
    /// #[derive(TableRow, Clone)]
    /// #[table]
    /// struct SomeStruct {
    ///     #[table(format(string = "%H:%M:%S"))]
    ///     my_field: Time
    /// }
    /// ```
    Time
);

jiff_cell_value_impl!(
    /// Implementation for [`DateTime`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
    /// ```
    /// # use leptos_struct_table::*;
    /// # use leptos::prelude::*;
    /// # use ::jiff::civil::DateTime;
    /// #[derive(TableRow, Clone)]
    /// #[table]
    /// struct SomeStruct {
    ///     #[table(format(string = "%Y-%m-%d %H:%M:%S"))]
    ///     my_field: DateTime
    /// }
    /// ```
    DateTime
);

jiff_cell_value_impl!(
    /// Implementation for [`Timestamp`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
    /// ```
    /// # use leptos_struct_table::*;
    /// # use leptos::prelude::*;
    /// # use ::jiff::Timestamp;
    /// #[derive(TableRow, Clone)]
    /// #[table]
    /// struct SomeStruct {
    ///     #[table(format(string = "%Y-%m-%dT%H:%M:%S%z"))]
    ///     my_field: Timestamp
    /// }
    /// ```
    Timestamp
);

jiff_cell_value_impl!(
    /// Implementation for [`Zoned`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
    /// ```
    /// # use leptos_struct_table::*;
    /// # use leptos::prelude::*;
    /// # use ::jiff::Zoned;
    /// #[derive(TableRow, Clone)]
    /// #[table]
    /// struct SomeStruct {
    ///     #[table(format(string = "%Y-%m-%d %H:%M:%S %:Q"))]
    ///     my_field: Zoned
    /// }
    /// ```
    Zoned
);

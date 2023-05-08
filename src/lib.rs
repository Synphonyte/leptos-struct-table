//! Easily create Leptos table components from structs.
//!
//! # Features
//!
//! - **Async data loading** - The data is loaded asynchronously. This allows for loading data from a REST API or a database etc.
//! - **Selectable** - Optional. If turned on: Click on a row to select it. You can select multiple rows (TODO).
//! - **Fully Customizable** - You can customize every aspect of the table by plugging in your own components for rendering rows, cells, headers. See [Custom Renderers](#custom-renderers) for more information.
//! - **Headless** - No default styling is applied to the table. You can fully customize the classes that are applied to the table. See [Classes customization](#classes-customization) for more information.
//! - **Sorting** - Optional. If turned on: Click on a column header to sort the table by that column. You can even sort by multiple columns.
//! - **Virtualization (TODO)** - Only the visible rows are rendered. This allows for very large tables.
//!
//! # Usage
//!
//! ```
//! use leptos::*;
//! use leptos_struct_table::*;
//! use serde::{Deserialize, Serialize};
//! use async_trait::async_trait;
//!
//! // This generates the component PersonTable
//! #[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
//! pub struct Person {
//!     #[table(key)]
//!     id: u32,
//!     name: String,
//!     age: u32,
//! }
//!
//! fn main() {
//!     mount_to_body(|cx| {
//!         // Create a few Person items
//!         let items = create_rw_signal( cx, vec![
//!             Person { id: 1, name: "John".to_string(), age: 32 },
//!             Person { id: 2, name: "Jane".to_string(), age: 28 },
//!             Person { id: 3, name: "Bob".to_string(), age: 45 },
//!         ]);
//!
//!         // Use the generated component
//!         view! { cx,
//!             <PersonTable items=items />
//!         }
//!     });
//! }
//! ```
//!
//! # Macro options
//!
//! The `#[table(...)]` attribute can be used to customize the generated component. The following options are available:
//!
//! ## Struct attributes
//!
//! These attributes can be applied to the struct itself.
//!
//! - **`sortable`** - Specifies that the table should be sortable. This makes the header clickable to toggle sorting.
//! - **`selection_mode`** - Specifies the selection mode. Can be one of `none`, `single`, (TODO: `multiple`). Defaults to `none`.
//!    If given `single` then the generated component has a `selected_key: RwSignal<Option<K>>` property that can be used to get/set the selected key (of type K, the field specified by `#[table(key)]` - see below).
//!    Clicking on a row will set the selected key to the key of that row.
//! - **`component_name`** - Specifies the name of the generated component. Defaults to `StructNameTable`.
//! - **`classes_provider`** - Specifies the name of the class provider. Used to customize the classes that are applied to the table.
//!    For convenience sensible presets for major CSS frameworks are provided. See [`TableClassesProvider`] for more information.
//! - **`tag`** - Specifies the tag that is used as the root element for the table. Defaults to `"table"`.
//! - **`row_renderer`** - Specifies the name of the row renderer component. Used to customize the rendering of rows. Defaults to [`DefaultTableRowRenderer`].
//! - **`head_row_renderer`** - Specifies the name of the head row renderer component/tag. Used to customize the rendering of the head rows Defaults to the tag [`tr`]. This only takes a `class` attribute.
//! - **`head_cell_renderer`** - Specifies the name of the header cell renderer component. Used to customize the rendering of header cells. Defaults to [`DefaultTableHeaderRenderer`].
//! - **`row_class`** - Specifies the classes that are applied to each row. Can be used in conjuction with `classes_provider` to customize the classes.
//! - **`head_row_class`** - Specifies the classes that are applied to the header row. Can be used in conjuction with `classes_provider` to customize the classes.
//!
//! ## Field attributes
//!
//! These attributes can be applied to any field in the struct.
//!
//! - **`key`** - Specifies the field that is used as the key for each row. This is required on exactly one field.
//! - **`class`** - Specifies the classes that are applied to each cell (head and body) in the field's column. Can be used in conjuction with `classes_provider` to customize the classes.
//! - **`head_class`** - Specifies the classes that are applied to the header cell in the field's column. Can be used in conjuction with `classes_provider` to customize the classes.
//! - **`cell_class`** - Specifies the classes that are applied to the body cells in the field's column. Can be used in conjuction with `classes_provider` to customize the classes.
//! - **`skip`** - Specifies that the field should be skipped. This is useful for fields that are not displayed in the table.
//! - **`skip_sort`** - Only applies if `sortable` is set on the struct. Specifies that the field should not be used for sorting. Clicking it's header will not do anything.
//! - **`title`** - Specifies the title that is displayed in the header cell. Defaults to the field name converted to title case (`this_field` becomes `"This Field"`).
//! - **`renderer`** - Specifies the name of the cell renderer component. Used to customize the rendering of cells.
//!    Defaults to [`DefaultNumberTableCellRenderer`] for number types and [`DefaultTableCellRenderer`] for anything else.
//!    As long as Leptos supports rendering the type it will work.
//!    If the feature `chrono` is enabled then [`DefaultNaiveDateTableCellRenderer`], [`DefaultNaiveDateTimeTableCellRenderer`] and
//!    [`DefaultNaiveTimeTableCellRenderer`] are used for [`chrono::NaiveDate`], [`chrono::NaiveDateTime`] and [`chrono::NaiveTime`] respectively.
//!  - **`format`** - Quick way to customize the formatting of cells without having to create a custom renderer. See [Formatting](#formatting) below for more information.
//!
//! ### Formatting
//!
//! The `format` attribute can be used to customize the formatting of cells. It is an easier alternative to creating a custom renderer when you just want to customize some basic formatting.
//!
//! - **`precision`** - Specifies the number of digits to display after the decimal point. Only works for numbers.
//! - **`string`** - Specifies a format string. Currently only used for `NaiveDate`, `NaiveDateTime` and `NaiveTime`. See [`chrono::format::strftime`] for more information.

#![cfg_attr(
    feature = "chrono",
    doc = r##"
Example:

```
# use leptos::*;
# use leptos_struct_table::*;
# use serde::{Deserialize, Serialize};
# use async_trait::async_trait;
# use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TemperatureMeasurement {
    #[table(key)]
    id: u32,
    #[table(title = "Temperature (°C)", format(precision = 2))]
    temperature: f32,
    #[table(format(string = "%m.%d.%Y"))]
    date: NaiveDate,
}
```
"##
)]

//! # Classes Customization
//!
//! Classes can be easily customized by using the `classes_provider` attribute on the struct with.
//! Specify a type that implementats the trait [`TableClassesProvider`]. Please see documentation for that trait for more information.
//! You can also look at [`TailwindClassesPreset`] for an example how this can be implemented.
//!
//! Example:
//!
//! ```
//! # use leptos::*;
//! # use leptos_struct_table::*;
//! # use serde::{Deserialize, Serialize};
//! # use async_trait::async_trait;
//! #[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
//! #[table(classes_provider = "TailwindClassesPreset")]
//! pub struct Book {
//!     #[table(key)]
//!     id: u32,
//!     title: String,
//! }
//! ```
//!
//! # Custom Renderers
//!
//! Custom renderers can be used to customize almost every aspect of the table.
//! They are specified by using the different `...renderer` attribute on the struct or a field.
//! To implement a custom renderer please have a look at the default renderers listed below.
//!
//! On the struct level you can use these attributes:
//! - `row_renderer` - Defaults to [`DefaultTableRowRenderer`].
//! - `head_cell_renderer` - Defaults to [`DefaultTableHeaderRenderer`].
//!
//! On the field level you can use the `renderer` attribute.
//!
//! It defaults to [`DefaultNumberTableCellRenderer`] for number types and [`DefaultTableCellRenderer`] for anything else.
//! As long as Leptos supports rendering the type it will work.
//! If the feature `chrono` is enabled then [`DefaultNaiveDateTableCellRenderer`], [`DefaultNaiveDateTimeTableCellRenderer`] and
//! [`DefaultNaiveTimeTableCellRenderer`] are used for [`chrono::NaiveDate`], [`chrono::NaiveDateTime`] and [`chrono::NaiveTime`] respectively.
//!
//! Example:
//!
//! ```
//! # use leptos::*;
//! # use leptos_struct_table::*;
//! # use serde::{Deserialize, Serialize};
//! # use async_trait::async_trait;
//! #[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
//! pub struct Book {
//!     #[table(key)]
//!     id: u32,
//!     title: String,
//!     #[table(renderer = "ImageTableCellRenderer")]
//!     img: String,
//! }
//!
//! // Easy cell renderer that just displays an image from an URL.
//! #[component]
//! fn ImageTableCellRenderer(
//!     cx: Scope,
//!     #[prop(into)] class: MaybeSignal<String>,
//!     #[prop(into)] value: MaybeSignal<T>,
//! ) -> impl IntoView
//! where
//!     T: IntoView + Clone + 'static,
//! {
//!     view! { cx,
//!         <td class=class>
//!             <img src=value alt="Book image" height="64"/>
//!         </td>
//!     }
//! }
//! ```
//!
//! For more detailed information please have a look at the `custom_renderers_svg` example for a complete customization.
//!
//! # Contribution
//!
//! All contributions are welcome. Please open an issue or a pull request if you have any ideas or problems.

#![allow(non_snake_case)]

mod class_providers;
mod components;
mod data_provider;

pub use class_providers::*;
pub use components::*;
pub use data_provider::*;
pub use leptos_struct_table_macro::TableComponent;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColumnSort {
    Ascending,
    Descending,
    None,
}

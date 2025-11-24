//! Easily create Leptos table components from structs.
//!
//! ![Hero Image](https://raw.githubusercontent.com/synphonyte/leptos-struct-table/master/hero.webp)
//!
//! # Features
//!
//! - **Easy to use** - yet powerful.
//! - **Async data loading** - The data is loaded asynchronously. This allows to load data from a REST API or a database etc.
//! - **Selection** - Can be turned off or single/multi select
//! - **Customization** - You can customize every aspect of the table by plugging in your own components for rendering rows, cells, headers. See [Custom Renderers](#custom-renderers) for more information.
//! - **Headless** - No default styling is applied to the table. You can fully customize the classes that are applied to the table. See [Classes customization](#classes-customization) for more information.
//! - **Sorting** - Optional. If turned on: Click on a column header to sort the table by that column. You can even sort by multiple columns.
//! - **Virtualization** - Only the visible rows are rendered. This allows for very large tables.
//! - **Pagination** - Instead of virtualization you can paginate the table.
//! - **Caching** - Only visible rows are loaded and cached.
//! - **Editing** - Optional. You can provide custom renderers for editable cells. See [Editable Cells](#editable-cells) for more information.
//!
//! # Usage
//!
//! ```
//! use leptos::prelude::*;
//! use leptos_struct_table::*;
//!
//! #[derive(TableRow, Clone)]
//! #[table(impl_vec_data_provider)]
//! pub struct Person {
//!     id: u32,
//!     name: String,
//!     age: u32,
//! }
//!
//! #[component]
//! fn Demo() -> impl IntoView {
//!     let rows = vec![
//!         Person { id: 1, name: "John".to_string(), age: 32 },
//!         Person { id: 2, name: "Jane".to_string(), age: 28 },
//!         Person { id: 3, name: "Bob".to_string(), age: 45 },
//!     ];
//!
//!     view! {
//!         <table>
//!             <TableContent rows scroll_container="html" />
//!         </table>
//!     }
//! }
//! ```
//!
//! # Leptos Compatibility
//!
//! | Crate version | Compatible Leptos version |
//! |---------------|---------------------------|
//! | <= 0.2        | 0.3                       |
//! | 0.3           | 0.4                       |
//! | 0.4, 0.5, 0.6 | 0.5                       |
//! | 0.7 – 0.12    | 0.6                       |
//! | 0.14.0-beta   | 0.7                       |
//! | 0.15          | 0.8                       |
//!
//! # Server-Side Rendering
//!
//! To use this with Leptos' server-side rendering, you can have to add `leptos-use` as a dependency to your `Cargo.toml` and
//! then configure it for SSR like the following.
//!
//! ```toml
//! [dependencies]
//! leptos-use = "<current version>"
//! # ...
//!
//! [features]
//! hydrate = [
//!     "leptos/hydrate",
//!     # ...
//! ]
//! ssr = [
//!     "leptos/ssr",
//!     # ...
//!     "leptos-use/ssr",
//! ]
//! ```
//!
//! Please see the [serverfn_sqlx example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/serverfn_sqlx/Cargo.toml)
//! for a working project with SSR.
//!
//! # Data Providers
//!
//! As shown in the initial usage example, when you add `#[table(impl_vec_data_provider)]` to your struct,
//! the table will automatically generate a data provider for you. You can then directly pass a `Vec<T>` to the `rows` prop.
//! Internally this implements the trait [`TableDataProvider`] for `Vec<T>`.
//!
//! To leverage the full power of async partial data loading with caching you should implement the trait
//! [`PaginatedTableDataProvider`] or the trait [`TableDataProvider`] yourself. It's quite easy to do so.
//! Which of the two traits you choose depends on your data source. If your data source provides
//! paginated data, as is the case for many REST APIs, you should implement [`PaginatedTableDataProvider`].
//! Otherwise you should probably implement [`TableDataProvider`].
//!
//! See the [paginated_rest_datasource example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/paginated_rest_datasource/src/data_provider.rs)
//! and the [serverfn_sqlx example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/serverfn_sqlx/src/data_provider.rs)
//! for working demo projects that implement these traits.
//!
//! # Macro options
//!
//! The `#[table(...)]` attribute can be used to customize the generated component. The following options are available:
//!
//! ## Struct attributes
//!
//! These attributes can be applied to the struct itself.
//!
//! - **`sortable`** - Specifies that the table should be sortable. This makes the header titles clickable to control sorting.
//!   You can specify two sorting modes with the prop `sorting_mode` on the `TableContent` component:
//!   - `sorting_mode=SortingMode::MultiColumn` (the default) allows the table to be sorted by multiple columns ordered by priority.
//!   - `sorting_mode=SortingMode::SingleColumn"` allows the table to be sorted by a single column. Clicking on another column will simply replace the sorting column.
//!
//!   See the [simple example](https://github.com/synphonyte/leptos-struct-table/blob/master/examples/simple/src/main.rs) and the
//!   [selectable example](https://github.com/synphonyte/leptos-struct-table/blob/master/examples/selectable/src/main.rs) for more information.
//! - **`classes_provider`** - Specifies the name of the class provider. Used to quickly customize all of the classes that are applied to the table.
//!   For convenience sensible presets for major CSS frameworks are provided. See [`TableClassesProvider`] and [tailwind example](https://github.com/synphonyte/leptos-struct-table/blob/master/examples/tailwind/src/main.rs) for more information.
//! - **`head_cell_renderer`** - Specifies the name of the header cell renderer component. Used to customize the rendering of header cells. Defaults to [`DefaultTableHeaderRenderer`]. See the [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs) for more information.
//! - **`impl_vec_data_provider`** - If given, then [`TableDataProvider`] is automatically implemented for `Vec<ThisStruct>` to allow
//!   for easy local data use. See the [simple example](https://github.com/synphonyte/leptos-struct-table/blob/master/examples/simple/src/main.rs) for more information.
//! - **`row_type`** - Specifies the type of the rows in the table. Defaults to the struct that this is applied to. See the [custom_type example](https://github.com/synphonyte/leptos-struct-table/blob/master/examples/custom_type/src/main.rs) for more information.
//! - **`i18n`** - Allows to specify the i18n scope for all fields of the struct as well as the `i18n` module path which defaults to `crate::i18n`. See [I18n](#i18n) for more information.
//!
//! ## Field attributes
//!
//! These attributes can be applied to any field in the struct.
//!
//! - **`class`** - Specifies the classes that are applied to each cell (head and body) in the field's column. Can be used in conjunction with `classes_provider` to customize the classes.
//! - **`head_class`** - Specifies the classes that are applied to the header cell in the field's column. Can be used in conjunction with `classes_provider` to customize the classes.
//! - **`cell_class`** - Specifies the classes that are applied to the body cells in the field's column. Can be used in conjunction with `classes_provider` to customize the classes.
//! - **`skip`** - Specifies that the field should be skipped. This is useful for fields that are not displayed in the table.
//! - **`skip_sort`** - Only applies if `sortable` is set on the struct. Specifies that the field should not be used for sorting. Clicking it's header will not do anything.
//! - **`skip_header`** - Makes the title of the field not be displayed in the head row.
//! - **`title`** - Specifies the title that is displayed in the header cell. Defaults to the field name converted to title case (`this_field` becomes `"This Field"`).
//! - **`renderer`** - Specifies the name of the cell renderer component. Used to customize the rendering of cells.
//!   Defaults to [`DefaultTableCellRenderer`].
//!  - **`format`** - Quick way to customize the formatting of cells without having to create a custom renderer. See [Formatting](#formatting) below for more information.
//! - **`getter`** - Specifies a method that returns the value of the field instead of accessing the field directly when rendering.
//! - **`none_value`** - Specifies a display value for `Option` types when they are `None`. Defaults to empty string
//! - **`i18n`** - Overrides the i18n key for the field. See [I18n](#i18n) for more information.
//!
//! ### Formatting
//!
//! The `format` attribute can be used to customize the formatting of cells. It is an easier alternative to creating a custom renderer when you just want to customize some basic formatting.
//! It is type safe and tied to the type the formatting is applied on. see [`CellValue`] and the associated type for the type you are rendering to see a list of options
//!
//! See:
//! - [`cell_value::NumberRenderOptions`]
#![cfg_attr(feature = "chrono", doc = r##"- [`chrono::RenderChronoOptions`]"##)]
#![cfg_attr(
    feature = "rust_decimal",
    doc = r##"- [`rust_decimal::DecimalNumberRenderOptions`]"##
)]
//!
//!
#![cfg_attr(
    feature = "chrono",
    doc = r##"
Example:

```
# use leptos::prelude::*;
# use leptos_struct_table::*;
# use ::chrono::{NaiveDate, NaiveDateTime, NaiveTime};
#
#[derive(TableRow, Clone)]
pub struct TemperatureMeasurement {
    #[table(title = "Temperature (°C)", format(precision = 2usize))]
    temperature: f32,
    #[table(format(string = "%m.%d.%Y"))]
    date: NaiveDate,
}
```
"##
)]

//! # Features
//!
//! - **`chrono`** - Adds support for types from the crate `chrono`.
//! - **`rust_decimal`** - Adds support for types from the crate `rust_decimal`.
//! - **`time`** - Adds support for types from the crate `time`.
//! - **`uuid`** - Adds support for types from the crate `uuid`.
//!
//! # Classes Customization
//!
//! Classes can be easily customized by using the `classes_provider` attribute on the struct.
//! You can specify any type that implements the trait [`TableClassesProvider`]. Please see the documentation for that trait for more information.
//! You can also look at [`TailwindClassesPreset`] for an example how this can be implemented.
//!
//! Example:
//!
//! ```
//! # use leptos::prelude::*;
//! # use leptos_struct_table::*;
//! #
//! #[derive(TableRow, Clone)]
//! #[table(classes_provider = "TailwindClassesPreset")]
//! pub struct Book {
//!     id: u32,
//!     title: String,
//! }
//! ```
//!
//! # Field Getters
//!
//! Sometimes you want to display a field that is not part of the struct but a derived value either
//! from other fields or sth entirely different. For this you can use either the [`FieldGetter`] type
//! or the `getter` attribute.
//!
//! Let's start with [`FieldGetter`] and see an example:
//!
//! ```
//! # use leptos::prelude::*;
//! # use leptos_struct_table::*;
//! # use serde::{Deserialize, Serialize};
//! #
//! #[derive(TableRow, Clone)]
//! #[table(classes_provider = "TailwindClassesPreset")]
//! pub struct Book {
//!     id: u32,
//!     title: String,
//!     author: String,
//!
//!     // this tells the macro that you're going to provide a method called `title_and_author` that returns a `String`
//!     title_and_author: FieldGetter<String>
//! }
//!
//! impl Book {
//!     // Returns the value that is displayed in the column
//!     pub fn title_and_author(&self) -> String {
//!         format!("{} by {}", self.title, self.author)
//!     }
//! }
//! ```
//!
//! To provide maximum flexibility you can use the `getter` attribute.
//!
//! ```
//! # use leptos::prelude::*;
//! # use leptos_struct_table::*;
//! #
//! #[derive(TableRow, Clone)]
//! #[table(classes_provider = "TailwindClassesPreset")]
//! pub struct Book {
//!     // this tells the macro that you're going to provide a method called `get_title` that returns a `String`
//!     #[table(getter = "get_title")]
//!     title: String,
//! }
//!
//! impl Book {
//!     pub fn get_title(&self) -> String {
//!         format!("Title: {}", self.title)
//!     }
//! }
//! ```
//!
//! ## When to use `FieldGetter` vs `getter` attribute
//!
//! A field of type `FieldGetter<T>` is a virtual field that doesn't really exist on the struct.
//! Internally `FieldGetter` is just a new-typed `PhantomData` and thus is removed during compilation.
//! Hence it doesn't increase memory usage. That means you should use it for purely derived data.
//!
//! The `getter` attribute should be used on a field that actually exists on the struct but whose
//! value you want to modify before it's rendered.
//!
//! # Custom Renderers
//!
//! Custom renderers can be used to customize almost every aspect of the table.
//! They are specified by using the various `...renderer` attributes on the struct or fields or props of the [`TableContent`] component.
//! To implement a custom renderer please have a look at the default renderers listed below.
//!
//! On the struct level you can use this attribute:
//! - **`thead_cell_renderer`** - Defaults to [`DefaultTableHeaderCellRenderer`] which renders `<th><span>Title</span></th>`
//!   together with sorting functionality (if enabled).
//!
//! As props of the [`TableContent`] component you can use the following:
//! - **`thead_renderer`** - Defaults to [`DefaultTableHeadRenderer`] which just renders the tag `thead`.
//! - **`thead_row_renderer`** - Defaults to [`DefaultTableHeadRowRenderer`] which just renders the tag `tr`.
//! - **`tbody_renderer`** - Defaults to the tag `tbody`. Takes no attributes.
//! - **`row_renderer`** - Defaults to [`DefaultTableRowRenderer`].
//! - **`loading_row_renderer`** - Defaults to [`DefaultLoadingRowRenderer`].
//! - **`error_row_renderer`** - Defaults to [`DefaultErrorRowRenderer`].
//! - **`row_placeholder_renderer`** - Defaults to [`DefaultRowPlaceholderRenderer`].
//!
//! On the field level you can use the **`renderer`** attribute.
//!
//! It defaults to [`DefaultTableCellRenderer`]
//! Works for any type that implements the [`CellValue`] trait that is implemented for types in the standard library, popular crates with feature flags and for your own type if you implement this trait for them.
//!
//! Example:
//!
//! ```
//! # use leptos::prelude::*;
//! # use leptos_struct_table::*;
//! #
//! #[derive(TableRow)]
//! pub struct Book {
//!     title: String,
//!     #[table(renderer = "ImageTableCellRenderer")]
//!     img: String,
//! }
//!
//! // Easy cell renderer that just displays an image from an URL.
//! #[component]
//! fn ImageTableCellRenderer(
//!     class: String,
//!     value: Signal<String>,
//!     row: RwSignal<Book>,
//!     index: usize,
//! ) -> impl IntoView
//! {
//!     view! {
//!         <td class=class>
//!             <img src=value alt="Book image" height="64"/>
//!         </td>
//!     }
//! }
//! ```
//!
//! For more detailed information please have a look at the [custom_renderers_svg example](https://github.com/synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs) for a complete customization.
//!
//!
//! ## Editable Cells
//!
//! You might have noticed the prop `row` in the custom cell renderer above. This can be used
//! to edit the data. Simply use the `RwSignal` to access the row and change the fields.
//!
//! ```
//! # use leptos::{prelude::*, logging};
//! # use leptos_struct_table::*;
//! #
//! #[derive(TableRow, Clone, Default, Debug)]
//! #[table(impl_vec_data_provider)]
//! pub struct Book {
//!     id: u32,
//!     #[table(renderer = "InputCellRenderer")]
//!     title: String,
//! }
//!
//! #[component]
//! fn InputCellRenderer(
//!     class: String,
//!     value: Signal<String>,
//!     row: RwSignal<Book>,
//!     index: usize,
//! ) -> impl IntoView {
//!     let on_change = move |evt| {
//!         row.write().title = event_target_value(&evt);
//!     };
//!
//!     view! {
//!         <td class=class>
//!             <input type="text" value=value on:change=on_change />
//!         </td>
//!     }
//! }
//!
//! // Then in the table component you can listen to the `on_change` event:
//!
//! #[component]
//! pub fn App() -> impl IntoView {
//!     let rows = vec![Book::default(), Book::default()];
//!
//!     let on_change = move |evt: ChangeEvent<Book>| {
//!         logging::log!("Changed row at index {}:\n{:#?}", evt.row_index, evt.changed_row.get_untracked());
//!     };
//!
//!     view! {
//!         <table>
//!             <TableContent rows on_change scroll_container="html" />
//!         </table>
//!     }
//! }
//! ```
//!
//! Please have a look at the [editable example](https://github.com/Synphonyte/leptos-struct-table/tree/master/examples/editable/src/main.rs) for a fully working example.
//!
//! # Pagination / Virtualization / InfiniteScroll
//!
//! This table component supports different display acceleration strategies. You can set them through the `display_strategy` prop of
//! the [`TableContent`] component.
//!
//! The following options are available. Check their docs for more details.
//! - [`DisplayStrategy::Virtualization`] (default)
//! - [`DisplayStrategy::InfiniteScroll`]
//! - [`DisplayStrategy::Pagination`]
//!
//! Please have a look at the [pagination example](https://github.com/Synphonyte/leptos-struct-table/tree/master/examples/pagination/src/main.rs) for more information on how to use pagination.
//!
//! # I18n
//!
//! To translate the column titles of the table using `leptos-i18n` you can enable the `"i18n"`
//! feature. The field names of the struct are used as keys by default and can be customized using the `i18n` attribute.
//!
//! Please have a look at the
//! [i18n example](https://github.com/Synphonyte/leptos-struct-table/tree/master/examples/i18n)
//! and at the sections [Struct attributes](#struct-attributes) and
//! [Field attributes](#field-attributes) for more information.
//!
//! # Contribution
//!
//! All contributions are welcome. Please open an issue or a pull request if you have any ideas or problems.

#![allow(non_snake_case)]

mod cell_value;
#[cfg(feature = "chrono")]
pub mod chrono;
mod class_providers;
mod components;
mod data_provider;
mod display_strategy;
mod events;
mod loaded_rows;
mod reload_controller;
mod refresh_controller;
mod row_reader;
#[cfg(feature = "rust_decimal")]
pub mod rust_decimal;
mod selection;
mod sorting;
mod table_row;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "uuid")]
mod uuid;

pub use cell_value::*;
pub use class_providers::*;
pub use components::*;
pub use data_provider::*;
pub use display_strategy::*;
pub use events::*;
pub use leptos_struct_table_macro::TableRow;
pub use loaded_rows::RowState;
pub use reload_controller::*;
pub use refresh_controller::*;
pub use row_reader::*;
pub use selection::*;
pub use sorting::*;
pub use table_row::*;

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Type of sorting of a column
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ColumnSort {
    Ascending,
    Descending,
    None,
}

impl ColumnSort {
    /// Returns the a default class name
    pub fn as_class(&self) -> &'static str {
        match self {
            ColumnSort::Ascending => "sort-asc",
            ColumnSort::Descending => "sort-desc",
            _ => "",
        }
    }

    /// Returns the SQL sort order (ASC or DESC) or `None` if `ColumnSort::None`.
    pub fn as_sql(&self) -> Option<&'static str> {
        match self {
            ColumnSort::Ascending => Some("ASC"),
            ColumnSort::Descending => Some("DESC"),
            _ => None,
        }
    }
}

/// Type of struct field used to specify that the value of this field is
/// obtained by calling a getter method on the struct.
///
/// Please refer to the [`getter` example](https://github.com/Synphonyte/leptos-struct-table/tree/master/examples/getter) for how this is used
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub struct FieldGetter<T>(PhantomData<T>);

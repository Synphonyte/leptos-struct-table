# Changelog

## [0.13.0] - 2024-09-05

### Breaking Change 🛠️

- Updated dependency leptos-use to version 0.13 which fixes some unsatisfied trait bounds.

## [0.12.1] - 2024-09-01

### Features 🚀

- Added support for generics in struct field types (thanks to @frnsys)
- Added macro options for much more flexibility with leptos-i18n (thanks to @Baptistemontan)
- Added hint to readmes of examples how to run them (thanks to @luckynumberke7in)

## [0.12.0] - 2024-08-14

### Breaking Change 🛠️

- Updated dependency leptos-use to version 0.12 which supports web-sys 0.3.70 which introduced breaking changes. (thanks
  to @frnsys)

## [0.11.0] - 2024-08-05

### Features 🚀

- Changed leptos-use to version 0.11
- Added i18n support via the `"i18n"` feature which uses `leptos-i18n`. See the `i18n` example for usage.
- Added row reader to `TableComponent`
- Added `default_th_sorting_style` to make it easier to write a custom thead cell render component.

## [0.10.2] - 2024-06-07

### Fixes 🐛

- Fixed race condition with loading row count and sorting update.
- Fixed console errors/warnings for signals accessed in async blocks after component was disposed of.

## [0.10.1] - 2024-06-05

### Change 🔥

- `CellValue` is now implemented for `leptos::View`. This makes `FieldGetter<View` possible out of the box.

### Fix 🐛

- Fixed mutating the sorting signal programmatically didn't trigger loading or rerendering (thanks @dakaizou).

## [0.10.0] - 2024-05-26

### Breaking Changes 🛠️

- There is no longer a new-typed `Uuid`. You can now use the `Uuid` type from the crate `uuid` directly (thanks to
  @lukashermansson).
- The `DefaultCellRenderer` no longer requires values that implement Leptos' `IntoView` but our own trait `CellValue`
  which is basically the same but gives us much more flexibility (thanks to @lukashermansson).
- There are no more chrono default cell renderers or `DefaultNumberTableCellRenderer` anymore. This can now all be
  handled by the `DefaultCellRenderer` thanks to the new `CellValue` trait (thanks to @lukashermansson).

### Features 🚀

- The new `CellValue` trait allows us to have less macro magic and allow you to specify your own format arguments
  that can be used in the macro attribute `#[table(format(...))]` (thanks to @lukashermansson).
- There is now the new feature `time` to add support for the equally named crate as cell values
  (thanks to @lukashermansson).
- This crate is now ready to be used with stable Rust (thanks to @tyoeer).
- You can now specify the prop `sorting_mode` on the component `TableContent` to specify multi-column (the default)
  or single-column sorting.

### Examples 🧪

- The `serverfn_sqlx` example now shows how to implement sorting with sqlx (thanks to @lukashermansson).

### Fixes 🐛

- Fixed pagination with data that is too short to fill the first page (thanks to @TimTom2016).
- Removed serde dependency from feature flag `chrono` (thanks to @lukashermansson).

## [0.9.1] - 2024-02-28

### Fixes 🐛

- Fixed row height detection for virtualization
- Row count now reloads when the data source triggers changes

## [0.9.0] - 2024-02-22

### Breaking Changes 🛠️

- Added methods `TableRow::col_name`, `ColumnSort::as_sql` and `TableRow::sorting_to_sql` to make it easy to implement
  db sorting
- Removed dependency `async-trait`. The traits `TableDataProvider` and `PaginatedTableDataProvider` now use the native
  async method support.

### Fix 🐛

- The default placeholder renderer now uses `<tr>...</tr>` to produce valid HTML. This fixes SSR rendering issues.

### Other Changes

- Added an example for how to use server functions and sqlx together with this crate.

## [0.8.3] - 2024-02-20

### Fix 🐛

- When not limiting a scroll container this could lead to a runaway row loading. This is now limited to max 500 rows.

## [0.8.2] - 2024-02-18

### Feature 🚀

- Added method `TableDataProvider::track` to easily specify reactive dependencies of data loading

## [0.8.1] - 2024-02-17

### Fix 🐛

- Removed debug log

## [0.8.0] - 2024-02-17

### Feature 🚀

- Added `loading_row_display_limit` prop to `TableContent` to make it possible to load smaller row counts nicely

### Breaking Changes 🛠️

- Added `row_index` and `col_index` to `TableClassesProvider::loading_cell`
- Added `col_index` to `TableClassesProvider::loading_cell_inner`
- Changed the type of prop `loading_row_renderer` of the component `TableContent`

### Fix 🐛

- Data loading for small data sets

## [0.7.1] - 2024-02-14

### Changes

- Added generic error type to `TableDataProvider`
- Fixed sorting for tables with skipped fields

## [0.7.0] - 2024-02-08

### Features 🚀

- Virtualization — Only elements that are visible are rendered (with some extra for smooth scrolling).
    - Other display acceleration strategies like infinite scroll and pagination are implemented as well.
- Caching — Only rows that are visible are requested from the data source and then cached.
- Error handling — If an error occurs while loading data, it is displayed in a table row instead of the failed data.
- Easy reloading — The data can be reloaded through the `ReloadController`.

### Breaking Changes 🛠️

Everything? - sorry. This release is like half a rewrite with much less macro magic.
Please check the docs and examples.

## [0.6.0] - 2023-11-02

### New Feature 🎉

- Support for generic structs

### Fix 🐛

- Fixed `#[table(skip_sort)]` on fields

## [0.5.0] - 2023-10-20

### Breaking Changes 🛠️

- Added `on_change` events to support editable data (see new editable example)

### Fixes 🐛

- Fixed selection with `key`s that are not `Copy`

### Other Changes

- Modified REST example to include sorting

## [0.4.0] - 2023-10-02

- Updated to leptos 0.5

## [0.3.0]

- Updated to leptos 0.4

## [0.2.0]

- Updated to leptos 0.3
- Deactivated `default-features` of leptos
- New class provider `BootstrapClassesPreset`
- New example `bootstrap`
- Added `thead` and `tbody` with customizable renderers
- Added `getter` and `FieldGetter<T>` with new example
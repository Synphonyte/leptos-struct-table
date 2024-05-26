# Changelog

## [0.10.0] - 2024-05-26

### Features 🚀

- You can now specify the prop `sorting_mode` on the component `TableContent` to specify multi-column (the default)
  or single-column sorting.

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
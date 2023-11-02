# Changelog

## [Unreleased]

### Features 🚀

- Virtualization — Only elements that are visible are rendered (with some extra for smooth scrolling).
- Caching — Only rows that are visible are requested from the data source and then cached.
- Error handling — If an error occurs while loading data, it is displayed in a table row instead of the failed data.
- Easy reloading — The data can be reloaded through calling `TableData::reload`.

### Breaking Changes 🛠️

- The table component now takes a `TableData` instead of `RwSignal<impl TableDataProvider>`.
  Anything that implements `TableDataProvider` can be passed to the table component and is automatically converted.
- The trait `TableDataProvider` has been extended to accomodate all the new functionaliy. 
  Please check it's documentation for more details.

## [0.6.0] - 2023-11-02

### New Feature 🎉

- Support for generic structs

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
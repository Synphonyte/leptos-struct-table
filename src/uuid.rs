//! Support for [uuid::Uuid] type.
#![doc(cfg(feature = "uuid"))]
use leptos::*;
use ::uuid::Uuid;
use crate::*;

/// Implementation for [`Uuid`] to work with the [`TableRow`] derive and the [`DefaultTableCellRenderer`]
/// ``` 
/// # use leptos_struct_table::*;
/// # use leptos::*;
/// # use uuid::Uuid;
/// #[derive(TableRow, Clone)]
/// #[table]
/// struct SomeStruct {
///     my_field: Uuid
/// }
/// ```
impl CellValue for Uuid {
    type RenderOptions = ();
    fn render_value(self, _options: &Self::RenderOptions) -> impl IntoView {
        self.to_string()
    }
}
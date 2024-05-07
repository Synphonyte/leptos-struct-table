//! Support for [uuid::Uuid] type.

use leptos::*;

/// CellValue implementation for uuid for uuid to work with the TableRow derive
/// ``` 
/// # use leptos_struct_table::*;
/// # use leptos::*;
/// # use uuid::Uuid;
/// 
/// #[derive(TableRow, Clone)]
/// #[table]
/// struct SomeStruct {
///     my_field: ::uuid::Uuid
/// }
/// ```
impl crate::cell_value::CellValue for uuid::Uuid {
    type RenderOptions = ();
    fn render_value(self, _options: &Self::RenderOptions) -> impl IntoView {
        self.to_string()
    }
}
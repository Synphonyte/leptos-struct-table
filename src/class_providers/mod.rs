mod bootstrap;
mod tailwind;

use crate::ColumnSort;
pub use bootstrap::*;
pub use tailwind::*;

/// A trait for providing classes for the table.
pub trait TableClassesProvider: Clone {
    /// Create a new instance of the class provider.
    fn new() -> Self;

    /// Get the classes for the root table element.
    /// The `classes` parameter contains the classes specified in the `classes` prop of the generated component.
    fn table(&self, classes: &str) -> String {
        classes.to_string()
    }

    /// Get the classes for the head row.
    /// The `template_classes` parameter contains the classes specified in the `head_row_class` attribute of the struct.
    fn head_row(&self, template_classes: &str) -> String {
        template_classes.to_string()
    }

    #[allow(unused_variables)]
    /// Get the classes for the head cells.
    /// The `sort` parameter contains the sort state of the column.
    /// The `template_classes` parameter contains the classes specified in the `head_class` attribute of the field.
    fn head_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        template_classes.to_string()
    }

    /// Get the classes for the head cells' inner element.
    fn head_cell_inner(&self) -> String {
        "".to_string()
    }

    #[allow(unused_variables)]
    /// Get the classes for the body rows.
    /// The `row_index` parameter contains the index of the row. The first row has index 0.
    /// The `selected` parameter indicates whether the row is selected.
    /// The `template_classes` parameter contains the classes specified in the `row_class` attribute of the struct.
    fn row(&self, row_index: usize, selected: bool, template_classes: &str) -> String {
        template_classes.to_string() + if selected { " selected" } else { "" }
    }

    /// Get the classes for the body cells.
    /// The `template_classes` parameter contains the classes specified in the `class` attribute of the field.
    fn cell(&self, template_classes: &str) -> String {
        template_classes.to_string()
    }
}

mod bootstrap;
mod tailwind;

use crate::ColumnSort;
pub use bootstrap::*;
pub use tailwind::*;

/// A trait for providing classes for the table.
pub trait TableClassesProvider {
    /// Create a new instance of the class provider.
    fn new() -> Self;

    /// Get the class attribute for the thead.
    /// The `prop_class` parameter contains the classes specified in the
    /// `thead_class` prop of the [`TableContent`] component.
    fn thead(&self, prop_class: &str) -> String {
        prop_class.to_string()
    }

    /// Get the classes for the thead row.
    /// The `prop_class` parameter contains the classes specified in the
    /// `thead_row_class` prop of the [`TableContent`] component.
    fn thead_row(&self, prop_class: &str) -> String {
        prop_class.to_string()
    }

    /// Get the classes for the thead cells.
    /// The `sort` parameter contains the sort state of the column.
    /// The `macro_class` parameter contains the classes specified in the `head_class` macro attribute of the field.
    fn thead_cell(&self, sort: ColumnSort, macro_class: &str) -> String {
        format!("{} {}", sort.as_class(), macro_class)
    }

    /// Get the classes for the thead cells' inner element.
    fn thead_cell_inner(&self) -> String {
        "".to_string()
    }

    /// Get the classes for the tbody.
    /// The `prop_class` parameter contains the classes specified in the
    /// `tbody_class` prop of the [`TableContent`] component.
    fn tbody(&self, prop_class: &str) -> String {
        prop_class.to_string()
    }

    #[allow(unused_variables)]
    /// Get the classes for the body rows.
    /// The `row_index` parameter contains the index of the row. The first row has index 0.
    /// The `selected` parameter indicates whether the row is selected.
    /// The `prop_class` parameter contains the classes specified in the `row_class`
    /// prop of the [`TableContent`] component.
    fn row(&self, row_index: usize, selected: bool, prop_class: &str) -> String {
        prop_class.to_string() + if selected { " selected" } else { "" }
    }

    #[allow(unused_variables)]
    /// Get the classes for the elements inside of the cells of rows that are currently
    /// being loaded.
    /// The `prop_class` parameter contains the classes specified in the
    /// `loading_cell_class` prop of the [`TableContent`] component.
    fn loading_cell(&self, row_index: usize, col_index: usize, prop_class: &str) -> String {
        prop_class.to_string()
    }

    #[allow(unused_variables)]
    /// Get the classes for the elements inside of the cells of rows that are currently
    /// being loaded. Usually this will be some loading indicator like a sceleton bar.
    /// The `prop_class` parameter contains the classes specified in the
    /// `loading_cell_inner_class` prop of the [`TableContent`] component.
    fn loading_cell_inner(&self, row_index: usize, col_index: usize, prop_class: &str) -> String {
        prop_class.to_string()
    }

    /// Get the classes for the body cells.
    /// The `macro_class` parameter contains the classes specified in the `class` macro attribute of the field.
    fn cell(&self, macro_class: &str) -> String {
        macro_class.to_string()
    }
}

#[derive(Copy, Clone)]
pub struct DummyTableClassesProvider;

impl TableClassesProvider for DummyTableClassesProvider {
    fn new() -> Self {
        Self
    }
}

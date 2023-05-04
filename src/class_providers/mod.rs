mod tailwind;

use crate::ColumnSort;
pub use tailwind::*;

pub trait TableClassesProvider: Clone {
    fn new() -> Self;

    fn table(&self, classes: &str) -> String {
        classes.to_string()
    }
    fn head_row(&self, template_classes: &str) -> String {
        template_classes.to_string()
    }
    #[allow(unused_variables)]
    fn head_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        template_classes.to_string()
    }
    fn head_cell_inner(&self) -> String {
        "".to_string()
    }
    fn row(&self, _row_index: usize, selected: bool, template_classes: &str) -> String {
        template_classes.to_string() + if selected { " selected" } else { "" }
    }
    fn cell(&self, template_classes: &str) -> String {
        template_classes.to_string()
    }
}

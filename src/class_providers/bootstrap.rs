use crate::{ColumnSort, TableClassesProvider};

#[derive(Clone, Copy)]
pub struct BootstrapClassesPreset;

impl TableClassesProvider for BootstrapClassesPreset {
    fn new() -> Self {
        Self
    }

    fn table(&self, classes: &str) -> String {
        todo!()
    }

    fn head_row(&self, template_classes: &str) -> String {
        todo!()
    }

    fn head_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        todo!()
    }

    fn head_cell_inner(&self) -> String {
        todo!()
    }

    fn row(&self, row_index: usize, selected: bool, template_classes: &str) -> String {
        todo!()
    }

    fn cell(&self, template_classes: &str) -> String {
        todo!()
    }
}

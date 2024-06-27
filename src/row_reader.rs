use crate::loaded_rows::RowState;
use std::cell::RefCell;
use std::rc::Rc;

/// Allows you to read the cached state of rows from inside the table component which handles
/// loading and caching automatically.
pub struct RowReader<Row: Clone> {
    pub(crate) get_loaded_rows: RefCell<Rc<dyn Fn(usize) -> RowState<Row>>>,
}

impl<Row: Clone> Default for RowReader<Row> {
    fn default() -> Self {
        Self {
            get_loaded_rows: RefCell::new(Rc::new(|_| RowState::Placeholder)),
        }
    }
}

impl<Row: Clone> RowReader<Row> {
    /// Returns the cached state of the row at the given index
    pub fn cached_row(&self, index: usize) -> RowState<Row> {
        (*self.get_loaded_rows.borrow())(index)
    }
}

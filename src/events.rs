use leptos::ev::MouseEvent;
use std::rc::Rc;

/// The event provided to the `on_change` prop of the table component
#[derive(Debug, Clone)]
pub struct TableChangeEvent<Row: Clone> {
    /// The index of the table row that contains the cell that was changed. Starts at 0.
    pub row_index: usize,
    /// The index of the table column that contains the cell that was changed. Starts at 0.
    pub col_index: usize,
    /// The the row that was changed. This is the struct for which the table component is generated.
    pub changed_row: Row,
}

/// Event emitted when a table head cell is clicked.
#[derive(Debug)]
pub struct TableHeadEvent {
    /// The index of the column. Starts at 0 for the first column.
    /// The order of the columns is the same as the order of the fields in the struct.
    pub index: usize,
    /// The mouse event that triggered the event.
    pub mouse_event: MouseEvent,
}

/// New type wrapper of a closure that takes a `TableChangeEvent`. This allows the `on_change` prop
/// to be optional while being able to take a simple closure.
#[derive(Clone)]
pub struct ChangeEventHandler<Row: Clone>(Rc<dyn Fn(TableChangeEvent<Row>)>);

impl<Row: Clone> Default for ChangeEventHandler<Row> {
    fn default() -> Self {
        Self(Rc::new(|_| {}))
    }
}

impl<F, Row> From<F> for ChangeEventHandler<Row>
where
    F: Fn(TableChangeEvent<Row>) + 'static,
    Row: Clone,
{
    fn from(f: F) -> Self {
        Self(Rc::new(f))
    }
}

impl<Row: Clone> ChangeEventHandler<Row> {
    pub fn run(&self, event: TableChangeEvent<Row>) {
        (self.0)(event)
    }
}

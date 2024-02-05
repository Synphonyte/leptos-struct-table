use leptos::ev::MouseEvent;
use std::rc::Rc;

/// The event provided to the `on_change` prop of the table component
#[derive(Debug, Clone)]
pub struct ChangeEvent<Row: Clone> {
    /// The index of the table row that contains the cell that was changed. Starts at 0.
    pub row_index: usize,
    /// The index of the table column that contains the cell that was changed. Starts at 0.
    pub col_index: usize,
    /// The the row that was changed.
    pub changed_row: Row,
}

/// The event provided to the `on_selection_change` prop of the table component
#[derive(Debug, Clone)]
pub struct SelectionChangeEvent<Row: Clone> {
    /// `true` is the row was selected, `false` if it was de-selected.
    pub selected: bool,
    /// The index of the row that was de-/selected.
    pub row_index: usize,
    /// The row that was de-/selected.
    pub row: Row,
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
pub struct EventHandler<T>(Rc<dyn Fn(T)>);

impl<T> Default for EventHandler<T> {
    fn default() -> Self {
        Self(Rc::new(|_| {}))
    }
}

impl<F, T> From<F> for EventHandler<T>
where
    F: Fn(T) + 'static,
{
    fn from(f: F) -> Self {
        Self(Rc::new(f))
    }
}

impl<T> EventHandler<T> {
    pub fn run(&self, event: T) {
        (self.0)(event)
    }
}

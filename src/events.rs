use leptos::ev::MouseEvent;
use leptos::prelude::*;
use std::sync::Arc;

/// The event provided to the `on_change` prop of the table component
#[derive(Debug)]
pub struct ChangeEvent<Row: Send + Sync + 'static> {
    /// The index of the table row that contains the cell that was changed. Starts at 0.
    pub row_index: usize,
    /// The the row that was changed.
    pub changed_row: Signal<Row>,
}

impl<Row: Send + Sync + 'static> Clone for ChangeEvent<Row> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Row: Send + Sync + 'static> Copy for ChangeEvent<Row> {}

/// The event provided to the `on_selection_change` prop of the table component
#[derive(Debug)]
pub struct SelectionChangeEvent<Row: Send + Sync + 'static> {
    /// `true` is the row was selected, `false` if it was de-selected.
    pub selected: bool,
    /// The index of the row that was de-/selected.
    pub row_index: usize,
    /// The row that was de-/selected.
    pub row: Signal<Row>,
}

impl<Row: Send + Sync + 'static> Clone for SelectionChangeEvent<Row> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Row: Send + Sync + 'static> Copy for SelectionChangeEvent<Row> {}

/// Event emitted when a table head cell is clicked.
#[derive(Debug)]
pub struct TableHeadEvent<Column> {
    /// The index of the column. Starts at 0 for the first column.
    /// The order of the columns is the same as the order of the fields in the struct.
    pub index: Column,
    /// The mouse event that triggered the event.
    pub mouse_event: MouseEvent,
}

/// New type wrapper of a closure that takes a parameter `T`. This allows the event handler props
/// to be optional while being able to take a simple closure.
#[derive(Clone)]
pub struct EventHandler<T>(Arc<dyn Fn(T) + Send + Sync>);

impl<T> Default for EventHandler<T> {
    fn default() -> Self {
        #[allow(unused_variables)]
        Self(Arc::new(|event: T| {}))
    }
}

impl<F, T> From<F> for EventHandler<T>
where
    F: Fn(T) + Send + Sync + 'static,
{
    fn from(f: F) -> Self {
        Self(Arc::new(f))
    }
}

impl<T> EventHandler<T> {
    #[inline]
    pub fn run(&self, event: T) {
        (self.0)(event)
    }
}

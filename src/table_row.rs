use crate::{ChangeEvent, ColumnSort, EventHandler, TableClassesProvider, TableHeadEvent};
use leptos::*;
use std::collections::VecDeque;

/// This trait has to implemented in order for [`TableContent`] to be able to render rows and the head row of the table.
/// Usually this is done by `#[derive(TableRow, Clone)]`.
///
/// Please see the [simple example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/simple/src/main.rs)
/// for how to use.
pub trait TableRow: Clone {
    type ClassesProvider: TableClassesProvider + Copy;

    /// How many columns this row has (i.e. the number of fields in the struct)
    const COLUMN_COUNT: usize;

    ///
    /// This render function has to render exactly one root element.
    fn render_row(&self, index: usize, on_change: EventHandler<ChangeEvent<Self>>)
        -> impl IntoView;

    fn render_head_row<F>(
        sorting: Signal<VecDeque<(usize, ColumnSort)>>,
        on_head_click: F,
    ) -> impl IntoView
    where
        F: Fn(TableHeadEvent) + Clone + 'static;
}

pub fn get_sorting_for_column(
    col_index: usize,
    sorting: Signal<VecDeque<(usize, ColumnSort)>>,
) -> ColumnSort {
    sorting.with(|sorting| {
        sorting
            .into_iter()
            .find(|(col, _)| *col == col_index)
            .map(|(_, sort)| *sort)
            .unwrap_or(ColumnSort::None)
    })
}

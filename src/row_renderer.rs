use crate::{ColumnSort, TableClassesProvider, TableHeadEvent};
use leptos::*;
use std::collections::VecDeque;

pub trait RowRenderer<Key> {
    type ClassesProvider: TableClassesProvider + Copy;

    fn key(&self) -> Key;

    fn render_row(&self) -> impl IntoView;

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

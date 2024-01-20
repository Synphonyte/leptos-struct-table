use crate::{
    ChangeEventHandler, ColumnSort, TableChangeEvent, TableClassesProvider, TableHeadEvent,
};
use leptos::*;
use std::collections::VecDeque;

pub trait RowRenderer: Clone {
    type ClassesProvider: TableClassesProvider + Copy;

    const COLUMN_COUNT: usize;

    fn key(&self) -> String;

    fn render_row(&self, index: usize, on_change: ChangeEventHandler<Self>) -> impl IntoView;

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

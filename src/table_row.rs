use crate::{ColumnSort, DragManager, TableClassesProvider, TableHeadEvent};
use leptos::prelude::*;
use std::collections::VecDeque;

/// This trait has to be implemented in order for [`TableContent`] to be able to render rows and the head row of the table.
/// Usually this is done by `#[derive(TableRow)]`.
///
/// Please see the [simple example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/simple/src/main.rs)
/// for how to use.
pub trait TableRow<Column: Copy + Send + Sync + 'static>: Sized {
    type ClassesProvider: TableClassesProvider + Copy;

    /// How many columns this row has (i.e. the number of fields in the struct)
    const COLUMN_COUNT: usize;

    /// Renders the inner of one row of the table using the cell renderers.
    /// This produces the children that go into the `row_renderer` given to [`TableContent`].
    ///
    /// This render function has to render exactly one root element.
    fn render_row(
        row: RwSignal<Self>,
        index: usize,
        columns: RwSignal<Vec<Column>>,
    ) -> impl IntoView;

    /// Render the head row of the table.
    fn render_head_row<F>(
        sorting: Signal<VecDeque<(Column, ColumnSort)>>,
        on_head_click: F,
        drag_handlers: DragManager<Column>,
        columns: RwSignal<Vec<Column>>,
    ) -> impl IntoView
    where
        F: Fn(TableHeadEvent<Column>) + Send + Clone + 'static;

    /// All columns this row can show in their default order.
    fn columns() -> &'static [Column];

    /// The name of the column (= struct field name) at the given index. This can be used to implement
    /// sorting in a database. Information on column indexes is available at: the [Column index type](crate#column-index-type) section.
    fn col_name(col_index: Column) -> &'static str;

    /// Converts the given sorting to an SQL statement.
    /// Return `None` when there is nothing to be sorted otherwise `Some("ORDER BY ...")`.
    /// Uses [`Self::col_name`] to get the column names for sorting.
    fn sorting_to_sql(sorting: &VecDeque<(Column, ColumnSort)>) -> Option<String>
    where
        Column: Send + Sync + 'static,
    {
        let mut sort = vec![];

        for (col, col_sort) in sorting {
            if let Some(col_sort) = col_sort.as_sql() {
                sort.push(format!("{} {}", Self::col_name(*col), col_sort))
            }
        }

        if sort.is_empty() {
            return None;
        }

        Some(format!("ORDER BY {}", sort.join(", ")))
    }
}

pub fn get_sorting_for_column<Column>(
    col_index: Column,
    sorting: Signal<VecDeque<(Column, ColumnSort)>>,
) -> ColumnSort
where
    Column: Eq + Send + Sync + 'static,
{
    sorting
        .read()
        .iter()
        .find(|(col, _)| *col == col_index)
        .map(|(_, sort)| *sort)
        .unwrap_or(ColumnSort::None)
}

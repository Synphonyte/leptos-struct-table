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

    /// Renders the inner of one row of the table using the cell renderers.
    /// This produces the children that go into the `row_renderer` given to [`TableContent`].
    ///
    /// This render function has to render exactly one root element.
    fn render_row(&self, index: usize, on_change: EventHandler<ChangeEvent<Self>>)
        -> impl IntoView;

    /// Render the head row of the table.
    fn render_head_row<F>(
        sorting: Signal<VecDeque<(usize, ColumnSort)>>,
        on_head_click: F,
    ) -> impl IntoView
    where
        F: Fn(TableHeadEvent) + Clone + 'static;

    /// The name of the column (= struct field name) at the given index. This can be used to implement
    /// sorting in a database. It takes the `#[table(skip)]` attributes into account. `col_index`
    /// refers to the index of the field in the struct while ignoring skipped ones.
    ///
    /// For example:
    /// ```
    /// # use leptos_struct_table::*;
    /// # use leptos::*;
    /// #
    /// #[derive(TableRow, Clone)]
    /// struct Person {
    ///     #[table(skip)]
    ///     id: i64,            // -> ignored
    ///
    ///     name: String,       // -> col_index = 0
    ///
    ///     #[table(skip)]
    ///     internal: usize,    // -> ignored
    ///
    ///     age: u16,           // -> col_index = 1
    /// }
    ///
    /// assert_eq!(Person::col_name(0), "name");
    /// assert_eq!(Person::col_name(1), "age");
    /// ```
    fn col_name(col_index: usize) -> &'static str;

    /// Converts the given sorting to an SQL statement.
    /// Return `None` when there is nothing to be sorted otherwise `Some("ORDER BY ...")`.
    /// Uses [`Self::col_name`] to get the column names for sorting.
    fn sorting_to_sql(sorting: &VecDeque<(usize, ColumnSort)>) -> Option<String> {
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

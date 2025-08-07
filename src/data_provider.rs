#![allow(async_fn_in_trait)]

use crate::ColumnSort;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Range;
use leptos::prelude::RwSignal;

/// The trait that provides data for the `<TableContent>` component.
/// Anything that is passed to the `rows` prop must implement this trait.
///
/// If you add `#[table(impl_vec_data_provider)]` to your row struct,
/// this is automatically implemented for `Vec<Row>`.
/// This way a simple list of items can be passed to the table.
///
/// This is also automatically implemented for any struct that implements
/// [`PaginatedTableDataProvider`] or [`ExactTableDataProvider`].
/// The first is a more convenient way of connecting to a paginated data source and the second is
/// more convenient if you know you're always going to return exactly the requested range (except maybe
/// at the end of the data).
pub trait TableDataProvider<Row, Err: Debug = String> {
    /// If Some(...), data will be loaded in chunks of this size. This is useful for paginated data sources.
    /// If you have such a paginated data source, you probably want to implement `PaginatedTableDataProvider`
    /// instead of this trait.
    const CHUNK_SIZE: Option<usize> = None;

    /// Get all data rows for the table specified by the range. This method is called when the table is rendered.
    /// The range is determined by the visible rows and used to virtualize the table.
    /// The parameter `range` is only determined by visibility and may be out of bounds. It is the
    /// responsibility of the implementation to handle this case. Use [get_vec_range_clamped] to get a
    /// range that is clamped to the length of the vector.
    ///
    /// It returns a `Vec` of all rows loaded and the range that these rows cover. Depending on
    /// the data source you might not be able to load exactly the requested range; that's why
    /// the actual loaded range is returned in addition to the rows. You should always return
    /// at least the range that is requested or more. If you return less rows than requested,
    /// it is assumed that the data source is done and there are no more rows to load.
    ///
    /// In the case of an error the returned error `String` is going to be displayed in a
    /// in place of the failed rows.
    async fn get_rows(&self, range: Range<usize>) -> Result<(Vec<Row>, Range<usize>), Err>;

    /// Refresh data dependant side effects without reloading the data.
    /// This method is called right after get_page.
    #[allow(unused_variables)]
    fn refresh(&self, rows: Vec<RwSignal<Row>>) {
        // By default, do nothing.
    }

    /// The total number of rows in the table. Returns `None` if unknown (which is the default).
    async fn row_count(&self) -> Option<usize> {
        None
    }

    /// Set the sorting of the table. The sorting is a list of column names and the sort order sorted by priority.
    /// The first entry in the list is the most important one.
    /// The default implementation does nothing.
    /// For example: `[(0, ColumnSort::Ascending), (1, ColumnSort::Descending)]`
    /// will sort by name first and then by age.
    /// Please note that after calling this method, data will be reloaded through [`get_rows`](TableDataProvider::get_rows).
    #[allow(unused_variables)]
    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        // by default do nothing
    }

    /// Call `.track()` in this method on all signals that loading data relies on.
    /// For example a search of filters. Please check the [paginated_rest_datasource example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/paginated_rest_datasource/src/data_provider.rs)
    fn track(&self) {
        // by default do nothing
    }
}

/// A paginated data source. This is meant to provide a more convenient way
/// of connecting to a paginated data source instead of implementing [`TableDataProvider`] directly.
///
/// If you implement this for your struct, [`TableDataProvider`] is automatically implemented for you.
///
/// > Please note that this is independent from using [`DisplayStrategy::Pagination`] with [`TableContent`].
/// > You do not have implement this trait if you're using pagination and you vice versa if you're not using pagination
/// > you can still implement this trait. And in case if you use this trait together with pagination the
/// > display row count can be different from the `PAGE_ROW_COUNT`.
pub trait PaginatedTableDataProvider<Row, Err: Debug = String> {
    /// How many rows per page
    const PAGE_ROW_COUNT: usize;

    /// Get all data rows for the table specified by the page index (starts a 0).
    ///
    /// If you return less than `PAGE_ROW_COUNT` rows, it is assumed that the end of the
    /// data has been reached.
    async fn get_page(&self, page_index: usize) -> Result<Vec<Row>, Err>;

    /// Refresh data dependant side effects without reloading the data.
    /// This method is called right after get_page.
    #[allow(unused_variables)]
    fn refresh(&self, rows: Vec<RwSignal<Row>>) {
        // By default, do nothing.
    }

    /// The total number of rows in the table. Returns `None` if unknown (which is the default).
    ///
    /// By default this is computed from the [`page_count`] method. But if your data source
    /// tells you the number of rows instead of the number of pages you should override this method.
    async fn row_count(&self) -> Option<usize> {
        self.page_count().await.map(|pc| pc * Self::PAGE_ROW_COUNT)
    }

    /// The total number of pages in the data source. Returns `None` if unknown (which is the default).
    ///
    /// If your data source gives you the number of rows instead of the number of pages
    /// you should implement [`row_count`] instead of this method.
    async fn page_count(&self) -> Option<usize> {
        None
    }

    /// Same as [`TableDataProvider::set_sorting`]
    #[allow(unused_variables)]
    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        // by default do nothing
    }

    /// Same as [`TableDataProvider::track`]
    fn track(&self) {
        // by default do nothing
    }
}

impl<Row, Err, D> TableDataProvider<Row, Err> for D
where
    D: PaginatedTableDataProvider<Row, Err>,
    Err: Debug,
{
    const CHUNK_SIZE: Option<usize> = Some(D::PAGE_ROW_COUNT);

    async fn get_rows(&self, range: Range<usize>) -> Result<(Vec<Row>, Range<usize>), Err> {
        let Range { start, end } = range;

        debug_assert_eq!(start % D::PAGE_ROW_COUNT, 0);
        debug_assert_eq!(end - start, D::PAGE_ROW_COUNT);

        self.get_page(start / D::PAGE_ROW_COUNT).await.map(|rows| {
            let len = rows.len();
            (rows, start..start + len)
        })
    }

    fn refresh(&self, rows: Vec<RwSignal<Row>>) {
        PaginatedTableDataProvider::<Row, Err>::refresh(self, rows)
    }

    async fn row_count(&self) -> Option<usize> {
        PaginatedTableDataProvider::<Row, Err>::row_count(self).await
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        PaginatedTableDataProvider::<Row, Err>::set_sorting(self, sorting)
    }

    fn track(&self) {
        PaginatedTableDataProvider::<Row, Err>::track(self)
    }
}

/// Return `vec[range.start..range.end]` where `range` is clamped to the length of `vec`.
pub fn get_vec_range_clamped<T: Clone>(vec: &[T], range: Range<usize>) -> (Vec<T>, Range<usize>) {
    if vec.is_empty() {
        return (vec![], 0..0);
    }

    let start = range.start.min(vec.len() - 1);
    let end = range.end.min(vec.len());

    let return_range = start..end;

    (vec[return_range.clone()].to_vec(), return_range)
}

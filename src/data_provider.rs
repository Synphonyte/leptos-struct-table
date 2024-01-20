use crate::ColumnSort;
use async_trait::async_trait;
use std::collections::VecDeque;
use std::ops::Range;

/// The trait that provides data for the generated table component.
/// Anything that is passed to the `items` prop must implement this trait.
///
/// This is automatically implemented for `Vec<Row>`.
/// This way a simple list of items can be passed to the table.
///
/// Please note that because of the use of [`async-trait`](https://docs.rs/async-trait/latest/async_trait/) this documentation is a bit cluttered.
#[async_trait(?Send)]
pub trait TableDataProvider<Row> {
    /// Load data in chunks of multiples of this size.
    const PREFERRED_CHUNK_SIZE: usize = 20;

    /// Get all data rows for the table specified by the range. This method is called when the table is rendered.
    /// The range is determined by the visible rows and used to virtualize the table.
    /// The parameter `range` is only determined by visibility and may be out of bounds. It is the
    /// responsibility of the implementation to handle this case. Use [get_vec_range_clamped] to get a
    /// range that is clamped to the length of the vector.
    ///
    /// It returns a `Vec` of all rows loaded and the range that these rows cover. Depending on
    /// the data source you might not be able to load exactly the requested range; that's why
    /// the actual loaded range is returned in addition to the rows.
    ///
    /// In the case of an error the `String` is going to be displayed in a table row in place
    /// of the failed data.
    async fn get_rows(&self, range: Range<usize>) -> Result<(Vec<Row>, Range<usize>), String>;

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
}

/// Return `vec[range.start..range.end]` where `range` is clamped to the length of `vec`.
pub fn get_vec_range_clamped<T: Clone>(
    vec: &Vec<T>,
    range: Range<usize>,
) -> (Vec<T>, Range<usize>) {
    if vec.is_empty() {
        return (vec![], 0..0);
    }

    let start = range.start.min(vec.len() - 1);
    let end = range.end.min(vec.len());

    let return_range = start..end;

    (vec[return_range.clone()].to_vec(), return_range)
}

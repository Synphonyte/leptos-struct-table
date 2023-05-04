use crate::ColumnSort;
use async_trait::async_trait;
use core::fmt::Debug;
use std::collections::VecDeque;
use std::ops::Range;

/// The trait that provides data for the generated table component.
/// Anything that is passed to the `items` prop must implement this trait.
///
/// This is automatically implemented for `Vec<T>`.
/// This way a simple list of items can be passed to the table.
#[async_trait(?Send)]
pub trait TableDataProvider<T>
where
    T: Debug + PartialEq,
{
    type ColumnName: Copy;

    /// Get all data rows for the table specified by the range. This method is called when the table is rendered.
    /// The range is determined by the visible rows and used to virtualize the table.
    /// The parameter `range` is only determined by visibility and may be out of bounds. It is the
    /// responsibility of the implementation to handle this case.
    async fn get_rows(&self, range: Range<usize>) -> Vec<T>;

    /// Set the sorting of the table. The sorting is a list of column names and the sort order sorted by priority.
    /// The first entry in the list is the most important one.
    /// The default implementation does nothing.
    /// For example: `[(Column::Name, ColumnSort::Ascending), (Column::Age, ColumnSort::Descending)]`
    /// will sort by name first and then by age.
    /// Please note that after calling this method, data will be reloaded through [`get_rows`].
    fn set_sorting(&mut self, _sorting: &VecDeque<(Self::ColumnName, ColumnSort)>) {
        // by default do nothing
    }
}

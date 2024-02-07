use leptos::*;

/// The display acceleration strategy. Defaults to `Virtualization`.
#[derive(Copy, Clone, Default)]
pub enum DisplayStrategy {
    /// Only visible rows (plus some extra) will be displayed but the scrollbar
    /// will seem as if all rows are there.
    ///
    /// If the data provider doesn't know how many rows there are (i.e. [`TableDataProvider::row_count`]
    /// returns `None`), this will be the same as `InfiniteScroll`.
    #[default]
    Virtualization,

    /// Only the amount of rows specified is shown. Once the user scrolls down,
    /// more rows will be loaded. The scrollbar handle will shrink progressively
    /// as more and more rows are loaded.
    InfiniteScroll,

    // TODO : LoadMore(usize),
    /// Only the amount of rows specified is shown at a time. You can use the
    /// `controller` to manipulate which page of rows is shown.
    /// Scrolling will have no effect on what rows are loaded.
    ///
    /// > Please note that this will work wether your data source implements
    /// > [`PaginatedTableDataProvider`] or [`TableDataProvider`] directly.
    /// > Also `row_count` can be different from `PaginatedTableDataProvider::PAGE_ROW_COUNT`.
    Pagination {
        row_count: usize,
        controller: PaginationController,
    },
}

impl DisplayStrategy {
    pub(crate) fn set_row_count(&self, row_count: usize) {
        match self {
            Self::Pagination {
                row_count: page_row_count,
                controller,
            } => {
                controller
                    .page_count_signal
                    .set(Some(row_count / *page_row_count + 1));
            }
            _ => {
                // do nothing
            }
        }
    }
}

/// Allows to control what page is displayed as well as reading the page count and current page
#[derive(Copy, Clone)]
pub struct PaginationController {
    /// The current page. The first page is `0`.
    pub current_page: RwSignal<usize>,
    page_count_signal: RwSignal<Option<usize>>,
}

impl Default for PaginationController {
    fn default() -> Self {
        Self {
            // the value here doesn't really matter. We'll react only to changes later
            current_page: create_rw_signal(0),
            page_count_signal: create_rw_signal(None),
        }
    }
}

impl PaginationController {
    /// Call this to go to the next page
    pub fn next(&self) {
        self.current_page.set(self.current_page.get_untracked() + 1);
    }

    /// Call this to go to the previous page
    pub fn previous(&self) {
        self.current_page
            .set(self.current_page.get_untracked().saturating_sub(1));
    }

    /// Returns a `Signal` of the page count once loaded. Depending on your table data provider
    /// this might not be available and thus always be `None`.
    pub fn page_count(&self) -> Signal<Option<usize>> {
        self.page_count_signal.into()
    }
}

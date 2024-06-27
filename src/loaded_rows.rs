use std::ops::{Index, Range};

#[derive(Clone)]
pub enum RowState<T: Clone> {
    /// The row is not yet loaded and a placeholder is displayed if the row is visible in the viewport.
    Placeholder,
    /// The row is loading and a placeholder is displayed if the row is visible in the viewport.
    Loading,
    /// The row has been loaded.
    Loaded(T),
    /// The row failed to load. This error is shown in the row if it's visible in the viewport.
    Error(String),
}

impl<T: Clone> std::fmt::Debug for RowState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RowState::Placeholder => write!(f, "Placeholder"),
            RowState::Loading => write!(f, "Loading"),
            RowState::Loaded(_) => write!(f, "Loaded"),
            RowState::Error(e) => write!(f, "Error({})", e),
        }
    }
}

/// This is basically a cache for rows and used by [`TableContent`] internally to track
/// which rows are already loaded, which are still loading and which are missing.
pub struct LoadedRows<T: Clone> {
    rows: Vec<RowState<T>>,
}

impl<T: Clone> LoadedRows<T> {
    pub fn new() -> Self {
        Self { rows: vec![] }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    #[inline]
    pub fn resize(&mut self, len: usize) {
        self.rows.resize(len, RowState::Placeholder);
    }

    pub fn write_loading(&mut self, range: Range<usize>) {
        if range.end > self.rows.len() {
            self.rows.resize(range.end, RowState::Placeholder);
        }

        for row in &mut self.rows[range] {
            *row = RowState::Loading;
        }
    }

    pub fn write_loaded(
        &mut self,
        loading_result: Result<(Vec<T>, Range<usize>), String>,
        missing_range: Range<usize>,
    ) {
        match loading_result {
            Ok((rows, range)) => {
                if range.end > self.rows.len() {
                    self.rows.resize(range.end, RowState::Placeholder);
                }

                for (self_row, loaded_row) in self.rows[range].iter_mut().zip(rows) {
                    *self_row = RowState::Loaded(loaded_row);
                }
            }
            Err(error) => {
                let range = missing_range.start..missing_range.end.min(self.rows.len());
                if range.start >= range.end {
                    return;
                }

                for row in &mut self.rows[range] {
                    *row = RowState::Error(error.clone());
                }
            }
        }
    }

    #[inline]
    pub fn missing_range(&self, range: Range<usize>) -> Option<Range<usize>> {
        let do_load_predicate = |row| matches!(row, &RowState::Placeholder);

        let slice = &self.rows[range.clone()];

        let start = slice.iter().position(do_load_predicate)?;
        let end = slice.iter().rposition(do_load_predicate)?;

        let start = start + range.start;
        let end = end + range.start + 1;

        Some(start..end)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.rows.fill(RowState::Placeholder);
    }
}

impl<T: Clone> Index<Range<usize>> for LoadedRows<T> {
    type Output = [RowState<T>];

    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T: Clone> Index<usize> for LoadedRows<T> {
    type Output = RowState<T>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

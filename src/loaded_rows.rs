use std::ops::{Index, Range};

#[derive(Clone, Debug)]
pub enum RowState<T: Clone> {
    Placeholder,
    Loading,
    Loaded(T),
    Error(String),
}

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

    pub fn splice_loading(&mut self, range: Range<usize>) {
        if range.end > self.rows.len() {
            self.rows.resize(range.end, RowState::Placeholder);
        }

        self.rows
            .splice(range, std::iter::repeat(RowState::Loading));
    }

    pub fn splice_loaded(
        &mut self,
        loading_result: &Result<(Vec<T>, Range<usize>), (String, Range<usize>)>,
    ) {
        match loading_result {
            Ok((rows, range)) => {
                if range.end > self.rows.len() {
                    self.rows.resize(range.end, RowState::Placeholder);
                }

                self.rows.splice(
                    range.clone(),
                    rows.into_iter().cloned().map(RowState::Loaded),
                );
            }
            Err((error, range)) => {
                self.rows.splice(
                    range.clone(),
                    std::iter::repeat(RowState::Error(error.clone())),
                );
            }
        }
    }

    #[inline]
    pub fn missing_range(&self, range: Range<usize>) -> Option<Range<usize>> {
        let do_load_predicate = |row| matches!(row, &RowState::Placeholder | &RowState::Error(_));

        let start = self.rows[range.clone()]
            .iter()
            .position(do_load_predicate)?;
        let end = self.rows[range].iter().rposition(do_load_predicate)?;

        Some(start..end + 1)
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

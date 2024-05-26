use crate::{ColumnSort, TableHeadEvent};
use std::collections::VecDeque;

/// Sorting mode
#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum SortingMode {
    /// The table can be sorted by only one single column at a time
    SingleColumn,

    /// The table can be sorted by multiple columns ordered by priority
    #[default]
    MultiColumn,
}

impl SortingMode {
    pub fn update_sorting_from_event(
        &self,
        sorting: &mut VecDeque<(usize, ColumnSort)>,
        event: TableHeadEvent,
    ) {
        let (i, (_, mut sort)) = sorting
            .iter()
            .enumerate()
            .find(|(_, (col_index, _))| col_index == &event.index)
            .unwrap_or((0, &(event.index, ColumnSort::None)));

        if i == 0 || sort == ColumnSort::None {
            sort = match sort {
                ColumnSort::None => ColumnSort::Ascending,
                ColumnSort::Ascending => ColumnSort::Descending,
                ColumnSort::Descending => ColumnSort::None,
            };
        }

        *sorting = sorting
            .clone()
            .into_iter()
            .filter(|(col_index, sort)| *col_index != event.index && *sort != ColumnSort::None)
            .collect();

        if sort != ColumnSort::None {
            sorting.push_front((event.index, sort));
        }

        if self == &SortingMode::SingleColumn {
            sorting.truncate(1);
        }
    }
}

use leptos::*;
use std::collections::HashSet;

/// Type of selection together with the `RwSignal` to hold the selection
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Selection {
    /// No selection possible (the default).
    #[default]
    None,

    /// Allow only one row to be selected at a time. `None` if no rows are selected.
    /// `Some(<row index>)` if a row is selected.
    Single(RwSignal<Option<usize>>),

    /// Allow multiple rows to be selected at a time. Each entry in the `Vec`
    /// is the index of a selected row.
    Multiple(RwSignal<HashSet<usize>>),
}

impl Selection {
    /// Clear the selection
    pub fn clear(&self) {
        match self {
            Selection::None => {}
            Selection::Single(selected_index) => {
                selected_index.set(None);
            }
            Selection::Multiple(selected_indices) => {
                selected_indices.update(|selected_indices| {
                    selected_indices.clear();
                });
            }
        }
    }
}

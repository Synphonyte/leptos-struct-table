#![deny(missing_docs)]
//! Simple showcase example.
use crate::grouping::GroupRow;
use crate::grouping::GroupTableRowRenderer;
use crate::grouping::GroupingInfo;
use leptos::prelude::*;
use leptos_struct_table::*;
use std::sync::Arc;

mod grouping;

/// Makes a number-cell's content red
#[component]
#[allow(unused_variables)]
pub fn RedCellRenderer(
    /// Class
    class: String,
    /// Value
    value: Signal<f64>,
    /// Row data
    row: RwSignal<Flower>,
    /// Column index
    index: FlowerColumn,
) -> impl IntoView {
    view! {
        <td class=class>
            <span class="text-red-500">{ move || value.get() }</span>
        </td>
    }
}

/// Entry of a flower measurment data-set.
#[derive(TableRow, Clone, Default, Debug)]
#[table(
    impl_vec_data_provider,
    column_index_type = "enum",
    classes_provider = "TailwindClassesPreset"
)]
pub struct Flower {
    species: String,
    sepal_width: f64,
    sepal_length: f64,
    petal_width: f64,
    #[table(renderer = "RedCellRenderer")]
    petal_length: f64,

    #[table(skip)]
    grouping_info: GroupingInfo<FlowerColumn>,
}

// Derive GroupRow to make the row groupable
impl GroupRow<FlowerColumn> for Flower {
    fn group_info(&self) -> &GroupingInfo<FlowerColumn> {
        &self.grouping_info
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let grouped_by = Arc::new(vec![FlowerColumn::Species, FlowerColumn::SepalWidth]);
        let rows = vec![
            Flower {
                species: "Versicolor".to_string(),
                sepal_width: 2.0,
                sepal_length: 5.0,
                petal_width: 1.0,
                petal_length: 3.5,
                grouping_info: GroupingInfo {
                    row_index: 0,
                    nb_entries: 1,
                    grouped_by: grouped_by.clone(),
                },
            },
            Flower {
                species: "Versicolor".to_string(),
                sepal_width: 2.2,
                sepal_length: 6.0,
                petal_width: 1.0,
                petal_length: 4.0,
                grouping_info: GroupingInfo {
                    row_index: 0,
                    nb_entries: 2,
                    grouped_by: grouped_by.clone(),
                },
            },
            Flower {
                species: "Versicolor".to_string(),
                sepal_width: 2.2,
                sepal_length: 6.2,
                petal_width: 1.5,
                petal_length: 4.5,
                grouping_info: GroupingInfo {
                    row_index: 1,
                    nb_entries: 2,
                    grouped_by: grouped_by.clone(),
                },
            },
            Flower {
                species: "Virginica".to_string(),
                sepal_width: 2.2,
                sepal_length: 6.0,
                petal_width: 1.5,
                petal_length: 5.0,
                grouping_info: GroupingInfo {
                    row_index: 0,
                    nb_entries: 1,
                    grouped_by: grouped_by.clone(),
                },
            },
            Flower {
                species: "Setosa".to_string(),
                sepal_width: 2.3,
                sepal_length: 4.5,
                petal_width: 0.3,
                petal_length: 1.3,
                grouping_info: GroupingInfo {
                    row_index: 0,
                    nb_entries: 1,
                    grouped_by: grouped_by.clone(),
                },
            },
            Flower {
                species: "Versicolor".to_string(),
                sepal_width: 2.3,
                sepal_length: 5.0,
                petal_width: 1.0,
                petal_length: 3.3,
                grouping_info: GroupingInfo {
                    row_index: 0,
                    nb_entries: 3,
                    grouped_by: grouped_by.clone(),
                },
            },
            Flower {
                species: "Versicolor".to_string(),
                sepal_width: 2.3,
                sepal_length: 6.3,
                petal_width: 1.3,
                petal_length: 4.4,
                grouping_info: GroupingInfo {
                    row_index: 1,
                    nb_entries: 3,
                    grouped_by: grouped_by.clone(),
                },
            },
            Flower {
                species: "Versicolor".to_string(),
                sepal_width: 2.3,
                sepal_length: 5.5,
                petal_width: 1.3,
                petal_length: 4.0,
                grouping_info: GroupingInfo {
                    row_index: 2,
                    nb_entries: 3,
                    grouped_by: grouped_by.clone(),
                },
            },
            Flower {
                species: "Versicolor".to_string(),
                sepal_width: 2.4,
                sepal_length: 5.5,
                petal_width: 1.0,
                petal_length: 3.7,
                grouping_info: GroupingInfo {
                    row_index: 0,
                    nb_entries: 3,
                    grouped_by: grouped_by.clone(),
                },
            },
            Flower {
                species: "Versicolor".to_string(),
                sepal_width: 2.4,
                sepal_length: 4.9,
                petal_width: 1.0,
                petal_length: 3.3,
                grouping_info: GroupingInfo {
                    row_index: 1,
                    nb_entries: 3,
                    grouped_by: grouped_by.clone(),
                },
            },
        ];

        view! {
            <table>
                <TableContent
                    rows=rows
                    row_renderer=GroupTableRowRenderer
                    scroll_container="html"
                />
            </table>
        }
    })
}

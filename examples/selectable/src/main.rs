mod tailwind;

use ::chrono::NaiveDate;
use leptos::*;
use leptos_struct_table::*;
use tailwind::TailwindClassesPreset;

// This generates the component BookTable
#[derive(TableRow, Clone)]
#[table(
    sortable,
    classes_provider = "TailwindClassesPreset",
    impl_vec_data_provider
)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    #[table(
        cell_class = "text-red-600 dark:text-red-400",
        head_class = "text-red-700 dark:text-red-300"
    )]
    pub publish_date: NaiveDate,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let rows = vec![
            Book {
                id: 1,
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
                publish_date: NaiveDate::from_ymd_opt(1925, 4, 10).unwrap(),
            },
            Book {
                id: 2,
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: NaiveDate::from_ymd_opt(1939, 4, 14).unwrap(),
            },
            Book {
                id: 3,
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: NaiveDate::from_ymd_opt(1949, 6, 8).unwrap(),
            },
            Book {
                id: 4,
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: NaiveDate::from_ymd_opt(1922, 2, 2).unwrap(),
            },
        ];

        let selected_index = create_rw_signal(None);
        let (selected_row, set_selected_row) = create_signal(None);

        view! {
            <div class="rounded-md overflow-clip m-10 border dark:border-gray-700 float-left">
                <table class="text-sm text-left text-gray-500 dark:text-gray-400 mb-[-1px]">
                    <TableContent
                        rows=rows
                        selection=Selection::Single(selected_index)
                        row_class="select-none"
                        on_selection_change={move |evt: SelectionChangeEvent<Book>| {
                            set_selected_row.update(|selected_row| {
                                *selected_row = Some(evt.row);
                            })
                        }}
                        sorting_mode=SortingMode::SingleColumn
                    />
                </table>
            </div>

            { move || selected_row.get().map(|selected_row| {
                view! {
                    <div class="rounded-md overflow-clip m-10 border dark:border-gray-700 float-left px-5 py-3 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-400">
                        <pre>
                            "          Id:  " {selected_row.id} "\n"
                            "       Title:  " {selected_row.title} "\n"
                            "      Author:  " {selected_row.author} "\n"
                            "Publish Date:  " {selected_row.publish_date.to_string()}
                        </pre>
                    </div>
                }
            }) }
        }
    })
}

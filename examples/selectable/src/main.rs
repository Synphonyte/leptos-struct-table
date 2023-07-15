mod tailwind;

use chrono::NaiveDate;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
use tailwind::TailwindClassesPreset;

// This generates the component BookTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(
    sortable,
    selection_mode = "single",
    row_class = "select-none",
    classes_provider = "TailwindClassesPreset"
)]
pub struct Book {
    #[table(key)]
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

    mount_to_body(|cx| {
        let provider = store_value(
            cx,
            MemoryStorage::new(vec![
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
            ]),
        );

        let selected_key = create_rw_signal(cx, None);

        create_effect(cx, move |_| log!("{:?}", selected_key.get()));

        view! { cx,
            <div class="rounded-md overflow-clip m-10 border dark:border-gray-700 float-left".to_string()>
                <BookTable class="mb-[-1px]".to_string() data_provider=provider selected_key=selected_key/>
            </div>
        }
    })
}

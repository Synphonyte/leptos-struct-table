use chrono::NaiveDate;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

// This generates the component BookTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(sortable, classes_provider = "BootstrapClassesPreset")]
pub struct Book {
    #[table(key)]
    pub id: u32,
    pub title: String,
    pub author: String,
    // TODO:
    // #[table(
    //     cell_class = "text-red-600 dark:text-red-400",
    //     head_class = "text-red-700 dark:text-red-300"
    // )]
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

        view! { cx,
            <div class="container".to_string()>
                <BookTable class="table-hover".to_string() data_provider=provider />
            </div>
        }
    })
}

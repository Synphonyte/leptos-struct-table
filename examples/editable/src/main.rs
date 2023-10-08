mod renderer;
mod tailwind;

use crate::renderer::*;
use async_trait::async_trait;
use chrono::NaiveDate;
use leptos::logging::log;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
use tailwind::TailwindClassesPreset;

// This generates the component BookTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(classes_provider = "TailwindClassesPreset")]
pub struct Book {
    #[table(key)]
    pub id: u32,
    #[table(renderer = "InputCellRenderer")]
    pub title: String,
    #[table(renderer = "InputCellRenderer")]
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
        let items = create_rw_signal(vec![
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
        ]);

        let on_change = move |evt: TableChangeEvent<Book, BookColumnName, BookColumnValue>| {
            items.update(|items| {
                items[evt.row_index] = match evt.new_value {
                    BookColumnValue::Author(author) => Book {
                        author,
                        ..evt.old_row
                    },
                    BookColumnValue::Title(title) => Book {
                        title,
                        ..evt.old_row
                    },
                    _ => unreachable!(),
                };
            });
        };

        view! {
            <div class="rounded-md overflow-clip m-10 border dark:border-gray-700 w-[50%]".to_string()>
                <BookTable class="mb-[-1px]".to_string() items=items on_change=on_change/>
            </div>

            <pre>{move || format!("{:#?}", items())}</pre>
        }
    })
}

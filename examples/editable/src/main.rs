mod renderer;
mod tailwind;

use crate::renderer::*;
use ::chrono::NaiveDate;
use leptos::prelude::*;
use leptos_struct_table::*;
use std::ops::Range;
use tailwind::TailwindClassesPreset;

// This generates the component BookTable
#[derive(TableRow, Clone, Debug)]
#[table(classes_provider = "TailwindClassesPreset")]
pub struct Book {
    pub id: u32,
    #[table(renderer = "InputCellRenderer")]
    pub title: String,
    #[table(renderer = "InputCellRenderer")]
    pub author: String,
    pub publish_date: NaiveDate,
}

impl TableDataProvider<Book> for RwSignal<Vec<Book>> {
    async fn get_rows(&self, _: Range<usize>) -> Result<(Vec<Book>, Range<usize>), String> {
        let books = self.get_untracked().to_vec();
        let len = books.len();
        Ok((books, 0..len))
    }

    async fn row_count(&self) -> Option<usize> {
        Some(self.get_untracked().len())
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let rows = RwSignal::new(vec![
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

        let on_change = move |evt: ChangeEvent<Book>| {
            rows.write()[evt.row_index] = evt.changed_row.get_untracked();
        };

        view! {
            <div class="rounded-md overflow-clip m-10 border dark:border-gray-700 w-[50%]"
                .to_string()>
                <table class="text-sm text-left text-gray-500 dark:text-gray-400 mb-[-1px]">
                    <TableContent rows on_change scroll_container="html" />
                </table>
            </div>

            <pre>{move || format!("{:#?}", rows.get())}</pre>
        }
    })
}

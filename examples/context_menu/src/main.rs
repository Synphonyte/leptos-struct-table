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

impl TableDataProvider<Book, usize> for RwSignal<Vec<Book>> {
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

        // The table caches the rows it got from the data provider. When rows are added
        // or removed, we have to tell it to load them again.
        let reload = ReloadController::default();

        // (x, y, row index) of the open context menu
        let menu = RwSignal::new(None::<(i32, i32, usize)>);

        let on_row_context_menu = move |evt: RowContextMenuEvent<Book>| {
            // suppress the browser's own context menu
            evt.mouse_event.prevent_default();
            menu.set(Some((
                evt.mouse_event.client_x(),
                evt.mouse_event.client_y(),
                evt.row_index,
            )));
        };

        let add_row_after = move |index: usize| {
            rows.update(|rows| {
                let id = rows.iter().map(|b| b.id).max().unwrap_or(0) + 1;
                rows.insert(
                    (index + 1).min(rows.len()),
                    Book {
                        id,
                        title: String::new(),
                        author: String::new(),
                        publish_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
                    },
                );
            });
            reload.reload();
        };

        let delete_row = move |index: usize| {
            rows.update(|rows| {
                if index < rows.len() {
                    rows.remove(index);
                }
            });
            reload.reload();
        };

        view! {
            <div class="rounded-md overflow-clip m-10 border border-gray-300 dark:border-gray-700 w-[50%]"
                .to_string()>
                <table class="text-sm text-left text-gray-500 dark:text-gray-400 mb-[-1px]">
                    <TableContent rows on_change scroll_container="" reload_controller=reload on_row_context_menu />
                </table>
            </div>

            {move || menu.get().map(|(x, y, index)| view! {
                // click anywhere outside the menu closes it
                <div
                    class="fixed inset-0 z-40"
                    on:click=move |_| menu.set(None)
                    on:contextmenu=move |ev| {
                        ev.prevent_default();
                        menu.set(None);
                    }
                ></div>
                <ul
                    class="fixed z-50 min-w-40 rounded-md border border-gray-300 bg-white py-1 text-sm shadow-lg"
                    style=format!("left:{x}px;top:{y}px")
                >
                    <li>
                        <button
                            class="w-full px-3 py-1.5 text-left hover:bg-gray-100"
                            on:click=move |_| {
                                menu.set(None);
                                add_row_after(index);
                            }
                        >
                            "Add row after"
                        </button>
                    </li>
                    <li>
                        <button
                            class="w-full px-3 py-1.5 text-left hover:bg-gray-100"
                            on:click=move |_| {
                                menu.set(None);
                                delete_row(index);
                            }
                        >
                            "Delete row"
                        </button>
                    </li>
                </ul>
            })}

            <pre>{move || format!("{:#?}", rows.get())}</pre>
        }
    })
}

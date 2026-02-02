#![deny(missing_docs)]
//! Column order, reordering and visibility showcase example.

mod tailwind;

use crate::tailwind::TailwindClassesPreset;
use ::chrono::NaiveDate;
use derive_more::{Deref, DerefMut};
use leptos::prelude::*;
use leptos_struct_table::*;
use std::sync::Arc;

/// This generates the component BookTable
#[derive(TableRow, Clone)]
#[table(
    sortable,
    impl_vec_data_provider,
    column_index_type = "enum",
    classes_provider = "TailwindClassesPreset"
)]
pub struct Book {
    /// Title of the book.
    pub title: String,

    /// Author of the book.
    pub author: String,
    /// Date when book has been published.
    pub publish_date: Option<NaiveDate>,
    /// Description of the book. Optional.
    #[table(none_value = "-")]
    pub description: Option<String>,
}

/// New-type pattern because otherwise the impl TableRow doesn't work because of orphan rules.
#[derive(Deref, DerefMut, Clone)]
pub struct ArcBook(Arc<Book>);

struct CustomHeadDragHandler;

impl DragHandler<BookColumn> for CustomHeadDragHandler {
    fn grabbed_class(&self) -> &'static str {
        "outline outline-blue-500 outline-dashed -outline-offset-1 bg-blue-500/10"
    }

    fn hover_left_class(&self) -> &'static str {
        "relative border-l-2 border-blue-500 after:content-[''] after:absolute after:left-0 after:top-0 after:w-0 after:h-0 after:border-l-[6px] after:border-l-transparent after:border-r-[6px] after:border-r-transparent after:border-t-[8px] after:border-t-blue-500 after:-translate-x-[7px]"
    }

    fn hover_right_class(&self) -> &'static str {
        "relative border-r-2 border-blue-500 after:content-[''] after:absolute after:right-0 after:top-0 after:w-0 after:h-0 after:border-r-[6px] after:border-r-transparent after:border-l-[6px] after:border-l-transparent after:border-t-[8px] after:border-t-blue-500 after:translate-x-[7px]"
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let rows = vec![
            Book {
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1925, 4, 10).unwrap()),
                description: Some(
                    "A story of wealth, love, and the American Dream in the 1920s.".to_string(),
                ),
            },
            Book {
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1939, 4, 14).unwrap()),
                description: None,
            },
            Book {
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1949, 6, 8).unwrap()),
                description: None,
            },
            Book {
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1922, 2, 2).unwrap()),
                description: None,
            },
        ];

        let columns = RwSignal::new(Vec::from(Book::columns()));

        view! {
            <div class="m-10">
                <fieldset class="float-left mb-5 mr-5 border border-gray-300 dark:border-gray-700 rounded-md px-5">
                    <legend class="text-sm font-semibold text-gray-700 dark:text-gray-300 px-2">Visible Columns:</legend>
                    {Book::columns().iter().enumerate().map(|(idx, book)| view! {
                        <div class="flex items-center my-4">
                            <input
                                id=format!("column-checkbox-{idx}")
                                class="w-4 h-4 border border-default-medium rounded-xs bg-neutral-secondary-medium focus:ring-2 focus:ring-brand-soft"
                                type="checkbox"
                                checked
                                on:click=move |_| {
                                    let mut columns_internal = columns.get();
                                    if !columns_internal.contains(book) {
                                        let idx = columns_internal.iter().filter(|c| *c < book).count();
                                        columns_internal.insert(idx, *book);
                                    } else {
                                        columns_internal.retain(|c| c != book)
                                    }
                                    columns.set(columns_internal);
                                }/>
                                <label for=format!("column-checkbox-{idx}") class="ms-2 text-sm text-heading select-none">{ format!("{book:?}") }</label>

                        </div>
                    }).collect_view()}
                </fieldset>

                <div class="float-left rounded-md border border-gray-300 dark:border-gray-700 overflow-clip">
                    <table class="text-sm text-left text-gray-500 dark:text-gray-400 mb-[-1px]">
                        <TableContent rows scroll_container="html" columns drag_handler=HeadDragHandler::new(CustomHeadDragHandler) />
                    </table>
                </div>
            </div>
        }
    })
}

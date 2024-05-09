#![deny(missing_docs)]
//! Simple showcase example.

use ::chrono::NaiveDate;
use derive_more::{Deref, DerefMut};
use leptos::*;
use leptos_struct_table::*;
use std::rc::Rc;

/// This generates the component BookTable
#[derive(TableRow)]
#[table(sortable, impl_vec_data_provider, row_type = "RcBook")]
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
pub struct RcBook(Rc<Book>);

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let rows = vec![
            RcBook(Rc::new(Book {
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1925, 4, 10).unwrap()),
                description: Some(
                    "A story of wealth, love, and the American Dream in the 1920s.".to_string(),
                ),
            })),
            RcBook(Rc::new(Book {
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1939, 4, 14).unwrap()),
                description: None,
            })),
            RcBook(Rc::new(Book {
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1949, 6, 8).unwrap()),
                description: None,
            })),
            RcBook(Rc::new(Book {
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1922, 2, 2).unwrap()),
                description: None,
            })),
        ];

        view! {
            <table>
                <TableContent rows/>
            </table>
        }
    })
}

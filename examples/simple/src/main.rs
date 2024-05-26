#![deny(missing_docs)]
//! Simple showcase example.

use ::chrono::NaiveDate;
use ::time::Date;
use ::uuid::Uuid;
use leptos::*;
use leptos_struct_table::*;

/// This generates the component BookTable
#[derive(TableRow, Clone)]
#[table(sortable, impl_vec_data_provider)]
pub struct Book {
    /// Id of the entry.
    pub id: Uuid,
    /// Title of the book.
    pub title: String,
    /// Author of the book.
    pub author: String,
    /// Date when book has been published.
    pub publish_date: Option<NaiveDate>,
    /// Date when book was read
    pub read_date: Option<Date>,
    /// Description of the book. Optional.
    #[table(none_value = "-")]
    pub description: Option<String>,
    /// Example on hidden member.
    #[table(skip)]
    pub hidden_field: String,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let rows = vec![
            Book {
                id: Uuid::new_v4(),
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1925, 4, 10).unwrap()),
                read_date: Some(Date::from_calendar_date(2024, ::time::Month::January, 2).unwrap()),
                description: Some(
                    "A story of wealth, love, and the American Dream in the 1920s.".to_string(),
                ),
                hidden_field: "hidden".to_string(),
            },
            Book {
                id: Uuid::new_v4(),
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1939, 4, 14).unwrap()),
                read_date: None,
                description: None,
                hidden_field: "not visible in the table".to_string(),
            },
            Book {
                id: Uuid::new_v4(),
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1949, 6, 8).unwrap()),
                read_date: None,
                description: None,
                hidden_field: "hidden".to_string(),
            },
            Book {
                id: Uuid::new_v4(),
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1922, 2, 2).unwrap()),
                read_date: None,
                description: None,
                hidden_field: "hidden".to_string(),
            },
        ];

        view! {
            <table>
                <TableContent rows=rows />
            </table>
        }
    })
}

#![deny(missing_docs)]
//! Simple showcase example.

use ::chrono::NaiveDate;
use ::uuid::Uuid;
use leptos::prelude::*;
use leptos::web_sys;
use leptos_struct_table::*;

/// Custom row renderer that adds a link to the end of the row
#[allow(unused_variables, non_snake_case)]
pub fn CustomTableRowRenderer(
    // The class attribute for the row element. Generated by the classes provider.
    class: Signal<String>,
    // The row to render.
    row: Book,
    // The index of the row. Starts at 0 for the first body row.
    index: usize,
    // The selected state of the row. True, when the row is selected.
    selected: Signal<bool>,
    // Event handler callback when this row is selected
    on_select: EventHandler<web_sys::MouseEvent>,
    // Event handler callback for changes
    on_change: EventHandler<ChangeEvent<Book>>,
) -> impl IntoView {
    let id = row.id.to_string();

    view! {
        <tr class=class on:click=move |mouse_event| on_select.run(mouse_event)>
            {row.render_row(index, on_change)}
            <td>
                <a href=move || format!("/some-path/{}", id)>"Some link"</a>
            </td>
        </tr>
    }
}

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
    /// Description of the book. Optional.
    #[table(none_value = "-")]
    pub description: Option<String>,
    /// Example on hidden member.
    #[table(skip)]
    pub hidden_field: String,
    /// Example of a headerless column
    #[table(skip_header)]
    pub rating: String,
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
                description: Some(
                    "A story of wealth, love, and the American Dream in the 1920s.".to_string(),
                ),
                hidden_field: "hidden".to_string(),
                rating: "5/5".to_string(),
            },
            Book {
                id: Uuid::new_v4(),
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1939, 4, 14).unwrap()),
                description: None,
                hidden_field: "not visible in the table".to_string(),
                rating: "4/5".to_string(),
            },
            Book {
                id: Uuid::new_v4(),
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1949, 6, 8).unwrap()),
                description: None,
                hidden_field: "hidden".to_string(),
                rating: "19/84".to_string(),
            },
            Book {
                id: Uuid::new_v4(),
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1922, 2, 2).unwrap()),
                description: None,
                hidden_field: "hidden".to_string(),
                rating: "really long".to_string(),
            },
        ];

        view! {
            <table>
                <TableContent
                    rows=rows
                    row_renderer=CustomTableRowRenderer
                    scroll_container="html"
                />
            </table>
        }
    })
}

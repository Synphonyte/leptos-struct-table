//! Generic showcase example.

use crate::uuid::Uuid;
use async_trait::async_trait;
use chrono::NaiveDate;
use leptos::*;
use leptos_struct_table::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

/// This generates the component BookTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Book<T>
where
    // necessary trait bounds. `IntoView` is only necessary because we require it in our custom renderer below
    // otherwise you could remove it here.
    // If you also make the table sortable then you might have to add `PartialOrd` as well.
    T: PartialEq + Debug + Clone + Serialize + DeserializeOwned + IntoView + 'static,
{
    /// Id of the entry.
    #[table(key)]
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

    /// Generic field. You have to specify a custom renderer for a generic field
    /// and provide this serde bound attribute.
    #[table(renderer = "CustomDataRenderer")]
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    pub custom_data: T,
}

#[component]
#[allow(unused_variables)]
pub fn CustomDataRenderer<T, F>(
    #[prop(into)] class: MaybeSignal<String>,
    #[prop(into)] value: MaybeSignal<T>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    T: IntoView + Clone + 'static,
    F: Fn(T) + 'static,
{
    view! {
        <td class=class>{value}</td>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let items = create_rw_signal(vec![
            Book {
                id: Uuid::default(),
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1925, 4, 10).unwrap()),
                description: Some(
                    "A story of wealth, love, and the American Dream in the 1920s.".to_string(),
                ),
                custom_data: "custom data is a string here".to_string(),
            },
            Book {
                id: Uuid::default(),
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1939, 4, 14).unwrap()),
                description: None,
                custom_data: "custom data is a string here".to_string(),
            },
            Book {
                id: Uuid::default(),
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1949, 6, 8).unwrap()),
                description: None,
                custom_data: "custom data is a string here".to_string(),
            },
            Book {
                id: Uuid::default(),
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1922, 2, 2).unwrap()),
                description: None,
                custom_data: "still a string".to_string(),
            },
        ]);

        view! {
            <BookTable items=items />
        }
    })
}

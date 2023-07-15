use chrono::NaiveDate;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

// This generates the component BookTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(sortable)]
pub struct Book {
    #[table(key)]
    pub id: u32,
    pub title: String,
    #[table(editable)]
    pub author: String,
    pub publish_date: NaiveDate,
    #[table(skip)]
    pub hidden_field: String,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        let get_current_provider_state =
            create_action(cx, move |provider: &StoredValue<MemoryStorage<Book>>| {
                let p = provider.get_value();
                async move { p.get_rows(0..1000).await.unwrap() }
            });
        let current_provider_state = get_current_provider_state.value();
        let log_provider_state = move || {
            log::debug!("Provider state:\n{:#?}", current_provider_state.get());
        };

        let range_to_show = create_rw_signal(cx, 0..4);

        let provider = store_value(
            cx,
            MemoryStorage::new(vec![
                Book {
                    id: 1,
                    title: "The Great Gatsby".to_string(),
                    author: "F. Scott Fitzgerald".to_string(),
                    publish_date: NaiveDate::from_ymd_opt(1925, 4, 10).unwrap(),
                    hidden_field: "hidden".to_string(),
                },
                Book {
                    id: 2,
                    title: "The Grapes of Wrath".to_string(),
                    author: "John Steinbeck".to_string(),
                    publish_date: NaiveDate::from_ymd_opt(1939, 4, 14).unwrap(),
                    hidden_field: "not visible in the table".to_string(),
                },
                Book {
                    id: 3,
                    title: "Nineteen Eighty-Four".to_string(),
                    author: "George Orwell".to_string(),
                    publish_date: NaiveDate::from_ymd_opt(1949, 6, 8).unwrap(),
                    hidden_field: "hidden".to_string(),
                },
                Book {
                    id: 4,
                    title: "Ulysses".to_string(),
                    author: "James Joyce".to_string(),
                    publish_date: NaiveDate::from_ymd_opt(1922, 2, 2).unwrap(),
                    hidden_field: "hidden".to_string(),
                },
            ]),
        );

        view! { cx,
            <BookTable data_provider=provider range=range_to_show />
            <button on:click=move |_| {
                get_current_provider_state.dispatch(provider);
            }>{"Refetch data"}</button>
            { log_provider_state }
        }
    })
}

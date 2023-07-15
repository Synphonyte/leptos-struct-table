use async_trait::async_trait;
use chrono::NaiveDate;
use leptos::*;
use leptos_struct_table::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::ops::Range;

// This generates the component BookTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Book {
    #[table(key)]
    pub id: u32,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub description: String,
    pub isbn: String,
    pub published: NaiveDate,
    pub publisher: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct BookDataProvider;

#[derive(Deserialize)]
struct FakerApiResponse {
    pub data: Vec<Book>,
}

// All we need to load data from a rest api is to implement TableDataProvider and it's `get_rows` methods.
#[async_trait(?Send)]
impl TableDataStorage<Book> for BookDataProvider {
    // TODO : Error handling
    async fn get_rows(&self, range: Range<usize>) -> anyhow::Result<Vec<Book>> {
        // this would be probably better with some caching for previously loaded data but for
        // simplicity we just load the data every time

        let response = Request::get(&format!(
            // faker api doesn't support real pagination so we just load all data up to the end
            "https://fakerapi.it/api/v1/books?_seed=2293&_quantity={}",
            range.end
        ))
        .send()
        .await
        .unwrap();

        let resp: FakerApiResponse = response.json().await?;

        Ok(get_vec_range_clamped(&resp.data, range))
    }
    async fn set_row(&mut self, index: usize, row: Book) -> anyhow::Result<()> {
        Ok(())
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        let provider = store_value(cx, BookDataProvider {});

        view! { cx,
            <BookTable
                data_provider=provider
            />
        }
    })
}

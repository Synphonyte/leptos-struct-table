use crate::models::{Brewery, MetaResponse};
use gloo_net::http::Request;
use leptos::prelude::*;
use leptos_struct_table::{ColumnSort, PaginatedTableDataProvider};
use std::collections::VecDeque;

pub struct BreweryDataProvider {
    sorting: VecDeque<(usize, ColumnSort)>,
    pub search: RwSignal<String>,
}

impl Default for BreweryDataProvider {
    fn default() -> Self {
        Self {
            sorting: VecDeque::new(),
            search: RwSignal::new("".to_string()),
        }
    }
}

impl BreweryDataProvider {
    fn url_sort_param_for_column(&self, column: usize) -> &'static str {
        match column {
            0 => "name",
            1 => "city",
            2 => "country",
            _ => "",
        }
    }

    fn url_sort_param_for_sort_pair(&self, pair: &(usize, ColumnSort)) -> Option<(&str, String)> {
        let col = self.url_sort_param_for_column(pair.0);

        let dir = match pair.1 {
            ColumnSort::Ascending => "asc",
            ColumnSort::Descending => "desc",
            ColumnSort::None => return None,
        };

        Some(("sort", format!("{}:{}", col, dir)))
    }

    fn get_search_query_pair(&self) -> Option<(&str, String)> {
        let search = self.search.get_untracked();

        if !search.is_empty() {
            Some(("by_name", search))
        } else {
            None
        }
    }

    fn get_url(&self, page_index: usize) -> (&str, impl IntoIterator<Item = (&str, String)>) {
        let mut query = vec![];

        for pair in &self.sorting {
            if let Some(query_pair) = self.url_sort_param_for_sort_pair(pair) {
                query.push(query_pair);
            }
        }

        if let Some(query_pair) = self.get_search_query_pair() {
            query.push(query_pair);
        }

        query.push(("page", format!("{}", page_index + 1)));
        query.push(("per_page", format!("{}", Self::PAGE_ROW_COUNT)));

        ("https://api.openbrewerydb.org/v1/breweries", query)
    }
}

impl PaginatedTableDataProvider<Brewery, usize> for BreweryDataProvider {
    const PAGE_ROW_COUNT: usize = 200;

    async fn get_page(&self, page_index: usize) -> Result<Vec<Brewery>, String> {
        if page_index >= 10000 / Self::PAGE_ROW_COUNT {
            return Ok(vec![]);
        }

        let (url, query) = self.get_url(page_index);

        let resp: Vec<Brewery> = Request::get(url)
            .query(query)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

        Ok(resp)
    }

    async fn row_count(&self) -> Option<usize> {
        let mut query = Vec::new();

        if let Some(query_pair) = self.get_search_query_pair() {
            query.push(query_pair);
        }

        let resp: MetaResponse = Request::get("https://api.openbrewerydb.org/v1/breweries/meta")
            .query(query)
            .send()
            .await
            .map_err(|e| leptos::logging::error!("{e}"))
            .ok()?
            .json()
            .await
            .map_err(|e| leptos::logging::error!("{e}"))
            .ok()?;

        Some(resp.total)
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sorting = sorting.clone();
    }

    fn track(&self) {
        // we depend on the search so we need to track it here
        self.search.track();
    }
}

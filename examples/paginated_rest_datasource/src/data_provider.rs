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

    fn url_sort_param_for_sort_pair(&self, pair: &(usize, ColumnSort)) -> String {
        let col = self.url_sort_param_for_column(pair.0);

        let dir = match pair.1 {
            ColumnSort::Ascending => "asc",
            ColumnSort::Descending => "desc",
            ColumnSort::None => return "".to_string(),
        };

        format!("&sort={}:{}", col, dir)
    }

    fn get_url(&self, page_index: usize) -> String {
        let mut sort = String::new();
        for pair in &self.sorting {
            sort.push_str(&self.url_sort_param_for_sort_pair(pair));
        }

        format!(
            "https://api.openbrewerydb.org/v1/breweries?by_name={}{sort}&page={}&per_page={}",
            self.search.get_untracked(),
            page_index + 1,
            Self::PAGE_ROW_COUNT,
        )
    }
}

impl PaginatedTableDataProvider<Brewery, usize> for BreweryDataProvider {
    const PAGE_ROW_COUNT: usize = 200;

    async fn get_page(&self, page_index: usize) -> Result<Vec<Brewery>, String> {
        if page_index >= 10000 / Self::PAGE_ROW_COUNT {
            return Ok(vec![]);
        }

        let url = self.get_url(page_index);

        let resp: Vec<Brewery> = Request::get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

        Ok(resp)
    }

    async fn row_count(&self) -> Option<usize> {
        let url = format!("https://api.openbrewerydb.org/v1/breweries/meta?by_name={}", self.search.get_untracked());
        let resp: Option<MetaResponse> = Request::get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())
            .ok()?
            .json()
            .await
            .map_err(|e| e.to_string())
            .ok()?;

        let count = resp.map(|r| r.total)?.parse::<usize>().ok()?;

        Some(count)
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sorting = sorting.clone();
    }

    fn track(&self) {
        // we depend on the search so we need to track it here
        self.search.track();
    }
}

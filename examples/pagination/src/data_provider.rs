use crate::models::{ArchiveOrgApiResponse, ArchiveOrgCountRespone, Book};
use gloo_net::http::Request;
use leptos::logging;
use leptos_struct_table::{ColumnSort, PaginatedTableDataProvider};
use std::collections::VecDeque;

pub struct BookDataProvider {
    sorting: VecDeque<(usize, ColumnSort)>,
}

impl Default for BookDataProvider {
    fn default() -> Self {
        Self {
            sorting: VecDeque::new(),
        }
    }
}

impl BookDataProvider {
    fn url_sort_param_for_column(&self, column: usize) -> &'static str {
        match column {
            0 => "identifierSorter",
            1 => "titleSorter",
            2 => "creatorSorter",
            3 => "publicdate",
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

        format!("&sort%5B%5D={}+{}", col, dir)
    }

    fn get_url(&self, page_index: usize) -> String {
        let mut sort = String::new();
        for pair in &self.sorting {
            sort.push_str(&self.url_sort_param_for_sort_pair(pair));
        }

        format!(
                "https://archive.org/advancedsearch.php?q=creator%3A%28Lewis%29&fl%5B%5D=creator&fl%5B%5D=identifier&fl%5B%5D=publicdate&fl%5B%5D=title{sort}&rows={}&page={}&output=json&callback=",
                Self::PAGE_ROW_COUNT,
                page_index + 1,
            )
    }
}

impl PaginatedTableDataProvider<Book> for BookDataProvider {
    const PAGE_ROW_COUNT: usize = 50;

    async fn get_page(&self, page_index: usize) -> Result<Vec<Book>, String> {
        if page_index >= 10000 / Self::PAGE_ROW_COUNT {
            return Ok(vec![]);
        }

        let url = self.get_url(page_index);

        let resp: ArchiveOrgApiResponse = Request::get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

        match resp {
            ArchiveOrgApiResponse::Err { error } => Err(error),
            ArchiveOrgApiResponse::Ok { response } => Ok(response.docs),
        }
    }

    async fn row_count(&self) -> Option<usize> {
        let resp: Option<ArchiveOrgCountRespone> = Request::get("https://archive.org/advancedsearch.php?q=creator%3A(Lewis)&fl[]=creator&fl[]=identifier&fl[]=publicdate&rows=0&page=0&output=json&callback=")
            .send()
            .await
            .map_err(|err| logging::error!("Failed to load count: {:?}", err))
            .ok()?
            .json()
            .await
            .map_err(|err| logging::error!("Failed to parse count response: {:?}", err))
            .ok();

        // This API only allows to display up to 10000 results
        resp.map(|r| r.response.num_found.min(10000))
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sorting = sorting.clone();
    }
}

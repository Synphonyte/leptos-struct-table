use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, TimeZone};
use leptos::*;
use leptos_struct_table::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::Range;

#[derive(PartialEq, PartialOrd, Clone, Debug, Default)]
pub struct Link {
    text: String,
    href: String,
}

#[component]
pub fn ObjectLinkTableCellRenderer<F>(
    #[prop(into)] class: MaybeSignal<String>,
    #[prop(into)] value: MaybeSignal<Link>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(Link) + 'static,
{
    let link = format!(
        "https://archive.org/advancedsearch.php?q=identifier%3D{}&output=json&callback=",
        value.get_untracked().href,
    );
    view! { <td key=index class=class><a href=link>{value.get_untracked().text}</a></td> }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
#[serde(untagged)]
pub enum Authors {
    Single(String),
    Multiple(Vec<String>),
}

// we implement Display for Authors which gives use ToString as well. We'll use this for IntoView.
impl Display for Authors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authors::Single(author) => write!(f, "{}", author),
            Authors::Multiple(authors) => {
                write!(f, "{}", authors.join(", "))
            }
        }
    }
}

impl Default for Authors {
    fn default() -> Self {
        Self::Single("Unknown".to_string())
    }
}

// Anything that implements IntoView can be displayed by the default cell renderer.
impl IntoView for Authors {
    fn into_view(self) -> View {
        self.to_string().into_view()
    }
}

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(sortable)]
pub struct Book {
    #[serde(rename = "identifier")]
    #[table(key)]
    pub id: String,

    pub title: String,

    #[serde(rename = "creator")]
    pub author: Authors,

    #[serde(rename = "publicdate")]
    pub publish_date: String,

    #[serde(skip_deserializing)]
    #[table(skip)]
    pub hidden_field: String,

    #[serde(skip_deserializing)]
    #[table(title = "Link", renderer = "ObjectLinkTableCellRenderer")]
    pub link: FieldGetter<Link>,
}

impl Book {
    pub fn link(&self) -> Link {
        Link {
            text: self.title.clone(),
            href: self.id.clone(),
        }
    }
}

#[derive(Deserialize)]
struct ArchiveOrgApiResponse {
    pub response: ArchiveOrgApiResponseInner,
}

#[derive(Deserialize)]
pub struct ArchiveOrgApiResponseInner {
    pub docs: Vec<Book>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct BookDataProvider {
    reload_count: usize,
    sorting: VecDeque<(BookColumnName, ColumnSort)>,
}

impl Default for BookDataProvider {
    fn default() -> Self {
        Self {
            reload_count: 0,
            sorting: VecDeque::new(),
        }
    }
}

impl BookDataProvider {
    const ITEM_COUNT: usize = 20;

    fn url_sort_param_for_column(&self, column: BookColumnName) -> &'static str {
        match column {
            BookColumnName::Id => "identifierSorter",
            BookColumnName::Title => "titleSorter",
            BookColumnName::Author => "creatorSorter",
            BookColumnName::PublishDate => "publicdate",
            _ => "",
        }
    }

    fn url_sort_param_for_sort_pair(&self, pair: &(BookColumnName, ColumnSort)) -> String {
        let col = self.url_sort_param_for_column(pair.0);

        let dir = match pair.1 {
            ColumnSort::Ascending => "asc",
            ColumnSort::Descending => "desc",
            ColumnSort::None => return "".to_string(),
        };

        format!("&sort%5B%5D={}+{}", col, dir)
    }

    fn get_url(&self, range: Range<usize>) -> String {
        let mut sort = String::new();
        for pair in &self.sorting {
            sort.push_str(&self.url_sort_param_for_sort_pair(pair));
        }

        let len = ((range.end - range.start) / Self::ITEM_COUNT + 2) * Self::ITEM_COUNT;
        let page = range.start / len;

        let start = page * len;

        format!(
            "https://archive.org/advancedsearch.php?q=creator%3A%28Lewis%29&fl%5B%5D=creator&fl%5B%5D=identifier&fl%5B%5D=publicdate&fl%5B%5D=title{sort}&rows={}&page={}&output=json&callback=", 
            len,
            page + 1,
        )
    }

    pub fn reload(&mut self) {
        self.reload_count += 1;
    }
}

#[async_trait(?Send)]
impl TableDataProvider<Book> for BookDataProvider {
    type ColumnName = BookColumnName;

    async fn get_rows(&self, range: Range<usize>) -> Vec<Book> {
        let resp: ArchiveOrgApiResponse = Request::get(&self.get_url(range))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        resp.response.docs
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(Self::ColumnName, ColumnSort)>) {
        self.sorting = sorting.clone();
    }
}

#[component]
pub fn App() -> impl IntoView {
    let items = create_rw_signal(BookDataProvider::default());

    let refresh = move |_| {
        items.update(|items| items.reload());
    };

    view! {
        <button on:click=refresh>"Refresh"</button>
        <BookTable items=items />
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}

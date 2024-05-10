use crate::renderer::ObjectLinkTableCellRenderer;
use leptos::IntoView;
use leptos_struct_table::{FieldGetter, TableRow};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(PartialEq, PartialOrd, Clone, Debug, Default)]
pub struct Link {
    pub text: String,
    pub href: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
#[serde(untagged)]
pub enum Authors {
    Single(String),
    Multiple(Vec<String>),
}

// we implement Display for Authors which gives use ToString as well. We'll use this for CellValue.
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

// Anything that implements CellValue can be displayed by the default cell renderer.
impl leptos_struct_table::CellValue for Authors {
    type RenderOptions = ();
    
    fn render_value(self, _options: &Self::RenderOptions) -> impl IntoView {
        self.to_string()
    }
}

#[derive(TableRow, Serialize, Deserialize, Clone, Debug)]
// #[table(sortable)]
pub struct Book {
    #[serde(rename = "identifier")]
    #[table(skip)]
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

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum ArchiveOrgApiResponse {
    Ok {
        response: ArchiveOrgApiResponseInner,
    },
    Err {
        error: String,
    },
}

#[derive(Deserialize, Debug)]
pub struct ArchiveOrgApiResponseInner {
    pub docs: Vec<Book>,
}

#[derive(Deserialize, Debug)]
pub struct ArchiveOrgCountRespone {
    pub response: ArchiveOrgCountResponseInner,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveOrgCountResponseInner {
    pub num_found: usize,
}

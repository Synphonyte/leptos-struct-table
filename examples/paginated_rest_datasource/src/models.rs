use crate::renderer::ObjectLinkTableCellRenderer;
use leptos_struct_table::{FieldGetter, TableRow};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Clone, Debug, Default)]
pub struct Link {
    pub text: String,
    pub href: String,
}

#[derive(TableRow, Serialize, Deserialize, Clone, Debug)]
#[table(sortable)]
pub struct Brewery {
    #[table(skip)]
    pub id: String,

    pub name: String,

    pub city: String,

    pub country: String,

    #[table(skip)]
    pub website_url: Option<String>,

    #[serde(skip_deserializing)]
    #[table(title = "Link", renderer = "ObjectLinkTableCellRenderer")]
    pub link: FieldGetter<Link>,
}

impl Brewery {
    pub fn link(&self) -> Link {
        Link {
            text: self.website_url.clone().unwrap_or("".to_string()),
            href: self.website_url.clone().unwrap_or("".to_string()),
        }
    }
}
use async_trait::async_trait;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::Row;
use std::collections::VecDeque;
use std::ops::Range;

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[table(classes_provider = TailwindClassesPreset)]
pub struct Customer {
    pub customer_id: String,
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub city: String,
    pub country: String,
    pub phone: String,
    pub email: String,
    pub website: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerQuery {
    #[serde(default)]
    sort: VecDeque<(usize, ColumnSort)>,
    range: Range<usize>,
    name: String,
}

#[server]
pub async fn list_customers(query: CustomerQuery) -> Result<Vec<Customer>, ServerFnError<String>> {
    use crate::database::get_db;

    let CustomerQuery { sort, range, name } = query;

    let wher = if !name.is_empty() {
        format!("WHERE name ILIKE '%{name}%'")
    } else {
        "".to_owned()
    };

    let order: Vec<String> = sort
        .into_iter()
        .filter_map(|(col, col_sort)| {
            let col_sort = match col_sort {
                ColumnSort::Ascending => Some("ASC"),
                ColumnSort::Descending => Some("DESC"),
                ColumnSort::None => None,
            };

            col_sort.map(|col_sort| format!("{} {}", Customer::col_name(col), col_sort))
        })
        .collect();

    let mut order = order.join(", ");
    if !order.is_empty() {
        order = format!("ORDER BY {order}");
    }

    let sql = format!(
        r#"select customer_id, first_name, last_name, company, city, country, phone, email, website from customers {wher} {order} LIMIT $1 OFFSET $2"#
    );

    sqlx::query_as::<_, Customer>(&sql)
        .bind(range.len() as i64)
        .bind(range.start as i64)
        .fetch_all(get_db())
        .await
        .map_err(|e| ServerFnError::WrappedServerError(format!("{e:?}")))
}

#[server]
pub async fn customer_count() -> Result<usize, ServerFnError<String>> {
    use crate::database::get_db;

    let count: i64 = sqlx::query("SELECT COUNT(*) FROM customers")
        .fetch_one(get_db())
        .await
        .map_err(|err| ServerFnError::WrappedServerError(format!("{err:?}")))?
        .get(0);

    Ok(count as usize)
}

#[derive(Default)]
pub struct CustomerTableDataProvider {
    sort: VecDeque<(usize, ColumnSort)>,
    pub name: RwSignal<String>,
}

#[async_trait(? Send)]
impl TableDataProvider<Customer> for CustomerTableDataProvider {
    async fn get_rows(&self, range: Range<usize>) -> Result<(Vec<Customer>, Range<usize>), String> {
        list_customers(CustomerQuery {
            name: self.name.get_untracked(),
            sort: self.sort.clone(),
            range: range.clone(),
        })
        .await
        .map(|rows| {
            let len = rows.len();
            (rows, range.start..range.start + len)
        })
        .map_err(|e| format!("{e:?}"))
    }

    async fn row_count(&self) -> Option<usize> {
        customer_count().await.ok()
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sort = sorting.clone();
    }

    fn track(&self) {
        self.name.track();
    }
}

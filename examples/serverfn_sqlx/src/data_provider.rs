use crate::classes::ClassesPreset;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{QueryBuilder, Row};
use std::collections::VecDeque;
use std::ops::Range;

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[table(sortable, classes_provider = ClassesPreset)]
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

    let mut query = QueryBuilder::new("SELECT customer_id, first_name, last_name, company, city, country, phone, email, website FROM customers ");
    if !name.is_empty() {
        query.push("WHERE first_name LIKE concat('%', ");
        query.push_bind(&name);
        query.push(", '%') OR last_name LIKE concat('%', ");
        query.push_bind(&name);
        query.push(", '%') OR company LIKE concat('%', ");
        query.push_bind(&name);
        query.push(", '%') ");
    }

    if let Some(order) = Customer::sorting_to_sql(&sort) {
        query.push(order);
    }

    query.push(" LIMIT ");
    query.push_bind(range.len() as i64);
    query.push(" OFFSET ");
    query.push_bind(range.start as i64);

    query
        .build_query_as::<Customer>()
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

impl TableDataProvider<Customer> for CustomerTableDataProvider {
    async fn get_rows(&self, range: Range<usize>) -> Result<(Vec<Customer>, Range<usize>), String> {
        list_customers(CustomerQuery {
            name: self.name.get_untracked().trim().to_string(),
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

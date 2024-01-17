use crate::TableDataProvider;
use leptos::*;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct TableData<Provider, Row>
where
    Provider: TableDataProvider<Row> + 'static,
    Row: Debug + PartialEq,
{
    pub update_count: RwSignal<u16>,
    pub provider: StoredValue<Provider>,
    _marker: PhantomData<Row>,
}

impl<Provider, Row> TableData<Provider, Row>
where
    Provider: TableDataProvider<Row>,
    Row: Debug + PartialEq,
{
    pub fn reload(&self) {
        self.update_count
            .update(|count| *count = count.wrapping_add(1));
    }
}

impl<Provider, Row> From<Provider> for TableData<Provider, Row>
where
    Provider: TableDataProvider<Row> + 'static,
    Row: Debug + PartialEq,
{
    fn from(provider: Provider) -> Self {
        Self {
            update_count: create_rw_signal(0),
            provider: StoredValue::new(provider),
            _marker: PhantomData,
        }
    }
}

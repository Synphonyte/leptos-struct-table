use std::marker::PhantomData;

pub struct TableChangeEvent<Row, Col, T> {
    pub row_index: usize,
    pub col_index: usize,
    pub old_row: Row,
    pub column: Col,
    pub new_value: T,
}

pub struct ChangeEventHandler<Row, Col, T>(Option<Box<dyn Fn(TableChangeEvent<Row, Col, T>)>>);

impl<Row, Col, T> Default for ChangeEventHandler<Row, Col, T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<F, Row, Col, T> From<F> for ChangeEventHandler<Row, Col, T>
where
    F: Fn(TableChangeEvent<Row, Col, T>) + 'static,
{
    fn from(f: F) -> Self {
        Self(Some(Box::new(f)))
    }
}

impl<Row, Col, T> ChangeEventHandler<Row, Col, T> {
    pub fn call(&self, event: TableChangeEvent<Row, Col, T>) {
        if let Some(f) = &self.0 {
            f(event);
        }
    }
}

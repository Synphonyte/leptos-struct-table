/// The event provided to the `on_change` prop of the table component
pub struct TableChangeEvent<Row, Col, T> {
    /// The index of the table row that contains the cell that was changed. Starts at 0.
    pub row_index: usize,
    /// The index of the table column that contains the cell that was changed. Starts at 0.
    pub col_index: usize,
    /// The old value of the row that was changed. This is the struct for which the table component is generated.
    pub old_row: Row,
    /// The column enum variant of the column that was changed.
    pub column: Col,
    /// The new value of the row that was changed. This will be of a column value enum type that is
    /// generated with the table component.
    pub new_value: T,
}

/// New type wrapper of a closure that takes a `TableChangeEvent`. This allows the `on_change` prop
/// to be optional while being able to take a simple closure.
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

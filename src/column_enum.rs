/// Trait for the generated column enums
pub trait ColumnEnum {
    /// Returns the field name that is represented by the variant
    fn column_name(&self) -> &'static str;
}

//! Support for [uuid::Uuid] type.

use leptos::*;

impl crate::Value for uuid::Uuid {
    fn render_value(self) -> impl IntoView {
        view! {
            <>{self.to_string()}</>
        }
    }
}
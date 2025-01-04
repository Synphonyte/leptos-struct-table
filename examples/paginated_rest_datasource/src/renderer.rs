use crate::models::{Brewery, Link};
use leptos::prelude::*;

#[component]
#[allow(unused_variables)]
pub fn ObjectLinkTableCellRenderer(
    class: String,
    #[prop(into)] value: Signal<Link>,
    row: RwSignal<Brewery>,
    index: usize,
) -> impl IntoView {
    view! {
        <td class=class>
            <Show when=move || !value.get_untracked().href.is_empty() fallback=move || view! { <span>{value.get_untracked().text}</span> }>
                <a href=value.get_untracked().href target="_blank">{value.get_untracked().text}</a>
            </Show>
        </td>
    }
}

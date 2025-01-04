use crate::models::Link;
use leptos::prelude::*;

#[component]
#[allow(unused_variables)]
pub fn ObjectLinkTableCellRenderer<F>(
    class: String,
    #[prop(into)] value: Signal<Link>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(Link) + 'static,
{
    view! {
        <td class=class>
            <Show when=move || !value.get_untracked().href.is_empty() fallback=move || view! { <span>{value.get_untracked().text}</span> }>
                <a href=value.get_untracked().href target="_blank">{value.get_untracked().text}</a>
            </Show>
        </td>
    }
}

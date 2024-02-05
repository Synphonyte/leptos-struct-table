use crate::models::Link;
use leptos::*;

#[component]
#[allow(unused_variables)]
pub fn ObjectLinkTableCellRenderer<F>(
    class: String,
    #[prop(into)] value: MaybeSignal<Link>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(Link) + 'static,
{
    let link = format!(
        "https://archive.org/advancedsearch.php?q=identifier%3D{}&output=json&callback=",
        value.get_untracked().href,
    );
    view! {
        <td key=index class=class>
            <a href=link>{value.get_untracked().text}</a>
        </td>
    }
}

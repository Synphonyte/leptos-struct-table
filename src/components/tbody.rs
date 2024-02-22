use leptos::html::{AnyElement, Tbody};
use leptos::*;

/// Default tbody renderer. Please note that this is **NOT** a `#[component]`.
///
/// # Arguments
///
/// * `content` - The content of the renderer. It's like the children of this view.
/// * `class` - The class attribute that is passed to the root element
/// * `node_ref` - The `NodeRef` referencing the root tbody element.
///
/// This render function has to render exactly one root element.
#[allow(non_snake_case)]
pub fn DefaultTableBodyRenderer(
    content: Fragment,
    class: Signal<String>,
    node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
    let tbody_ref = create_node_ref::<Tbody>();
    tbody_ref.on_load(move |e| node_ref.load(&e.into_any()));

    view! { <tbody class=class node_ref=tbody_ref>{content}</tbody> }
}

use leptos::ev::MouseEvent;
use leptos::*;

#[derive(Debug)]
pub struct TableRowEvent<K: 'static> {
    pub key: K,
    pub index: usize,
    pub mouse_event: MouseEvent,
}

#[allow(unused_variables)]
#[component]
pub fn DefaultTableRowRenderer<K, F>(
    cx: Scope,
    #[prop(into)] class: MaybeSignal<String>,
    #[prop(optional)] is_head: bool,
    #[prop(into)] key: K,
    index: usize,
    #[prop(into)] selected: Signal<bool>,
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(TableRowEvent<K>) + 'static,
    K: Clone + 'static,
{
    view! { cx,
        <tr class=class on:click=move |mouse_event| on_click(TableRowEvent {
            key: key.clone(),
            index,
            mouse_event,
        })>
            {children(cx)}
        </tr>
    }
}

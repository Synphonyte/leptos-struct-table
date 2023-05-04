use crate::ColumnSort;
use leptos::ev::MouseEvent;
use leptos::*;

#[derive(Debug)]
pub struct TableHeadEvent<C: 'static> {
    pub index: usize,
    pub column: C,
    pub mouse_event: MouseEvent,
}

#[component]
pub fn DefaultTableHeaderRenderer<C, F>(
    cx: Scope,
    #[prop(into)] class: Signal<String>,
    #[prop(into)] inner_class: String,
    index: usize,
    column: C,
    #[prop(into)] sort_priority: Signal<Option<usize>>,
    #[prop(into)] sort_direction: Signal<ColumnSort>,
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(TableHeadEvent<C>) + 'static,
    C: 'static + Copy,
{
    let style = move || {
        let sort = match sort_direction() {
            ColumnSort::Ascending => "--sort-icon: '▲';",
            ColumnSort::Descending => "--sort-icon: '▼';",
            ColumnSort::None => "--sort-icon: '';",
        };

        let priority = match sort_priority() {
            Some(priority) => format!("--sort-priority: '{}';", priority + 1),
            None => "--sort-priority: '';".to_string(),
        };

        format!("{} {}", sort, &priority)
    };

    view! { cx,
        <th class=class
            on:click=move |mouse_event| on_click(TableHeadEvent {
                index,
                column,
                mouse_event,
            })
            style=style
        >
            <span class=inner_class>
                {children(cx)}
            </span>
        </th>
    }
}

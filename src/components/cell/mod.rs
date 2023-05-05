#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "chrono")]
pub use self::chrono::*;

use core::fmt::Display;
use leptos::*;

#[component]
pub fn DefaultTableCellRenderer<T>(
    cx: Scope,
    #[prop(into)] class: MaybeSignal<String>,
    #[prop(into)] value: MaybeSignal<T>,
) -> impl IntoView
where
    T: IntoView + Clone + 'static,
{
    view! { cx,
        <td class=class>{value}</td>
    }
}

#[component]
pub fn DefaultNumberTableCellRenderer<T>(
    cx: Scope,
    #[prop(into)] class: MaybeSignal<String>,
    #[prop(into)] value: MaybeSignal<T>,
    #[prop(optional)] precision: Option<usize>,
) -> impl IntoView
where
    T: Display + Clone + 'static,
{
    let text = create_memo(cx, move |_| match precision {
        Some(precision) => format!("{:.precision$}", value()),
        None => format!("{}", value()),
    });

    view! { cx,
        <td class=class>{text}</td>
    }
}

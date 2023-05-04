use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use core::fmt::Display;
use leptos::*;
use paste::paste;

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

macro_rules! date_cell_renderer {
    ($date_type:ident) => {
        paste! {
            #[component]
            pub fn [<Default $date_type TableCellRenderer>] (
                cx: Scope,
                #[prop(into)] class: MaybeSignal<String>,
                #[prop(into)] value: MaybeSignal<$date_type>,
                #[prop(optional)] format_string: Option<String>,
            ) -> impl IntoView {
                let text = match format_string {
                    Some(format_string) => create_memo(cx, move |_| value().format(&format_string).to_string()),
                    None => create_memo(cx, move |_| value().to_string()),
                };

                view! { cx,
                    <td class=class>{text}</td>
                }
            }
        }
    };
}

date_cell_renderer!(NaiveDate);
date_cell_renderer!(NaiveDateTime);
date_cell_renderer!(NaiveTime);

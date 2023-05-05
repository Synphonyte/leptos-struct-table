use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use leptos::*;
use paste::paste;

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

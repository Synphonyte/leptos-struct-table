#![allow(non_snake_case)]

mod components;

pub use components::*;

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use leptos::ev::MouseEvent;
use leptos::*;
use paste::paste;
use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use std::ops::{Range, RangeBounds};

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

// TODO : table head event

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


#[derive(Debug)]
pub struct TableHeadEvent<C: 'static> {
    pub index: usize,
    pub column: C,
    pub mouse_event: MouseEvent,
}

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColumnSort {
    Ascending,
    Descending,
    None,
}

pub trait TableClassesProvider: Clone {
    fn new() -> Self;

    fn table(&self, classes: &str) -> String {
        classes.to_string()
    }
    fn head_row(&self, template_classes: &str) -> String {
        template_classes.to_string()
    }
    fn head_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        template_classes.to_string()
    }
    fn head_cell_inner(&self) -> String {
        "".to_string()
    }
    fn row(&self, _row_index: usize, selected: bool, template_classes: &str) -> String {
        template_classes.to_string() + if selected { " selected" } else { "" }
    }
    fn cell(&self, template_classes: &str) -> String {
        template_classes.to_string()
    }
}

#[derive(Clone, Copy)]
pub struct TailwindClassesPreset;

impl TableClassesProvider for TailwindClassesPreset {
    fn new() -> Self {
        Self
    }

    fn table(&self, classes: &str) -> String {
        format!(
            "{} {}",
            "text-sm text-left text-gray-500 dark:text-gray-400", classes
        )
    }

    fn head_row(&self, template_classes: &str) -> String {
        format!(
            "{} {}",
            "text-xs text-gray-700 uppercase bg-gray-200 dark:bg-gray-700 dark:text-gray-400",
            template_classes
        )
    }

    fn head_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        let sort_class = match sort {
            ColumnSort::None => "",
            _ => "text-black dark:text-white",
        };

        format!(
            "cursor-pointer px-5 py-2 {} {}",
            sort_class, template_classes
        )
    }

    fn head_cell_inner(&self) -> String {
        "flex items-center after:content-[--sort-icon] after:pl-1 after:opacity-40 before:content-[--sort-priority] before:order-last before:pl-0.5 before:font-light before:opacity-40".to_string()
    }

    fn row(&self, row_index: usize, selected: bool, template_classes: &str) -> String {
        let bg_color = if row_index % 2 == 0 {
            if selected {
                "bg-sky-300 text-gray-700 dark:bg-sky-700 dark:text-gray-400"
            } else {
                "bg-white dark:bg-gray-900 hover:bg-gray-100 dark:hover:bg-gray-800"
            }
        } else {
            if selected {
                "bg-sky-300 text-gray-700 dark:bg-sky-700 dark:text-gray-400"
            } else {
                "bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700"
            }
        };

        format!(
            "{} {} {}",
            "border-b dark:border-gray-700", bg_color, template_classes
        )
    }

    fn cell(&self, template_classes: &str) -> String {
        format!("{} {}", "px-5 py-2", template_classes)
    }
}

/// The trait that provides data for the generated table component.
/// Anything that is passed to the `items` prop must implement this trait.
///
/// This is automatically implemented for `Vec<T>`.
/// This way a simple list of items can be passed to the table.
#[async_trait(?Send)]
pub trait TableDataProvider<T>
where
    T: Debug + PartialEq,
{
    type ColumnName: Copy;

    /// Get all data rows for the table specified by the range. This method is called when the table is rendered.
    /// The range is determined by the visible rows and used to virtualize the table.
    /// The parameter `range` is only determined by visibility and may be out of bounds. It is the
    /// responsibility of the implementation to handle this case.
    async fn get_rows(&self, range: Range<usize>) -> Vec<T>;

    /// Set the sorting of the table. The sorting is a list of column names and the sort order sorted by priority.
    /// The first entry in the list is the most important one.
    /// The default implementation does nothing.
    /// For example: `[(Column::Name, ColumnSort::Ascending), (Column::Age, ColumnSort::Descending)]`
    /// will sort by name first and then by age.
    /// Please note that after calling this method, data will be reloaded through [`get_rows`].
    fn set_sorting(&mut self, _sorting: &VecDeque<(Self::ColumnName, ColumnSort)>) {
        // by default do nothing
    }
}

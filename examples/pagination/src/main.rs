mod data_provider;
mod models;
mod tailwind;

use crate::data_provider::BookDataProvider;
use leptos::*;
use leptos_struct_table::*;

#[component]
pub fn App() -> impl IntoView {
    let rows = BookDataProvider::default();

    let pagination_controller = PaginationController::default();

    view! {
        <div class="rounded-md overflow-clip m-10 border dark:border-gray-700".to_string()>
            <table class="text-sm text-left text-gray-500 dark:text-gray-400 mb-[-1px] w-[calc(100vw-5rem)]">
                <TableContent
                    rows=rows
                    display_strategy=DisplayStrategy::Pagination {
                        controller: pagination_controller,
                        row_count: 10,
                    }
                />

            </table>
        </div>

        <Paginator pagination_controller />
    }
}

#[component]
pub fn Paginator(pagination_controller: PaginationController) -> impl IntoView {
    let current_page = pagination_controller.current_page;
    let page_count = pagination_controller.page_count();

    let page_range = move || {
        let mut start = current_page.get().saturating_sub(2);

        let mut end = start + 5;

        if let Some(row_count) = page_count.get() {
            if end > row_count {
                end = row_count;
                start = end.saturating_sub(5);
            }
        }

        start..end
    };

    view! {
        <nav aria-label="Page navigation example" class="m-10 flex justify-end">
            <ul class="inline-flex -space-x-px text-sm">
                <li>
                    <a
                        href="#"
                        class="flex items-center justify-center px-3 h-8 ms-0 leading-tight text-gray-500 bg-white border border-e-0 border-gray-300 rounded-s-lg hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
                        on:click=move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            pagination_controller.previous();
                        }
                    >

                        Previous
                    </a>
                </li>

                <For each=page_range key=|page| *page let:page>
                    <PageLink page pagination_controller />
                </For>

                <li>
                    <a
                        href="#"
                        class="flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 rounded-e-lg hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
                        on:click=move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            pagination_controller.next();
                        }
                    >

                        Next
                    </a>
                </li>
            </ul>
        </nav>
    }
}

#[component]
pub fn PageLink(page: usize, pagination_controller: PaginationController) -> impl IntoView {
    let is_selected = move || pagination_controller.current_page.get() == page;

    let class = move || {
        if is_selected() {
            "flex items-center justify-center px-3 h-8 text-blue-600 border border-gray-300 bg-blue-50 hover:bg-blue-100 hover:text-blue-700 dark:border-gray-700 dark:bg-gray-700 dark:text-white"
        } else {
            "flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
        }
    };

    view! {
        <li>
            <a
                href="#"
                class=class
                on:click=move |evt| {
                    evt.prevent_default();
                    evt.stop_propagation();
                    pagination_controller.current_page.set(page);
                }
            >

                {page + 1}
            </a>
        </li>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App/> }
    })
}

mod data_provider;
mod models;
mod renderer;

use crate::data_provider::BookDataProvider;
use leptos::html::Div;
use leptos::*;
use leptos_struct_table::*;
use leptos_use::use_debounce_fn_with_arg;

#[component]
pub fn App() -> impl IntoView {
    let rows = BookDataProvider::default();

    let reload_controller = ReloadController::default();

    let reload = move |_| {
        reload_controller.reload();
    };

    let container = create_node_ref::<Div>();

    let (count, set_count) = create_signal(0);

    let on_input = use_debounce_fn_with_arg(move |value| rows.search.set(value), 300.0);

    view! {
        <div class="container">
            <div class="top-bar">
                <button on:click=reload>"Reload"</button>
                <div id="search">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                        // !Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.
                        <path d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"/>
                    </svg>
                    <input
                        type="search"
                        placeholder="Search"
                        on:input=move |e| { on_input(event_target_value(&e)); }
                        value=rows.search
                    />
                </div>
                <Show when=move || { count.get() > 0 }>
                    <div>"Found " {count} " results"</div>
                </Show>
            </div>
            <div class="table-container" node_ref=container>
                <table>
                    <TableContent
                        rows=rows
                        scroll_container=container
                        loading_cell_inner_class="loading-skeleton"
                        reload_controller=reload_controller
                        on_row_count=move |count| set_count.set(count)
                    />
                </table>
            </div>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App/> }
    })
}

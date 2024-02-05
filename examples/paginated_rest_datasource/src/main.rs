mod data_provider;
mod models;
mod renderer;

use crate::data_provider::BookDataProvider;
use leptos::html::Div;
use leptos::*;
use leptos_struct_table::*;

#[component]
pub fn App() -> impl IntoView {
    let rows = BookDataProvider::default();

    let reload_controller = ReloadController::default();

    let reload = move |_| {
        reload_controller.reload();
    };

    let container = create_node_ref::<Div>();

    let (count, set_count) = create_signal(0);

    view! {
        <div class="container">
            <div class="top-bar">
                <button on:click=reload>"Reload"</button>
                <Show when=move || { count() > 0 }>
                    <div>"Found " {count} " results"</div>
                </Show>
            </div>
            <div class="table-container" node_ref=container>
                <table>
                    <TableContent
                        rows=rows
                        scroll_container=container
                        loading_row_inner_class="loading-skeleton"
                        reload_controller=reload_controller
                        on_row_count=set_count
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

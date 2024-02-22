use crate::data_provider::CustomerTableDataProvider;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::html::Div;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_struct_table::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/serverfn-sqlx.css"/>

        // sets the document title
        <Title text="Welcome to Leptos Struct Table"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let scroll_container = create_node_ref::<Div>();

    let rows = CustomerTableDataProvider::default();

    let name = rows.name;

    view! {
        <div class="flex flex-col h-[100vh] bg-white">
            <div class="border-b bg-slate-100 px-5 py-2">
                <label class="relative block">
                    <span class="absolute inset-y-0 left-0 flex items-center pl-3">
                        <svg
                            class="h-5 w-5 fill-black"
                            xmlns="http://www.w3.org/2000/svg"
                            x="0px"
                            y="0px"
                            width="30"
                            height="30"
                            viewBox="0 0 30 30"
                        >
                            <path d="M 13 3 C 7.4889971 3 3 7.4889971 3 13 C 3 18.511003 7.4889971 23 13 23 C 15.396508 23 17.597385 22.148986 19.322266 20.736328 L 25.292969 26.707031 A 1.0001 1.0001 0 1 0 26.707031 25.292969 L 20.736328 19.322266 C 22.148986 17.597385 23 15.396508 23 13 C 23 7.4889971 18.511003 3 13 3 z M 13 5 C 17.430123 5 21 8.5698774 21 13 C 21 17.430123 17.430123 21 13 21 C 8.5698774 21 5 17.430123 5 13 C 5 8.5698774 8.5698774 5 13 5 z"></path>
                        </svg>
                    </span>
                    <input
                        class="w-full bg-white placeholder:font-italitc border border-slate-300 rounded-full py-2 pl-10 pr-4 focus:outline-none"
                        placeholder="Search by name or company"
                        type="text"
                        value=name
                        on:change=move |e| name.set(event_target_value(&e))
                    />
                </label>
            </div>
            <div node_ref=scroll_container class="overflow-auto grow min-h-0">
                <table class="table-fixed text-sm text-left text-gray-500 dark:text-gray-400 w-full">
                    <TableContent
                        rows
                        scroll_container
                    />
                </table>
            </div>
        </div>
    }
}

mod cell;
mod renderer_fn;
mod row;
mod table_content;
mod tbody;
mod thead;

pub use cell::*;
pub use row::*;
pub use table_content::*;
pub use tbody::*;
pub use thead::*;

#[macro_export]
macro_rules! wrapper_render_fn {
    (
        #[$doc_name:meta]
        $name:ident,
        $tag:ident,
        $(#[$additional_doc:meta])*
    ) => {
        /// Default
        #[$doc_name]
        /// renderer. Please note that this is **NOT** a `#[component]`.
        ///
        /// # Arguments
        ///
        /// * `content` - The content of the renderer. It's like the children of this view.
        /// * `class` - The class attribute that is passed to the root element
        $(#[$additional_doc])*
        #[allow(non_snake_case)]
        pub fn $name(content: View, class: Signal<String>) -> impl IntoView {
            view! {
                <$tag class=class>
                    {content}
                </$tag>
            }
        }
    };
}

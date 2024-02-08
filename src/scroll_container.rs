use leptos::html::ElementDescriptor;
use leptos::{NodeRef, Signal};
use leptos_use::core::ElementMaybeSignal;
use leptos_use::use_document;
use wasm_bindgen::JsCast;

#[derive(Copy, Clone, Debug)]
pub struct ScrollContainer(Signal<Option<web_sys::Element>>);

impl Default for ScrollContainer {
    fn default() -> Self {
        Self(Signal::derive(move || {
            use_document()
                .body()
                .as_ref()
                .map(|w| w.unchecked_ref::<web_sys::Element>().clone())
        }))
    }
}

impl From<web_sys::Element> for ScrollContainer {
    fn from(element: web_sys::Element) -> Self {
        Self(Signal::derive(move || Some(element.clone())))
    }
}

impl From<Option<web_sys::Element>> for ScrollContainer {
    fn from(element: Option<web_sys::Element>) -> Self {
        Self(Signal::derive(move || element.clone()))
    }
}

impl<T> From<NodeRef<T>> for ScrollContainer
where
    T: ElementDescriptor + Clone + 'static,
{
    fn from(node_ref: NodeRef<T>) -> Self {
        Self(Signal::derive(move || {
            node_ref.get().map(|el| {
                let el: &web_sys::Element = &el.into_any();
                el.clone()
            })
        }))
    }
}

impl From<&str> for ScrollContainer {
    fn from(selector: &str) -> Self {
        let selector = selector.to_owned();

        Self(Signal::derive(move || {
            use_document().query_selector(&selector).unwrap_or_default()
        }))
    }
}

impl From<ScrollContainer> for ElementMaybeSignal<web_sys::Element, web_sys::Element> {
    fn from(scroll_container: ScrollContainer) -> Self {
        scroll_container.0.into()
    }
}

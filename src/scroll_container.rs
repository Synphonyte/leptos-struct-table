use leptos::html::ElementType;
use leptos::prelude::*;
use leptos_use::core::{ElementMaybeSignalType, IntoElementMaybeSignalType};
use leptos_use::use_document;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

#[derive(Copy, Clone, Debug)]
pub struct ScrollContainer(Signal<Option<SendWrapper<web_sys::Element>>>);

impl Default for ScrollContainer {
    fn default() -> Self {
        Self(Signal::derive(move || {
            use_document()
                .body()
                .as_ref()
                .map(|w| SendWrapper::new(w.unchecked_ref::<web_sys::Element>().clone()))
        }))
    }
}

impl From<web_sys::Element> for ScrollContainer {
    fn from(element: web_sys::Element) -> Self {
        let wrapped = SendWrapper::new(element);
        Self(Signal::derive(move || Some(wrapped.clone())))
    }
}

impl From<Option<web_sys::Element>> for ScrollContainer {
    fn from(element: Option<web_sys::Element>) -> Self {
        let wrapped = element.map(SendWrapper::new);
        Self(Signal::derive(move || wrapped.clone()))
    }
}

impl<E> From<NodeRef<E>> for ScrollContainer
where
    E: ElementType,
    E::Output: JsCast + Clone + 'static,
{
    fn from(node_ref: NodeRef<E>) -> Self {
        let wrapped = node_ref
            .get()
            .map(|e| SendWrapper::new(e.unchecked_ref::<web_sys::Element>().clone()));
        Self(Signal::derive(move || wrapped.clone()))
    }
}

impl From<&str> for ScrollContainer {
    fn from(selector: &str) -> Self {
        let selector = selector.to_owned();

        Self(Signal::derive(move || {
            use_document()
                .query_selector(&selector)
                .unwrap_or_default()
                .map(SendWrapper::new)
        }))
    }
}

impl IntoElementMaybeSignalType<web_sys::Element, Option<web_sys::Element>> for ScrollContainer {
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<web_sys::Element> {
        let value = self.0.get().map(|w| w.take());
        ElementMaybeSignalType::Static(StoredValue::new_local(value))
    }
}

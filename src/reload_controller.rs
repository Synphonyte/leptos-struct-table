use leptos::*;

/// You can pass this to a [`TableContent`] component's `reload_controller` prop to trigger a reload.
///
/// See the [paginated_rest_datasource example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/paginated_rest_datasource/src/main.rs)
/// for how to use.
#[derive(Copy, Clone)]
pub struct ReloadController(Trigger);

impl Default for ReloadController {
    fn default() -> Self {
        Self(create_trigger())
    }
}

impl ReloadController {
    pub fn reload(&self) {
        self.0.notify();
    }

    pub fn track(&self) {
        self.0.track();
    }
}

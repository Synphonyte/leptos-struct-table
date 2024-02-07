use leptos::*;

/// You can pass this to a [`TableContent`] component's `reload_controller` prop to trigger a reload.
///
/// See the [paginated_rest_datasource example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/paginated_rest_datasource/src/main.rs)
/// for how to use.
#[derive(Copy, Clone)]
pub struct ReloadController(RwSignal<()>);

impl Default for ReloadController {
    fn default() -> Self {
        Self(create_rw_signal(()))
    }
}

impl ReloadController {
    pub fn reload(&self) {
        self.0.set(());
    }
}

impl SignalGet for ReloadController {
    type Value = ();

    fn get(&self) -> () {
        self.0.get()
    }

    fn try_get(&self) -> Option<Self::Value> {
        self.0.try_get()
    }
}

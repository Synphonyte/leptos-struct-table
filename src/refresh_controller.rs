use leptos::prelude::*;

/// You can pass this to a [`TableContent`] component's `refresh_controller` prop to trigger a refresh of calculated values.
#[derive(Copy, Clone)]
pub struct RefreshController(Trigger);

impl Default for RefreshController {
    fn default() -> Self {
        Self(Trigger::default())
    }
}

impl RefreshController {
    pub fn refresh(&self) {
        self.0.notify();
    }

    pub fn track(&self) {
        self.0.track();
    }
}

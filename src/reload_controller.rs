use leptos::*;

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

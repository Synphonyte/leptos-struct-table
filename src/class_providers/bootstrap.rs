use crate::TableClassesProvider;

#[derive(Clone, Copy)]
pub struct BootstrapClassesPreset;

impl TableClassesProvider for BootstrapClassesPreset {
    fn new() -> Self {
        Self
    }

    fn row(&self, _: usize, selected: bool, template_classes: &str) -> String {
        let active = if selected { "table-active" } else { "" };

        format!("{} {}", active, template_classes)
    }

    // TODO : skeleton loading
}

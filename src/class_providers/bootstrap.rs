use crate::TableClassesProvider;

#[derive(Clone, Copy)]
pub struct BootstrapClassesPreset;

impl TableClassesProvider for BootstrapClassesPreset {
    fn new() -> Self {
        Self
    }

    fn table(&self, classes: &str) -> String {
        format!("{} {}", "table table-striped", classes)
    }

    fn row(&self, _: usize, selected: bool, template_classes: &str) -> String {
        let active = if selected { "table-active" } else { "" };

        format!("{} {}", active, template_classes)
    }
}

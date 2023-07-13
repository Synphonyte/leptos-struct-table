use crate::{ColumnSort, TableClassesProvider};

#[derive(Clone, Copy)]
pub struct TailwindClassesPreset;

impl TableClassesProvider for TailwindClassesPreset {
    fn new() -> Self {
        Self
    }

    fn table(&self, classes: &str) -> String {
        format!(
            "{} {}",
            "text-sm text-left text-gray-500 dark:text-gray-400", classes
        )
    }

    fn head_row(&self, template_classes: &str) -> String {
        format!(
            "{} {}",
            "text-xs text-gray-700 uppercase bg-gray-200 dark:bg-gray-700 dark:text-gray-300",
            template_classes
        )
    }

    fn head_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        let sort_class = match sort {
            ColumnSort::None => "",
            _ => "text-black dark:text-white",
        };

        format!(
            "cursor-pointer px-5 py-2 {} {}",
            sort_class, template_classes
        )
    }

    fn head_cell_inner(&self) -> String {
        "flex items-center after:content-[--sort-icon] after:pl-1 after:opacity-40 before:content-[--sort-priority] before:order-last before:pl-0.5 before:font-light before:opacity-40".to_string()
    }

    fn row(&self, row_index: usize, selected: bool, template_classes: &str) -> String {
        let bg_color = if row_index % 2 == 0 {
            if selected {
                "bg-sky-300 text-gray-700 dark:bg-sky-700 dark:text-gray-400"
            } else {
                "bg-white dark:bg-gray-900 hover:bg-gray-100 dark:hover:bg-gray-800"
            }
        } else if selected {
            "bg-sky-300 text-gray-700 dark:bg-sky-700 dark:text-gray-400"
        } else {
            "bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700"
        };

        format!(
            "{} {} {}",
            "border-b dark:border-gray-700", bg_color, template_classes
        )
    }

    fn cell(&self, template_classes: &str) -> String {
        format!("{} {}", "px-5 py-2", template_classes)
    }
}

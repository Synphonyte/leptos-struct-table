use ::chrono::NaiveDate;
use leptos::*;
use leptos_struct_table::*;

// This generates the component BookTable
#[derive(TableRow, Clone)]
#[table(
    sortable,
    classes_provider = "BootstrapClassesPreset",
    impl_vec_data_provider
)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub publish_date: NaiveDate,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let rows = vec![
            Book {
                id: 1,
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
                publish_date: NaiveDate::from_ymd_opt(1925, 4, 10).unwrap(),
            },
            Book {
                id: 2,
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: NaiveDate::from_ymd_opt(1939, 4, 14).unwrap(),
            },
            Book {
                id: 3,
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: NaiveDate::from_ymd_opt(1949, 6, 8).unwrap(),
            },
            Book {
                id: 4,
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: NaiveDate::from_ymd_opt(1922, 2, 2).unwrap(),
            },
        ];

        view! {
            <div class="container".to_string()>
                <table class="table-hover">
                    <TableContent rows />
                </table>
            </div>
        }
    })
}

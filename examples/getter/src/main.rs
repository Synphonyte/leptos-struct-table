use ::chrono::NaiveDate;
use leptos::*;
use leptos_struct_table::*;

// This generates the component BookTable
#[derive(TableRow, Clone)]
#[table(sortable, impl_vec_data_provider)]
pub struct Book {
    pub id: u32,
    pub title: String,

    // instead of accessing `item.publish_date` directly, we use a getter `item.get_publish_date()`
    #[table(getter = "get_publish_date")]
    pub publish_date: NaiveDate,

    #[table(skip)]
    pub author: Author,

    // specified that there is a getter method `author_name()`
    pub author_name: FieldGetter<String>,
}

impl Book {
    // if no otherwise specified the getter method should have the same name as the `FieldGetter` field
    pub fn author_name(&self) -> String {
        format!("{} {}", self.author.first_name, self.author.last_name)
    }

    // getter for publish date
    pub fn get_publish_date(&self) -> NaiveDate {
        // do sth...
        self.publish_date
    }
}

#[derive(Clone)]
pub struct Author {
    pub first_name: String,
    pub last_name: String,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let rows = vec![
            Book {
                id: 1,
                title: "The Great Gatsby".to_string(),
                author: Author {
                    first_name: "F. Scott".to_string(),
                    last_name: "Fitzgerald".to_string(),
                },
                publish_date: NaiveDate::from_ymd_opt(1925, 4, 10).unwrap(),
                author_name: Default::default(),
            },
            Book {
                id: 2,
                title: "The Grapes of Wrath".to_string(),
                author: Author {
                    first_name: "John".to_string(),
                    last_name: "Steinbeck".to_string(),
                },
                publish_date: NaiveDate::from_ymd_opt(1939, 4, 14).unwrap(),
                author_name: Default::default(),
            },
            Book {
                id: 3,
                title: "Nineteen Eighty-Four".to_string(),
                author: Author {
                    first_name: "George".to_string(),
                    last_name: "Orwell".to_string(),
                },
                publish_date: NaiveDate::from_ymd_opt(1949, 6, 8).unwrap(),
                author_name: Default::default(),
            },
            Book {
                id: 4,
                title: "Ulysses".to_string(),
                author: Author {
                    first_name: "James".to_string(),
                    last_name: "Joyce".to_string(),
                },
                publish_date: NaiveDate::from_ymd_opt(1922, 2, 2).unwrap(),
                author_name: Default::default(),
            },
        ];

        view! {
            <table>
                <TableContent rows />
            </table>
        }
    })
}

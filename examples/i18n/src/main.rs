use ::chrono::NaiveDate;
use ::time::Date;
use ::uuid::Uuid;
use i18n::*;
use leptos::*;
use leptos_struct_table::*;

leptos_i18n::load_locales!();

/// This generates the component BookTable
#[derive(TableRow, Clone)]
#[table(sortable, impl_vec_data_provider)]
pub struct Book {
    /// Id of the entry.
    #[table(i18n(skip))]
    pub id: Uuid,
    /// Title of the book.
    pub title: String,
    /// Author of the book.
    pub author: String,
    /// Date when book has been published.
    pub publish_date: Option<NaiveDate>,
    /// Date when book was read
    pub read_date: Option<Date>,
    /// Description of the book. Optional.
    #[table(none_value = "-", i18n(key = description))]
    pub desc: Option<String>,
    /// Example on hidden member.
    #[table(skip)]
    pub hidden_field: String,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let rows = vec![
            Book {
                id: Uuid::new_v4(),
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1925, 4, 10).unwrap()),
                read_date: Some(Date::from_calendar_date(2024, ::time::Month::January, 2).unwrap()),
                desc: Some(
                    "A story of wealth, love, and the American Dream in the 1920s.".to_string(),
                ),
                hidden_field: "hidden".to_string(),
            },
            Book {
                id: Uuid::new_v4(),
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1939, 4, 14).unwrap()),
                read_date: None,
                desc: None,
                hidden_field: "not visible in the table".to_string(),
            },
            Book {
                id: Uuid::new_v4(),
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1949, 6, 8).unwrap()),
                read_date: None,
                desc: None,
                hidden_field: "hidden".to_string(),
            },
            Book {
                id: Uuid::new_v4(),
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1922, 2, 2).unwrap()),
                read_date: None,
                desc: None,
                hidden_field: "hidden".to_string(),
            },
        ];

        let i18n = provide_i18n_context();

        let on_switch = move |_| {
            let new_lang = match i18n.get_locale() {
                Locale::en => Locale::de,
                Locale::de => Locale::en,
            };
            i18n.set_locale(new_lang);
        };

        view! {
            <button on:click=on_switch>{t!(i18n, click_to_change_lang)}</button>
            <table>
                <TableContent rows=rows />
            </table>
        }
    })
}

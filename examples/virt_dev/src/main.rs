use async_trait::async_trait;
use core::ops::Range;
use leptos::html::Tbody;
use leptos::*;
use leptos_struct_table::*;
use leptos_use::{
    use_element_size_with_options, use_scroll_with_options, UseElementSizeOptions,
    UseElementSizeReturn, UseScrollOptions, UseScrollReturn,
};
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, TableComponent)]
pub struct Book {
    #[table(key)]
    pub id: String,
    pub title: String,
}

pub struct LoadedRows<T: Clone> {
    rows: Vec<Option<T>>,
}

impl<T: Clone> LoadedRows<T> {
    pub fn new() -> Self {
        Self { rows: vec![] }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    #[inline]
    pub fn resize(&mut self, len: usize) {
        self.rows.resize(len, None);
    }

    pub fn splice(&mut self, range: Range<usize>, rows: &[T]) {
        if range.end > self.rows.len() {
            self.rows.resize(range.end, None);
        }

        self.rows.splice(range, rows.into_iter().cloned().map(Some));
    }

    #[inline]
    pub fn range_loaded(&self, range: Range<usize>) -> bool {
        self.rows[range].iter().all(Option::is_some)
    }
}

impl<T: Clone> Index<Range<usize>> for LoadedRows<T> {
    type Output = [Option<T>];

    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.rows[index]
    }
}

fn read_csv() -> Vec<Book> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(include_bytes!("jumbo_public_hotels.csv").as_slice());

    let mut result = vec![];

    for r in rdr.deserialize() {
        let book: Book = r.unwrap();
        result.push(book);
    }

    result
}

#[component]
pub fn App() -> impl IntoView {
    let data_source = read_csv();

    let loaded_rows = create_rw_signal(LoadedRows::<Book>::new());

    let tbody_el = create_node_ref::<Tbody>();

    let UseScrollReturn { y, .. } =
        use_scroll_with_options(tbody_el, UseScrollOptions::default().throttle(100.0));

    let UseElementSizeReturn { height, .. } = use_element_size_with_options(
        tbody_el,
        UseElementSizeOptions::default().box_(web_sys::ResizeObserverBoxOptions::ContentBox),
    );

    let (row_count, set_row_count) = create_signal(None::<usize>);
    spawn_local({
        let data_source = data_source.clone();

        async move {
            let row_count = data_source.row_count().await;

            set_row_count(row_count);

            if let Some(row_count) = row_count {
                loaded_rows.update(|loaded_rows| loaded_rows.resize(row_count));
            }
        }
    });

    let (average_row_height, set_average_row_height) = create_signal(22.0);

    let first_visible_row_index =
        create_memo(move |_| (y.get() / average_row_height.get()).floor() as usize);
    let visible_row_count =
        create_memo(move |_| (height.get() / average_row_height.get()).ceil() as usize);

    let (display_range, set_display_range) = create_signal(0..0);

    let placeholder_height_before =
        create_memo(move |_| display_range.get().start as f64 * average_row_height.get());
    let placeholder_height_after = create_memo(move |_| {
        let row_count_after = if let Some(row_count) = row_count.get() {
            (row_count - display_range.get().end) as f64
        } else {
            0.0
        };

        row_count_after * average_row_height.get()
    });

    let compute_average_row_height = move || {
        if let Some(el) = tbody_el.get_untracked() {
            let el: &web_sys::Element = &el;
            let end_index = display_range.get_untracked().end;
            if end_index > 0 {
                set_average_row_height.set(
                    (el.scroll_height() as f64 - placeholder_height_after.get_untracked())
                        / end_index as f64,
                );
                logging::log!("el.scroll_height: {}", el.scroll_height());
                logging::log!("average row height: {}", average_row_height.get_untracked());
                logging::log!("display_range: {:?}", display_range.get_untracked());
            }
        }
    };

    create_effect(move |_| {
        let first_visible_row_index = first_visible_row_index.get();
        let visible_row_count = visible_row_count.get();

        let start = first_visible_row_index.saturating_sub(visible_row_count * 2);
        let mut end = first_visible_row_index + visible_row_count * 3;

        if let Some(count) = row_count.get() {
            end = end.min(count);
        }

        loaded_rows.update_untracked(|loaded_rows| {
            if end > loaded_rows.len() {
                loaded_rows.resize(end);
            }
        });

        let range = start..end;
        set_display_range.set(range.clone());

        let loaded =
            loaded_rows.with_untracked(|loaded_rows| loaded_rows.range_loaded(range.clone()));
        if !loaded {
            spawn_local({
                let data_source = data_source.clone();

                async move {
                    if let Ok((rows, loaded_range)) = data_source.get_rows(range.clone()).await {
                        if loaded_range.end < range.end {
                            set_row_count(Some(loaded_range.end));
                        }

                        loaded_rows.update(|loaded_rows| loaded_rows.splice(loaded_range, &rows));
                        compute_average_row_height();
                    } else {
                        // TODO : Error handling
                    }
                }
            });
        }
    });

    view! {
        <table>
            <thead>
                <tr>
                    <th>Id</th>
                    <th>Title</th>
                </tr>
            </thead>
            <tbody node_ref=tbody_el>
                <div style:height=move || format!("{}px", placeholder_height_before.get()) style="background: green;"></div>

                <For
                    each=move || {
                        loaded_rows
                            .with(move |loaded_rows| {
                                loaded_rows[display_range.get()]
                                    .iter()
                                    .cloned()
                                    .enumerate()
                                    .collect::<Vec<_>>()
                            })
                    }

                    key=|(idx, row)| {
                        row.as_ref().map(|row| row.id.clone()).unwrap_or(format!("empty-{idx}"))
                    }
                    let:row
                >
                    <tr>
                        {move || {
                            if let (_, Some(row)) = &row {
                                view! {
                                    <td>{row.id.clone()}</td>
                                    <td>{row.title.clone()}</td>
                                }
                                    .into_view()
                            } else {
                                logging::log!("empty");
                                view! { <td colspan="2" style="background: silver;">"Loading...!"</td> }.into_view()
                            }
                        }}

                    </tr>
                </For>

                <div style:height=move || format!("{}px", placeholder_height_after.get()) style="background: red;"></div>
            </tbody>
        </table>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App/> }
    })
}

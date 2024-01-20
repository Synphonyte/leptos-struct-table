use crate::components::renderer_fn::renderer_fn;
use crate::loaded_rows::{LoadedRows, RowState};
use crate::row_renderer::RowRenderer;
use crate::{
    ChangeEventHandler, ColumnSort, DefaultErrorRowRenderer, DefaultLoadingRowRenderer,
    DefaultRowPlaceholderRenderer, DefaultTableBodyRenderer, DefaultTableHeadRenderer,
    DefaultTableHeadRowRenderer, DefaultTableRowRenderer, TableClassesProvider, TableDataProvider,
    TableHeadEvent,
};
use leptos::html::{AnyElement, ElementDescriptor};
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_use::{
    use_element_size_with_options, use_scroll_with_options, UseElementSizeOptions,
    UseElementSizeReturn, UseScrollOptions, UseScrollReturn,
};
use std::collections::VecDeque;
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;

renderer_fn!(
    RowRendererFn<Row>(
        class: Signal<String>,
        row: Row,
        index: usize,
        selected: RwSignal<bool>,
        on_change: ChangeEventHandler<Row>
    )
    default DefaultTableRowRenderer
    where Row: RowRenderer + Clone + 'static
);

renderer_fn!(
    RowPlaceholderRendererFn(height: Signal<f64>)
    default DefaultRowPlaceholderRenderer
);

renderer_fn!(
    WrapperRendererFn(view: View, class: Signal<String>)
);

renderer_fn!(
    ErrorRowRendererFn(err: String)
    default DefaultErrorRowRenderer
);

renderer_fn!(
    LoadingRowRendererFn(col_count: usize, inner_class: Signal<String>)
    default DefaultLoadingRowRenderer
);

#[component]
pub fn TableContent<Row, DataP, ClsP>(
    rows: DataP,
    #[prop(optional, into)] on_change: ChangeEventHandler<Row>,
    #[prop(default = create_rw_signal(None), into)] selected_key: RwSignal<Option<String>>,
    #[prop(default = DefaultTableHeadRenderer.into(), into)] thead_renderer: WrapperRendererFn,
    #[prop(default = DefaultTableBodyRenderer.into(), into)] tbody_renderer: WrapperRendererFn,
    #[prop(default = DefaultTableHeadRowRenderer.into(), into)]
    thead_row_renderer: WrapperRendererFn,
    #[prop(optional, into)] row_renderer: RowRendererFn<Row>,
    #[prop(optional, into)] loading_row_renderer: LoadingRowRendererFn,
    #[prop(optional, into)] error_row_renderer: ErrorRowRendererFn,
    #[prop(optional, into)] row_placeholder_renderer_fn: RowPlaceholderRendererFn,
    #[prop(optional, into)] row_class: MaybeSignal<String>,
    #[prop(optional, into)] thead_class: MaybeSignal<String>,
    #[prop(optional, into)] thead_row_class: MaybeSignal<String>,
    #[prop(optional, into)] tbody_class: MaybeSignal<String>,
    #[prop(optional, into)] loading_row_inner_class: MaybeSignal<String>,
    #[prop(default = create_rw_signal(VecDeque::new()), into)] sorting: RwSignal<
        VecDeque<(usize, ColumnSort)>,
    >,
) -> impl IntoView
where
    Row: RowRenderer<ClassesProvider = ClsP> + Clone + 'static,
    DataP: TableDataProvider<Row> + Clone + 'static,
    ClsP: TableClassesProvider + Copy + 'static,
{
    let on_change = store_value(on_change);
    let rows = store_value(rows);

    let class_provider = ClsP::new();

    let row_class = Signal::derive(move || row_class.get());
    let loading_row_inner_class = Signal::derive(move || loading_row_inner_class.get());
    let thead_class = Signal::derive(move || class_provider.thead(&thead_class.get()));
    let thead_row_class = Signal::derive(move || class_provider.thead_row(&thead_row_class.get()));
    let tbody_class = Signal::derive(move || class_provider.tbody(&tbody_class.get()));

    let loaded_rows = create_rw_signal(LoadedRows::<Row>::new());

    let on_head_click = move |event: TableHeadEvent| {
        sorting.update(move |sorting| update_sorting_from_event(sorting, event));

        rows.update_value(move |rows| {
            rows.set_sorting(&sorting());
        });

        loaded_rows.update(|loaded_rows| {
            loaded_rows.clear();
        });
    };

    let is_selected = create_selector(selected_key);

    let (tbody_el, set_tbody_el) = create_signal(None::<web_sys::Element>);

    let UseScrollReturn { y, .. } =
        use_scroll_with_options(tbody_el, UseScrollOptions::default().throttle(100.0));

    let UseElementSizeReturn { height, .. } = use_element_size_with_options(
        tbody_el,
        UseElementSizeOptions::default().box_(web_sys::ResizeObserverBoxOptions::ContentBox),
    );

    let (row_count, set_row_count) = create_signal(None::<usize>);
    spawn_local(async move {
        let row_count = rows.get_value().row_count().await;

        set_row_count(row_count);

        if let Some(row_count) = row_count {
            loaded_rows.update(|loaded_rows| loaded_rows.resize(row_count));
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

        let start = (start / DataP::PREFERRED_CHUNK_SIZE) * DataP::PREFERRED_CHUNK_SIZE;
        let end = (end / DataP::PREFERRED_CHUNK_SIZE + 1) * DataP::PREFERRED_CHUNK_SIZE;

        let range = start..end;
        set_display_range.set(range.clone());

        let missing_range =
            loaded_rows.with_untracked(|loaded_rows| loaded_rows.missing_range(range.clone()));

        if let Some(missing_range) = missing_range {
            loaded_rows.update(|loaded_rows| loaded_rows.splice_loading(missing_range.clone()));

            spawn_local(async move {
                let result = rows
                    .get_value()
                    .get_rows(missing_range.clone())
                    .await
                    .map_err(|err| (err, missing_range.clone()));
                loaded_rows.update(|loaded_rows| loaded_rows.splice_loaded(&result));
            });
        }
    });

    let thead_content = Row::render_head_row(sorting.into(), on_head_click).into_view();

    // TODO : customizable placeholder renderer fn
    let tbody_content = view! {
        {row_placeholder_renderer_fn.run(placeholder_height_before.into())}

        {move || {
            let row_renderer = row_renderer.clone();
            let loading_row_renderer = loading_row_renderer.clone();
            let error_row_renderer = error_row_renderer.clone();
            let is_selected = is_selected.clone();
            view! {
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
                        match row {
                            RowState::Loaded(row) => row.key().to_string(),
                            _ => format!("empty-{idx}"),
                        }
                    }

                    children={
                        let row_renderer = row_renderer.clone();
                        let loading_row_renderer = loading_row_renderer.clone();
                        let error_row_renderer = error_row_renderer.clone();
                        let is_selected = is_selected.clone();
                        move |(i, row)| {
                            match row {
                                RowState::Loaded(row) => {
                                    let class_signal = Signal::derive({
                                        let key = row.key();
                                        let is_selected = is_selected.clone();
                                        move || {
                                            class_provider
                                                .clone()
                                                .row(
                                                    i,
                                                    is_selected.selected(Some(key.clone())),
                                                    &row_class.get(),
                                                )
                                        }
                                    });

                                    let selected_signal = create_rw_signal(
                                        is_selected.selected(Some(row.key())),
                                    );
                                    let _ = create_effect({
                                        let key = row.key();
                                        let is_selected = is_selected.clone();
                                        move |_| {
                                            let key = key.clone();
                                            let is_selected = is_selected.clone();
                                            let selected = is_selected.selected(Some(key.clone()));
                                            queue_microtask(move || { selected_signal.set(selected) })
                                        }
                                    });
                                    let _ = watch(
                                        move || selected_signal.get(),
                                        {
                                            let key = row.key();
                                            move |selected, prev_selected, _| {
                                                if *selected
                                                    && !prev_selected.map(|s| *s).unwrap_or_default()
                                                {
                                                    selected_key.set(Some(key.clone()));
                                                }
                                            }
                                        },
                                        false,
                                    );

                                    row_renderer.run(class_signal, row, i, selected_signal, on_change.get_value())
                                }
                                RowState::Error(err) => error_row_renderer.run(err),
                                RowState::Loading | RowState::Placeholder => {
                                    loading_row_renderer.run(
                                        Row::COLUMN_COUNT,
                                        Signal::derive(
                                            move || class_provider.loading_row_inner(&loading_row_inner_class.get())
                                        ),
                                    )
                                }
                            }
                        }
                    }
                />
            }

        }}

        {row_placeholder_renderer_fn.run(placeholder_height_after.into())}
    }.into_view();

    let tbody = tbody_renderer.run(tbody_content, tbody_class);
    let mut tbody_err = None;

    if let Ok(tbody_el) = tbody.clone().into_html_element() {
        set_tbody_el.set(Some(tbody_el.unchecked_ref::<web_sys::Element>().clone()));
    } else {
        tbody_err = Some("The tbody_renderer has to return a single root Element");
    }

    // TODO : error handling
    view! {
        {thead_renderer.run(
            thead_row_renderer.run(
                thead_content,
                thead_row_class,
            ).into_view(),
            thead_class,
        )}

        {tbody_err}
        {tbody}
    }
}

fn update_sorting_from_event(sorting: &mut VecDeque<(usize, ColumnSort)>, event: TableHeadEvent) {
    let (i, (_, mut sort)) = sorting
        .iter()
        .enumerate()
        .find(|(_, (col_index, _))| col_index == &event.index)
        .unwrap_or((0, &(event.index, ColumnSort::None)));

    if i == 0 || sort == ColumnSort::None {
        sort = match sort {
            ColumnSort::None => ColumnSort::Ascending,
            ColumnSort::Ascending => ColumnSort::Descending,
            ColumnSort::Descending => ColumnSort::None,
        };
    }

    *sorting = sorting
        .clone()
        .into_iter()
        .filter(|(col_index, sort)| *col_index != event.index && *sort != ColumnSort::None)
        .collect();

    if sort != ColumnSort::None {
        sorting.push_front((event.index, sort));
    }
}

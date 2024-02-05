use crate::components::renderer_fn::renderer_fn;
use crate::loaded_rows::{LoadedRows, RowState};
use crate::row_renderer::RowRenderer;
use crate::selection::Selection;
use crate::{
    ChangeEvent, ColumnSort, DefaultErrorRowRenderer, DefaultLoadingRowRenderer,
    DefaultRowPlaceholderRenderer, DefaultTableBodyRenderer, DefaultTableHeadRenderer,
    DefaultTableHeadRowRenderer, DefaultTableRowRenderer, EventHandler, ReloadController,
    ScrollContainer, SelectionChangeEvent, TableClassesProvider, TableDataProvider, TableHeadEvent,
};
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_use::{
    use_debounce_fn, use_element_size_with_options, use_scroll_with_options, UseElementSizeOptions,
    UseElementSizeReturn, UseScrollOptions, UseScrollReturn,
};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::ops::Range;
use std::rc::Rc;
use web_sys::Element;

renderer_fn!(
    RowRendererFn<Row>(
        class: Signal<String>,
        row: Row,
        index: usize,
        selected: Signal<bool>,
        on_select: EventHandler<web_sys::MouseEvent>,
        on_change: EventHandler<ChangeEvent<Row>>
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
    ErrorRowRendererFn(err: String, col_count: usize)
    default DefaultErrorRowRenderer
);

renderer_fn!(
    LoadingRowRendererFn(col_count: usize, class: Signal<String>, inner_class: Signal<String>)
    default DefaultLoadingRowRenderer
);

/// Render the content of a table. This is the main component of this crate.
#[component]
pub fn TableContent<Row, DataP, ClsP>(
    rows: DataP,
    #[prop(optional, into)] scroll_container: ScrollContainer,
    #[prop(optional, into)] on_change: EventHandler<ChangeEvent<Row>>,
    #[prop(optional, into)] selection: Selection,
    #[prop(optional, into)] on_selection_change: EventHandler<SelectionChangeEvent<Row>>,
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

    /// This is called once the number of rows is known.
    /// It will only be executed if [`TableDataProvider::row_count`] returns `Some(...)`.
    ///
    /// See the [paginated_rest_datasource example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/paginated_rest_datasource/src/main.rs)
    /// for how to use.
    #[prop(optional, into)]
    on_row_count: EventHandler<usize>,

    /// Allows to manually trigger a reload.
    ///
    /// See the [paginated_rest_datasource example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/paginated_rest_datasource/src/main.rs)
    /// for how to use.
    #[prop(optional)]
    reload_controller: ReloadController,
) -> impl IntoView
where
    Row: RowRenderer<ClassesProvider = ClsP> + Clone + 'static,
    DataP: TableDataProvider<Row> + 'static,
    ClsP: TableClassesProvider + Copy + 'static,
{
    let on_change = store_value(on_change);
    let rows = Rc::new(RefCell::new(rows));

    let class_provider = ClsP::new();

    let row_class = Signal::derive(move || row_class.get());
    let loading_row_inner_class = Signal::derive(move || loading_row_inner_class.get());
    let thead_class = Signal::derive(move || class_provider.thead(&thead_class.get()));
    let thead_row_class = Signal::derive(move || class_provider.thead_row(&thead_row_class.get()));
    let tbody_class = Signal::derive(move || class_provider.tbody(&tbody_class.get()));

    let loaded_rows = create_rw_signal(LoadedRows::<Row>::new());

    let first_selected_index = create_rw_signal(None::<usize>);

    let (do_reload, set_reload) = create_signal(false);
    let clear = move || {
        selection.clear();
        first_selected_index.set(None);

        loaded_rows.update(|loaded_rows| {
            loaded_rows.clear();
        });

        set_reload.set(true);
    };

    let on_head_click = {
        let rows = Rc::clone(&rows);

        move |event: TableHeadEvent| {
            sorting.update(move |sorting| update_sorting_from_event(sorting, event));

            rows.borrow_mut().set_sorting(&sorting());

            clear();
        }
    };

    create_effect(move |_| {
        // triggered when `ReloadController::reload()` is called
        reload_controller.get();
        clear();
    });

    let selected_indices = match selection {
        Selection::None => Signal::derive(|| HashSet::new()),
        Selection::Single(selected_index) => Signal::derive(move || {
            selected_index
                .get()
                .map(|i| HashSet::from([i]))
                .unwrap_or_default()
        }),
        Selection::Multiple(selected_indices) => selected_indices.into(),
    };

    let UseScrollReturn { y, set_y, .. } = use_scroll_with_options(
        scroll_container,
        UseScrollOptions::default().throttle(100.0),
    );

    let UseElementSizeReturn { height, .. } = use_element_size_with_options(
        scroll_container,
        UseElementSizeOptions::default().box_(web_sys::ResizeObserverBoxOptions::ContentBox),
    );

    let (row_count, set_row_count) = create_signal(None::<usize>);
    spawn_local({
        let rows = Rc::clone(&rows);

        async move {
            let row_count = rows.borrow().row_count().await;

            set_row_count.set(row_count);

            if let Some(row_count) = row_count {
                loaded_rows.update(|loaded_rows| loaded_rows.resize(row_count));
                on_row_count.run(row_count);
            }
        }
    });

    let (average_row_height, set_average_row_height) = create_signal(20.0);

    let first_visible_row_index =
        create_memo(move |_| (y.get() / average_row_height.get()).floor() as usize);
    let visible_row_count =
        create_memo(move |_| ((height.get() / average_row_height.get()).ceil() as usize).max(20));

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

    let (tbody_el, set_tbody_el) = create_signal(None::<web_sys::Element>);

    let compute_average_row_height = use_debounce_fn(
        move || {
            compute_average_row_height_from_loaded(
                tbody_el,
                display_range,
                y,
                &set_y,
                set_average_row_height,
                placeholder_height_before,
                loaded_rows,
            );
        },
        50.0,
    );

    create_effect(move |_| {
        let first_visible_row_index = first_visible_row_index.get();
        let visible_row_count = visible_row_count.get();

        if visible_row_count == 0 {
            return;
        }

        // with this a reload triggers this effect
        if do_reload.get() {
            set_reload.set_untracked(false);
        }

        let mut start = first_visible_row_index.saturating_sub(visible_row_count * 2);

        let mut end = start + visible_row_count * 5;

        if let Some(count) = row_count.get() {
            end = end.min(count);
        }

        end = end.min(start + 300);

        if let Some(chunk_size) = DataP::CHUNK_SIZE {
            start /= chunk_size;
            start *= chunk_size;

            end /= chunk_size;
            end += 1;
            end *= chunk_size;
        }

        loaded_rows.update_untracked(|loaded_rows| {
            if end > loaded_rows.len() {
                loaded_rows.resize(end);
            }
        });

        let range = start..end;
        set_display_range.set(range.clone());

        let missing_range =
            loaded_rows.with_untracked(|loaded_rows| loaded_rows.missing_range(range.clone()));

        if let Some(missing_range) = missing_range {
            loaded_rows.update(|loaded_rows| loaded_rows.write_loading(missing_range.clone()));

            let mut loading_ranges = vec![];
            if let Some(chunk_size) = DataP::CHUNK_SIZE {
                let mut current_range = missing_range.start..missing_range.start + chunk_size;
                while current_range.end <= missing_range.end {
                    loading_ranges.push(current_range.clone());
                    current_range = current_range.end..current_range.end + chunk_size;
                }
            } else {
                loading_ranges.push(missing_range);
            }

            // TODO : implement max concurrent requests
            for missing_range in loading_ranges {
                let compute_average_row_height = compute_average_row_height.clone();
                spawn_local({
                    let rows = Rc::clone(&rows);

                    async move {
                        let result = rows.borrow().get_rows(missing_range.clone()).await;
                        loaded_rows
                            .update(|loaded_rows| loaded_rows.write_loaded(result, missing_range));

                        compute_average_row_height();
                    }
                });
            }
        }
    });

    let thead_content = Row::render_head_row(sorting.into(), on_head_click).into_view();

    let tbody_content = view! {
        {row_placeholder_renderer_fn.run(placeholder_height_before.into())}

        {move || {
            let row_renderer = row_renderer.clone();
            let loading_row_renderer = loading_row_renderer.clone();
            let error_row_renderer = error_row_renderer.clone();
            let on_selection_change = on_selection_change.clone();

            view! {
                <For
                    each=move || {
                        with!(|loaded_rows, display_range| {
                            loaded_rows[display_range.clone()]
                                .iter()
                                .cloned()
                                .enumerate()
                                .map(|(i, row)| ( i + display_range.start, row))
                                .collect::<Vec<_>>()
                        })
                    }

                    key=|(idx, row)| {
                        match row {
                            RowState::Loaded(_) => idx.to_string(),
                            RowState::Placeholder => format!("placeholder-{idx}"),
                            RowState::Error(_) => format!("error-{idx}"),
                            RowState::Loading => format!("loading-{idx}"),
                        }
                    }

                    children={
                        let row_renderer = row_renderer.clone();
                        let loading_row_renderer = loading_row_renderer.clone();
                        let error_row_renderer = error_row_renderer.clone();
                        let on_selection_change = on_selection_change.clone();

                        move |(i, row)| {
                            match row {
                                RowState::Loaded(row) => {
                                    let selected_signal = Signal::derive(
                                        move || selected_indices.get().contains(&i)
                                    );

                                    let class_signal = Signal::derive(move || {
                                        class_provider
                                            .row(
                                                i,
                                                selected_signal.get(),
                                                &row_class.get(),
                                            )
                                    });

                                    let on_select = {
                                        let on_selection_change = on_selection_change.clone();
                                        let row = row.clone();

                                        move |evt: web_sys::MouseEvent| {
                                            update_selection(evt, selection, first_selected_index, i);

                                            let selection_change_event = SelectionChangeEvent {
                                                row: row.clone(),
                                                row_index:i,
                                                selected: selected_signal.get_untracked(),
                                            };
                                            on_selection_change.run(selection_change_event);
                                        }
                                    };

                                    row_renderer.run(class_signal, row, i, selected_signal, on_select.into(), on_change.get_value())
                                }
                                RowState::Error(err) => error_row_renderer.run(err, Row::COLUMN_COUNT),
                                RowState::Loading | RowState::Placeholder => {
                                    loading_row_renderer.run(
                                        Row::COLUMN_COUNT,
                                        Signal::derive(
                                            move || class_provider.row(i, false, &row_class.get())
                                        ),
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

fn compute_average_row_height_from_loaded<Row, ClsP>(
    tbody_el: ReadSignal<Option<Element>>,
    display_range: ReadSignal<Range<usize>>,
    y: Signal<f64>,
    set_y: &impl Fn(f64),
    set_average_row_height: WriteSignal<f64>,
    placeholder_height_before: Memo<f64>,
    loaded_rows: RwSignal<LoadedRows<Row>>,
) where
    Row: RowRenderer<ClassesProvider = ClsP> + Clone + 'static,
{
    if let Some(el) = tbody_el.get_untracked() {
        let el: &web_sys::Element = &el;
        let display_range = display_range.get_untracked();
        if display_range.end > 0 {
            let avg_row_height = loaded_rows.with_untracked(|loaded_rows| {
                let mut loaded_row_end_index = None;

                for i in display_range.clone() {
                    if matches!(loaded_rows[i], RowState::Loaded(_)) {
                        loaded_row_end_index = Some(i);
                    } else {
                        if loaded_row_end_index.is_some() {
                            break;
                        }
                    }
                }

                if let Some(loaded_row_end_index) = loaded_row_end_index {
                    if loaded_row_end_index == 0 {
                        return None;
                    }

                    let children = el.children();

                    let placeholder_before = children.get_with_index(0);
                    // skip first element, because it's the "before" placeholder
                    let last_loaded_row = children
                        .get_with_index((loaded_row_end_index + 1 - display_range.start) as u32);

                    if let (Some(placeholder_before), Some(last_loaded_row)) =
                        (placeholder_before, last_loaded_row)
                    {
                        return Some(
                            (last_loaded_row.get_bounding_client_rect().top()
                                - placeholder_before.get_bounding_client_rect().top())
                                / (loaded_row_end_index - 1) as f64,
                        );
                    }
                }

                None
            });

            if let Some(avg_row_height) = avg_row_height {
                let prev_placeholder_height_before = placeholder_height_before.get_untracked();

                set_average_row_height(avg_row_height);

                let new_placeholder_height_before = placeholder_height_before.get_untracked();
                set_y(
                    y.get_untracked() - prev_placeholder_height_before
                        + new_placeholder_height_before,
                );
            }
        }
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

fn get_keyboard_modifiers(evt: &web_sys::MouseEvent) -> (bool, bool) {
    let meta_pressed = evt.meta_key() || evt.ctrl_key();
    let shift_pressed = evt.shift_key();
    (meta_pressed, shift_pressed)
}

fn update_selection(
    evt: web_sys::MouseEvent,
    selection: Selection,
    first_selected_index: RwSignal<Option<usize>>,
    i: usize,
) {
    match selection {
        Selection::None => {}
        Selection::Single(selected_index) => {
            selected_index.set(Some(i));
        }
        Selection::Multiple(selected_indices) => {
            selected_indices.update(|selected_indices| {
                let (meta_pressed, shift_pressed) = get_keyboard_modifiers(&evt);

                if meta_pressed {
                    if selected_indices.contains(&i) {
                        selected_indices.remove(&i);
                    } else {
                        selected_indices.insert(i);
                    }
                    match selected_indices.len() {
                        0 => first_selected_index.set(None),
                        1 => {
                            first_selected_index.set(Some(i));
                        }
                        _ => {
                            // do nothing
                        }
                    }
                } else if shift_pressed {
                    if let Some(first_selected_index) = first_selected_index.get() {
                        let min = first_selected_index.min(i);
                        let max = first_selected_index.max(i);
                        for i in min..=max {
                            selected_indices.insert(i);
                        }
                    } else {
                        selected_indices.insert(i);
                        first_selected_index.set(Some(i));
                    }
                } else {
                    selected_indices.clear();
                    selected_indices.insert(i);
                    first_selected_index.set(Some(i));
                }
            });
        }
    }
}

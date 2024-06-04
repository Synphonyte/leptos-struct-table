use crate::components::renderer_fn::renderer_fn;
use crate::loaded_rows::{LoadedRows, RowState};
use crate::selection::Selection;
use crate::table_row::TableRow;
use crate::{
    ChangeEvent, ColumnSort, DefaultErrorRowRenderer, DefaultLoadingRowRenderer,
    DefaultRowPlaceholderRenderer, DefaultTableBodyRenderer, DefaultTableHeadRenderer,
    DefaultTableHeadRowRenderer, DefaultTableRowRenderer, DisplayStrategy, EventHandler,
    ReloadController, ScrollContainer, SelectionChangeEvent, SortingMode, TableClassesProvider,
    TableDataProvider, TableHeadEvent,
};
use leptos::html::AnyElement;
use leptos::leptos_dom::is_browser;
use leptos::*;
use leptos_use::{
    use_debounce_fn, use_element_size_with_options, use_scroll_with_options, UseElementSizeOptions,
    UseElementSizeReturn, UseScrollOptions, UseScrollReturn,
};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Range;
use std::rc::Rc;

const MAX_DISPLAY_ROW_COUNT: usize = 500;

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
    where Row: TableRow + Clone + 'static
);

renderer_fn!(
    RowPlaceholderRendererFn(height: Signal<f64>)
    default DefaultRowPlaceholderRenderer
);

renderer_fn!(
    WrapperRendererFn(view: View, class: Signal<String>)
);

renderer_fn!(
    TbodyRendererFn(view: Fragment, class: Signal<String>, node_ref: NodeRef<AnyElement>)
);

renderer_fn!(
    ErrorRowRendererFn(err: String, index: usize, col_count: usize)
    default DefaultErrorRowRenderer
);

renderer_fn!(
    LoadingRowRendererFn(class: Signal<String>, get_cell_class: Callback<usize, String>, get_cell_inner_class: Callback<usize, String>, index: usize, col_count: usize)
    default DefaultLoadingRowRenderer
);

/// Render the content of a table. This is the main component of this crate.
#[component]
pub fn TableContent<Row, DataP, Err, ClsP>(
    /// The data to be rendered in this table.
    /// This must implement [`TableDataProvider`] or [`PaginatedTableDataProvider`].
    rows: DataP,
    /// The container element which has scrolling capabilities. By default this is the `body` element.
    #[prop(optional, into)]
    scroll_container: ScrollContainer,
    /// Event handler for when a row is edited.
    /// Check out the [editable example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/editable/src/main.rs).
    #[prop(optional, into)]
    on_change: EventHandler<ChangeEvent<Row>>,
    /// Selection mode together with the `RwSignal` to hold the selection. Available modes are
    /// - `None` - No selection (default)
    /// - `Single` - Single selection
    /// - `Multiple` - Multiple selection
    ///
    /// Please see [`Selection`] for more information and check out the
    /// [selectable example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/selectable/src/main.rs).
    #[prop(optional, into)]
    selection: Selection,
    /// Event handler callback for when the selection changes.
    /// See the [selectable example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/selectable/src/main.rs) for details.
    #[prop(optional, into)]
    on_selection_change: EventHandler<SelectionChangeEvent<Row>>,
    /// Renderer function for the table head. Defaults to [`DefaultTableHeadRenderer`]. For a full example see the
    /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    #[prop(default = DefaultTableHeadRenderer.into(), into)]
    thead_renderer: WrapperRendererFn,
    /// Renderer function for the table body. Defaults to [`DefaultTableBodyRenderer`]. For a full example see the
    /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    #[prop(default = DefaultTableBodyRenderer.into(), into)]
    tbody_renderer: TbodyRendererFn,
    /// Renderer function for the table head row. Defaults to [`DefaultTableHeadRowRenderer`]. For a full example see the
    /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    #[prop(default = DefaultTableHeadRowRenderer.into(), into)]
    thead_row_renderer: WrapperRendererFn,
    /// The row renderer. Defaults to [`DefaultTableRowRenderer`]. For a full example see the
    /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    #[prop(optional, into)]
    row_renderer: RowRendererFn<Row>,
    /// The row renderer for when that row is currently being loaded.
    /// Defaults to [`DefaultLoadingRowRenderer`]. For a full example see the
    /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    #[prop(optional, into)]
    loading_row_renderer: LoadingRowRendererFn,
    /// The row renderer for when that row failed to load.
    /// Defaults to [`DefaultErrorRowRenderer`]. For a full example see the
    /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    #[prop(optional, into)]
    error_row_renderer: ErrorRowRendererFn,
    /// The row placeholder renderer. Defaults to [`DefaultRowPlaceholderRenderer`].
    /// This is used in place of rows that are not shown
    /// before and after the currently visible rows.
    #[prop(optional, into)]
    row_placeholder_renderer: RowPlaceholderRendererFn,
    /// Additional classes to add to rows
    #[prop(optional, into)]
    row_class: MaybeSignal<String>,
    /// Additional classes to add to the thead
    #[prop(optional, into)]
    thead_class: MaybeSignal<String>,
    /// Additional classes to add to the row inside the thead
    #[prop(optional, into)]
    thead_row_class: MaybeSignal<String>,
    /// Additional classes to add to the tbody
    #[prop(optional, into)]
    tbody_class: MaybeSignal<String>,
    /// Additional classes to add to the cell inside a row that is being loaded
    #[prop(optional, into)]
    loading_cell_class: MaybeSignal<String>,
    /// Additional classes to add to the inner element inside a cell that is inside a row that is being loaded
    #[prop(optional, into)]
    loading_cell_inner_class: MaybeSignal<String>,
    /// The sorting to apply to the table.
    /// For this to work you have add `#[table(sortable)]` to your struct.
    /// Please see the [simple example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/simple/src/main.rs).
    #[prop(default = create_rw_signal(VecDeque::new()), into)]
    sorting: RwSignal<VecDeque<(usize, ColumnSort)>>,
    /// The sorting mode to use. Defaults to `MultiColumn`. Please note that
    /// this to have any effect you have to add the macro attribute `#[table(sortable)]`
    /// to your struct.
    #[prop(optional)]
    sorting_mode: SortingMode,
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
    /// The display strategy to use when rendering the table.
    /// Can be one of
    /// - `Virtualization`
    /// - `InfiniteScroll`
    /// - `Pagination`
    ///
    /// Please check [`DisplayStrategy`] to see explanations of all available options.
    #[prop(optional)]
    display_strategy: DisplayStrategy,
    /// The maximum number of loading rows to display. Defaults to `None` which means unlimited.
    /// Use this if you load a small number of rows and don't want the entire screen to be full of
    /// loading rows.
    #[prop(optional)]
    loading_row_display_limit: Option<usize>,

    #[prop(optional)] _marker: PhantomData<Err>,
) -> impl IntoView
where
    Row: TableRow<ClassesProvider = ClsP> + Clone + 'static,
    DataP: TableDataProvider<Row, Err> + 'static,
    Err: Debug,
    ClsP: TableClassesProvider + Copy + 'static,
{
    let on_change = store_value(on_change);
    let rows = Rc::new(RefCell::new(rows));

    let class_provider = ClsP::new();

    let row_class = Signal::derive(move || row_class.get());
    let loading_cell_inner_class = Signal::derive(move || loading_cell_inner_class.get());
    let loading_cell_class = Signal::derive(move || loading_cell_class.get());
    let thead_class = Signal::derive(move || class_provider.thead(&thead_class.get()));
    let thead_row_class = Signal::derive(move || class_provider.thead_row(&thead_row_class.get()));
    let tbody_class = Signal::derive(move || class_provider.tbody(&tbody_class.get()));

    let loaded_rows = create_rw_signal(LoadedRows::<Row>::new());

    let first_selected_index = create_rw_signal(None::<usize>);

    let (row_count, set_row_count) = create_signal(None::<usize>);

    let set_known_row_count = move |row_count: usize| {
        set_row_count.set(Some(row_count));
        loaded_rows.update(|loaded_rows| loaded_rows.resize(row_count));
        on_row_count.run(row_count);
        display_strategy.set_row_count(row_count);
    };

    let load_row_count = {
        let rows = Rc::clone(&rows);
        let set_known_row_count = set_known_row_count.clone();

        move || {
            spawn_local({
                let rows = Rc::clone(&rows);
                let set_known_row_count = set_known_row_count.clone();

                async move {
                    let row_count = rows.borrow().row_count().await;

                    if let Some(row_count) = row_count {
                        set_known_row_count(row_count);
                    }
                }
            })
        }
    };

    let (reload_count, set_reload_count) = create_signal(0_usize);
    let clear = {
        let load_row_count = load_row_count.clone();

        move |clear_row_count: bool| {
            selection.clear();
            first_selected_index.set(None);

            loaded_rows.update(|loaded_rows| {
                loaded_rows.clear();
            });

            if clear_row_count {
                let reload = row_count.get_untracked().is_some();
                set_row_count.set(None);
                if reload {
                    load_row_count();
                }
            }

            set_reload_count.set(reload_count.get().overflowing_add(1).0);
        }
    };

    let on_head_click = {
        let rows = Rc::clone(&rows);
        let clear = clear.clone();

        move |event: TableHeadEvent| {
            sorting.update(move |sorting| sorting_mode.update_sorting_from_event(sorting, event));

            rows.borrow_mut().set_sorting(&sorting.get());

            clear(false);
        }
    };

    create_effect({
        let rows = Rc::clone(&rows);
        move |_| {
            rows.borrow_mut().set_sorting(&sorting.get());
        }
    });

    create_effect({
        let rows = Rc::clone(&rows);

        move |_| {
            // triggered when `ReloadController::reload()` is called
            reload_controller.track();
            rows.borrow().track();
            clear(true);
        }
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

    if is_browser()
        && matches!(
            display_strategy,
            DisplayStrategy::Virtualization | DisplayStrategy::Pagination { .. }
        )
    {
        load_row_count();
    }

    let (average_row_height, set_average_row_height) = create_signal(20.0);

    let first_visible_row_index = if let DisplayStrategy::Pagination {
        controller,
        row_count,
    } = display_strategy
    {
        create_memo(move |_| controller.current_page.get() * row_count)
    } else {
        create_memo(move |_| (y.get() / average_row_height.get()).floor() as usize)
    };
    let visible_row_count = match display_strategy {
        DisplayStrategy::Pagination { row_count, .. } => Signal::derive(move || row_count),

        DisplayStrategy::Virtualization | DisplayStrategy::InfiniteScroll => {
            create_memo(move |_| {
                ((height.get() / average_row_height.get()).ceil() as usize).max(20)
            })
            .into()
        }
    };

    let (display_range, set_display_range) = create_signal(0..0);

    let placeholder_height_before =
        if matches!(display_strategy, DisplayStrategy::Pagination { .. }) {
            Signal::derive(move || 0.0)
        } else {
            create_memo(move |_| display_range.get().start as f64 * average_row_height.get()).into()
        };

    let placeholder_height_after = if matches!(display_strategy, DisplayStrategy::Pagination { .. })
    {
        Signal::derive(move || 0.0)
    } else {
        create_memo(move |_| {
            let row_count_after = if let Some(row_count) = row_count.get() {
                (row_count.saturating_sub(display_range.get().end)) as f64
            } else {
                0.0
            };

            row_count_after * average_row_height.get()
        })
        .into()
    };

    let tbody_ref = create_node_ref::<AnyElement>();

    let compute_average_row_height = use_debounce_fn(
        move || {
            compute_average_row_height_from_loaded(
                tbody_ref,
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
        let visible_row_count = visible_row_count.get().min(MAX_DISPLAY_ROW_COUNT);

        // with this a reload triggers this effect
        reload_count.track();

        if visible_row_count == 0 {
            return;
        }

        let mut start = first_visible_row_index.saturating_sub(visible_row_count * 2);

        let mut end = start + visible_row_count * 5;

        if let Some(chunk_size) = DataP::CHUNK_SIZE {
            start /= chunk_size;
            start *= chunk_size;

            end /= chunk_size;
            end += 1;
            end *= chunk_size;
        }

        if let Some(row_count) = row_count.get() {
            end = end.min(row_count);
        }

        end = end.min(start + MAX_DISPLAY_ROW_COUNT);

        loaded_rows.update_untracked(|loaded_rows| {
            if end > loaded_rows.len() {
                loaded_rows.resize(end);
            }
        });

        let range = start..end;

        set_display_range.set(match display_strategy {
            DisplayStrategy::Virtualization | DisplayStrategy::InfiniteScroll => range.clone(),
            DisplayStrategy::Pagination { row_count, .. } => {
                first_visible_row_index..(first_visible_row_index + row_count).min(end)
            }
        });

        let missing_range =
            loaded_rows.with_untracked(|loaded_rows| loaded_rows.missing_range(range.clone()));

        if let Some(missing_range) = missing_range {
            let mut end = missing_range.end;
            if let Some(row_count) = row_count.get() {
                end = end.min(row_count);

                if end <= missing_range.start {
                    return;
                }
            }

            loaded_rows.update(|loaded_rows| loaded_rows.write_loading(missing_range.clone()));

            let mut loading_ranges = vec![];
            if let Some(chunk_size) = DataP::CHUNK_SIZE {
                let start = missing_range.start / chunk_size * chunk_size;
                let mut current_range = start..start + chunk_size;
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
                    let set_known_row_count = set_known_row_count.clone();

                    async move {
                        let latest_reload_count = reload_count.get_untracked();

                        let result = rows
                            .borrow()
                            .get_rows(missing_range.clone())
                            .await
                            .map_err(|err| format!("{err:?}"));

                        // make sure the loaded data is still valid
                        if reload_count.get_untracked() != latest_reload_count {
                            return;
                        }

                        if let Ok((_, loaded_range)) = &result {
                            if loaded_range.end < missing_range.end {
                                if let Some(row_count) = row_count.get_untracked() {
                                    if loaded_range.end < row_count {
                                        set_known_row_count(loaded_range.end);
                                    }
                                } else {
                                    set_known_row_count(loaded_range.end);
                                }
                            }
                        }
                        loaded_rows
                            .update(|loaded_rows| loaded_rows.write_loaded(result, missing_range));

                        compute_average_row_height();
                    }
                });
            }
        }
    });

    let thead_content = Row::render_head_row(sorting.into(), on_head_click).into_view();

    let tbody_content = {
        let row_renderer = row_renderer.clone();
        let loading_row_renderer = loading_row_renderer.clone();
        let error_row_renderer = error_row_renderer.clone();
        let on_selection_change = on_selection_change.clone();

        view! {
            {row_placeholder_renderer.run(placeholder_height_before.into())}

            <For
                each=move || {
                    with!(|loaded_rows, display_range| {
                        let iter = loaded_rows[display_range.clone()]
                            .iter()
                            .cloned()
                            .enumerate()
                            .map(|(i, row)| ( i + display_range.start, row));

                        if let Some(loading_row_display_limit) = loading_row_display_limit {
                            let mut loading_row_count = 0;

                            iter.filter(|(_, row)| {
                                if matches!(row, RowState::Loading | RowState::Placeholder) {
                                    loading_row_count += 1;
                                    loading_row_count <= loading_row_display_limit
                                } else {
                                    true
                                }
                            }).collect::<Vec<_>>()
                        } else {
                            iter.collect::<Vec<_>>()
                        }
                    })
                }

                key=|(idx, row)| {
                    match row {
                        RowState::Loaded(_) => idx.to_string(),
                        RowState::Error(_) => format!("error-{idx}"),
                        RowState::Loading | RowState::Placeholder => format!("loading-{idx}"),
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
                            RowState::Error(err) => error_row_renderer.run(err, i, Row::COLUMN_COUNT),
                            RowState::Loading | RowState::Placeholder => {
                                loading_row_renderer.run(
                                    Signal::derive(
                                        move || class_provider.row(i, false, &row_class.get())
                                    ),
                                    Callback::new(
                                        move |col_index: usize| class_provider.loading_cell(i, col_index, &loading_cell_class.get())
                                    ),
                                    Callback::new(
                                        move |col_index: usize| class_provider.loading_cell_inner(i, col_index, &loading_cell_inner_class.get())
                                    ),
                                    i,
                                    Row::COLUMN_COUNT,
                                )
                            }
                        }
                    }
                }
            />
            {row_placeholder_renderer.run(placeholder_height_after.into())}
        }
    };

    let tbody = tbody_renderer.run(tbody_content, tbody_class, tbody_ref);

    view! {
        {thead_renderer.run(
            thead_row_renderer.run(
                thead_content,
                thead_row_class,
            ).into_view(),
            thead_class,
        )}

        {tbody}
    }
}

fn compute_average_row_height_from_loaded<Row, ClsP>(
    tbody_ref: NodeRef<AnyElement>,
    display_range: ReadSignal<Range<usize>>,
    y: Signal<f64>,
    set_y: &impl Fn(f64),
    set_average_row_height: WriteSignal<f64>,
    placeholder_height_before: Signal<f64>,
    loaded_rows: RwSignal<LoadedRows<Row>>,
) where
    Row: TableRow<ClassesProvider = ClsP> + Clone + 'static,
{
    if let Some(el) = tbody_ref.get_untracked() {
        let el: &web_sys::Element = &el;
        let display_range = display_range.get_untracked();
        if display_range.end > 0 {
            let avg_row_height = loaded_rows.with_untracked(|loaded_rows| {
                let mut loading_row_start_index = None;
                let mut loading_row_end_index = None;

                for i in display_range.clone() {
                    if matches!(loaded_rows[i], RowState::Loaded(_) | RowState::Loading) {
                        if loading_row_start_index.is_none() {
                            loading_row_start_index = Some(i);
                        }
                        loading_row_end_index = Some(i);
                    } else {
                        if loading_row_end_index.is_some() {
                            break;
                        }
                    }
                }

                if let (Some(loading_row_start_index), Some(loading_row_end_index)) =
                    (loading_row_start_index, loading_row_end_index)
                {
                    if loading_row_end_index == loading_row_start_index {
                        return None;
                    }

                    let children = el.children();

                    // skip first element, because it's the "before" placeholder
                    let first_loading_row = children
                        .get_with_index((loading_row_start_index + 1 - display_range.start) as u32);
                    let last_loading_row = children
                        .get_with_index((loading_row_end_index + 1 - display_range.start) as u32);

                    if let (Some(first_loading_row), Some(last_loaded_row)) =
                        (first_loading_row, last_loading_row)
                    {
                        return Some(
                            (last_loaded_row.get_bounding_client_rect().top()
                                - first_loading_row.get_bounding_client_rect().top())
                                / (loading_row_end_index - loading_row_start_index) as f64,
                        );
                    }
                }

                None
            });

            if let Some(avg_row_height) = avg_row_height {
                let prev_placeholder_height_before = placeholder_height_before.get_untracked();

                set_average_row_height.set(avg_row_height);

                let new_placeholder_height_before = placeholder_height_before.get_untracked();
                set_y(
                    y.get_untracked() - prev_placeholder_height_before
                        + new_placeholder_height_before,
                );
            }
        }
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

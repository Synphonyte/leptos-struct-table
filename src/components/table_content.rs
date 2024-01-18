use crate::row_renderer::RowRenderer;
use crate::{
    ChangeEventHandler, ColumnSort, DefaultTableBodyRenderer, DefaultTableHeadRenderer,
    DefaultTableHeadRowRenderer, DefaultTableRowRenderer, TableClassesProvider, TableDataProvider,
    TableHeadEvent,
};
use leptos::*;
use std::collections::VecDeque;
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone)]
pub struct RowRendererFn<Row, Key>(
    Rc<dyn Fn(Signal<String>, Row, usize, RwSignal<bool>) -> View>,
    PhantomData<Key>,
)
where
    Row: RowRenderer<Key>,
    Key: Clone;

impl<F, Ret, Row, Key> From<F> for RowRendererFn<Row, Key>
where
    F: Fn(Signal<String>, Row, usize, RwSignal<bool>) -> Ret + 'static,
    Ret: IntoView,
    Row: RowRenderer<Key>,
    Key: Clone,
{
    fn from(f: F) -> Self {
        Self(
            Rc::new(move |class, row, index, selected| f(class, row, index, selected).into_view()),
            PhantomData,
        )
    }
}

impl<Row, Key> Default for RowRendererFn<Row, Key>
where
    Row: RowRenderer<Key> + 'static,
    Key: Clone + 'static,
{
    fn default() -> Self {
        Self(
            Rc::new(move |class, row, index, selected| {
                DefaultTableRowRenderer(class, row, index, selected).into_view()
            }),
            PhantomData,
        )
    }
}

impl<Row, Key> RowRendererFn<Row, Key>
where
    Row: RowRenderer<Key>,
    Key: Clone,
{
    pub fn run(
        &self,
        class: Signal<String>,
        row: Row,
        index: usize,
        selected: RwSignal<bool>,
    ) -> View {
        (self.0)(class, row, index, selected)
    }
}

#[derive(Clone)]
pub struct WrapperRendererFn(Rc<dyn Fn(View, Signal<String>) -> View>);

impl<F, Ret> From<F> for WrapperRendererFn
where
    F: Fn(View, Signal<String>) -> Ret + 'static,
    Ret: IntoView,
{
    fn from(f: F) -> Self {
        Self(Rc::new(move |view, class| f(view, class).into_view()))
    }
}

impl WrapperRendererFn {
    pub fn run(&self, view: View, class: Signal<String>) -> View {
        (self.0)(view, class)
    }
}

#[component]
pub fn TableContent<Row, DataP, Key, ClsP>(
    rows: DataP,
    #[prop(optional, into)] on_change: ChangeEventHandler<Row>,
    #[prop(default = create_rw_signal(None), into)] selected_key: RwSignal<Option<Key>>,
    #[prop(optional, into)] row_renderer: RowRendererFn<Row, Key>,
    #[prop(default = DefaultTableHeadRenderer.into(), into)] thead_renderer: WrapperRendererFn,
    #[prop(default = DefaultTableHeadRowRenderer.into(), into)]
    thead_row_renderer: WrapperRendererFn,
    #[prop(default = DefaultTableBodyRenderer.into(), into)] tbody_renderer: WrapperRendererFn,
    #[prop(optional, into)] row_class: MaybeSignal<String>,
    #[prop(optional, into)] thead_class: MaybeSignal<String>,
    #[prop(optional, into)] thead_row_class: MaybeSignal<String>,
    #[prop(optional, into)] tbody_class: MaybeSignal<String>,
    #[prop(optional, into)] sorting: RwSignal<VecDeque<(usize, ColumnSort)>>,
) -> impl IntoView
where
    Row: RowRenderer<Key, ClassesProvider = ClsP> + Clone + 'static,
    Key: PartialEq + Eq + Clone + Hash + 'static,
    DataP: TableDataProvider<Row> + Clone + 'static,
    ClsP: TableClassesProvider + Copy + 'static,
{
    let on_change = store_value(on_change);
    let rows = store_value(rows);

    let class_provider = ClsP::new();

    let row_class = Signal::derive(move || row_class.get());
    let thead_class = Signal::derive(move || class_provider.thead(&thead_class.get()));
    let thead_row_class = Signal::derive(move || class_provider.thead_row(&thead_row_class.get()));
    let tbody_class = Signal::derive(move || class_provider.tbody(&tbody_class.get()));

    let on_head_click = move |event: TableHeadEvent| {
        sorting.update(move |sorting| update_sorting_from_event(sorting, event));

        rows.update_value(move |rows| {
            rows.set_sorting(&sorting());
        });
    };

    let is_selected = create_selector(selected_key);

    let (items, set_items) = create_signal(Ok(vec![]));

    // TODO : this is only to test. Replace with virtualization...
    let _ = watch(
        move || sorting.get(),
        move |_, _, _| {
            spawn_local(async move {
                let rows = rows.get_value();
                set_items.set(
                    rows.get_rows(0..100)
                        .await
                        .map(|(rows, _)| rows.into_iter().enumerate().collect()),
                );
            })
        },
        true,
    );

    let thead_content = Row::render_head_row(sorting.into(), on_head_click).into_view();

    let tbody_content = view! {
        {move || {
            let items = items.get();
            items
                .map_err(|e| ServerFnError::ServerError(e))
                .map({
                    let row_renderer = row_renderer.clone();
                    let is_selected = is_selected.clone();
                    move |items| {
                        view! {
                            <For
                                each=move || items.clone()
                                key=|(_, item)| item.key()
                                children={
                                    let row_renderer = row_renderer.clone();
                                    let is_selected = is_selected.clone();
                                    move |(i, item)| {
                                        let class_signal = Signal::derive({
                                            let key = item.key();
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
                                            is_selected.selected(Some(item.key())),
                                        );
                                        let _ = create_effect({
                                            let key = item.key();
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
                                                let key = item.key();
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
                                        row_renderer.run(class_signal, item, i, selected_signal)
                                    }
                                }
                            />
                        }
                    }
                })
        }}
    }.into_view();

    // TODO : error handling
    view! {
        {thead_renderer.run(
            thead_row_renderer.run(
                thead_content,
                thead_row_class,
            ).into_view(),
            thead_class,
        )}

        <ErrorBoundary fallback=move |err| {
            view! { <p>{move || err.get().iter().map(|err| format!("{err:?}")).collect_view()}</p> }
        }>

            {tbody_renderer.run(tbody_content, tbody_class)}

        </ErrorBoundary>
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

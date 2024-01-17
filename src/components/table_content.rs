use crate::row_renderer::RowRenderer;
use crate::{
    ChangeEventHandler, ColumnSort, DefaultTableHeadRenderer, DefaultTableRowRenderer,
    TableClassesProvider, TableDataProvider, TableHeadEvent,
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
pub struct HeadRendererFn(Rc<dyn Fn(View) -> View>);

impl<F, Ret> From<F> for HeadRendererFn
where
    F: Fn(View) -> Ret + 'static,
    Ret: IntoView,
{
    fn from(f: F) -> Self {
        Self(Rc::new(move |view| f(view).into_view()))
    }
}

impl Default for HeadRendererFn {
    fn default() -> Self {
        Self(Rc::new(move |view| {
            DefaultTableHeadRenderer(view).into_view()
        }))
    }
}

impl HeadRendererFn {
    pub fn run(&self, view: View) -> View {
        (self.0)(view)
    }
}

#[component]
pub fn TableContent<Row, DataP, Key, ClsP>(
    rows: DataP,
    #[prop(optional, into)] on_change: ChangeEventHandler<Row>,
    #[prop(default = create_rw_signal(None), into)] selected_key: RwSignal<Option<Key>>,
    #[prop(optional, into)] row_renderer: RowRendererFn<Row, Key>,
    #[prop(optional, into)] row_class: MaybeSignal<String>,
    #[prop(optional, into)] head_renderer: HeadRendererFn,
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
    let row_class = Signal::derive(move || row_class.get());

    let on_head_click = move |event: TableHeadEvent| {
        sorting.update(move |sorting| update_sorting_from_event(sorting, event));

        rows.update_value(move |rows| {
            rows.set_sorting(&sorting());
        });
    };

    let is_selected = move |key| create_selector(selected_key).selected(key);

    let (items, set_items) = create_signal(Ok(vec![]));

    // TODO : this is only to test. Replace with virtualization...
    spawn_local(async move {
        let rows = rows.get_value();
        set_items.set(
            rows.get_rows(0..100)
                .await
                .map(|(rows, _)| rows.into_iter().enumerate().collect()),
        );
    });

    let class_provider = ClsP::new();

    // TODO : error handling
    view! {
        {Row::render_head_row(sorting.into(), on_head_click)}
        <ErrorBoundary fallback=move |err| {
            view! {
                <p>{
                    move || err.get().iter().map(|err| format!("{err:?}")).collect_view()
                }</p>
            }
        }>
            {
                move || {
                let items = items.get();
                items.map_err(|e| ServerFnError::ServerError(e)).map({
                    let row_renderer = row_renderer.clone();

                    move |items| view! {
                        <For
                            each=move || items.clone()
                            key=|(_, item)| item.key()
                            children={
                                let row_renderer = row_renderer.clone();
                                move |(i, item)| {
                                    let is_sel = is_selected.clone();

                                    let class_signal = Signal::derive({
                                        let key = item.key();
                                        move || class_provider.clone().row(i, is_sel(Some(key.clone())), &row_class.get())
                                    });

                                    let is_sel = is_selected.clone();

                                    let selected_signal = create_rw_signal(is_sel(Some(item.key())));

                                    create_effect({
                                        let key = item.key();

                                        move |_| {
                                            selected_signal.set(is_sel(Some(key.clone())))
                                        }
                                    });

                                    let _ = watch(
                                        move || selected_signal.get(),
                                        {
                                            let key = item.key();

                                            move |selected, prev_selected, _| {
                                                if *selected && !prev_selected.map(|s| *s).unwrap_or_default() {
                                                    selected_key.set(Some(key.clone()));
                                                }
                                            }
                                        },
                                        false
                                    );

                                    row_renderer.run(class_signal, item, i, selected_signal)
                                }
                            }
                        />
                    }
                })
            }}
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

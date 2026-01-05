#![deny(missing_docs)]
//! Column order, reordering and visibility showcase example.

use ::chrono::NaiveDate;
use derive_more::{Deref, DerefMut};
use leptos::prelude::*;
use leptos_struct_table::*;
use std::sync::{mpsc::Sender, Arc};
use web_sys::DragEvent;

/// This generates the component BookTable
#[derive(TableRow, Clone)]
#[table(
    sortable,
    impl_vec_data_provider,
    column_index_type = "enum",
    classes_provider = "TailwindClassesPreset"
)]
pub struct Book {
    /// Title of the book.
    pub title: String,

    /// Author of the book.
    pub author: String,
    /// Date when book has been published.
    pub publish_date: Option<NaiveDate>,
    /// Description of the book. Optional.
    #[table(none_value = "-")]
    pub description: Option<String>,
}

/// New-type pattern because otherwise the impl TableRow doesn't work because of orphan rules.
#[derive(Deref, DerefMut, Clone)]

pub struct ArcBook(Arc<Book>);

struct CustomHeadDragHandler<Column> {
    default_handler: DefaultHeadDragHandler,
    drop_tx: Sender<DragState<Column>>,
    trigger: Trigger,
}

impl<Column: Clone + PartialEq + Send + Sync + 'static> DragHandler<Column>
    for CustomHeadDragHandler<Column>
{
    fn received_drop(
        &self,
        drag_state: DragStateCarrier<Column>,
        column: Column,
        event: DragEvent,
    ) {
        if let Some(drop_state) = drag_state.get() {
            self.drop_tx
                .send(drop_state)
                .expect("dnd channel died before eol.");
            self.trigger.notify();
        }
        self.default_handler
            .received_drop(drag_state, column, event);
    }

    fn dragging_over(
        &self,
        drag_state: DragStateCarrier<Column>,
        column: Column,
        event: DragEvent,
    ) {
        self.default_handler
            .dragging_over(drag_state, column, event);
    }

    fn drag_leave(&self, drag_state: DragStateCarrier<Column>, column: Column, event: DragEvent) {
        self.default_handler.drag_leave(drag_state, column, event);
    }

    fn drag_start(&self, drag_state: DragStateCarrier<Column>, column: Column, event: DragEvent) {
        self.default_handler.drag_start(drag_state, column, event);
    }

    fn drag_end(&self, drag_state: DragStateCarrier<Column>, column: Column, event: DragEvent) {
        if let Some(drop_state) = drag_state.get() {
            self.drop_tx
                .send(drop_state)
                .expect("dnd channel died before eol.");
            self.trigger.notify();
        }
        self.default_handler.drag_end(drag_state, column, event);
    }

    fn get_drag_classes(
        &self,
        drag_state: DragStateCarrier<Column>,
        column: Column,
    ) -> Signal<String> {
        Signal::derive(move || {
            let Some(drag_state) = drag_state.get() else {
                return String::new();
            };
            if drag_state.hovering_over == column {
                return match drag_state.hovering_side {
                    DragSide::Left => String::from("border-l border-blue"),
                    DragSide::Right => String::from("border-r border-blue"),
                };
            }
            String::new()
        })
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let rows = vec![
            Book {
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1925, 4, 10).unwrap()),
                description: Some(
                    "A story of wealth, love, and the American Dream in the 1920s.".to_string(),
                ),
            },
            Book {
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1939, 4, 14).unwrap()),
                description: None,
            },
            Book {
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1949, 6, 8).unwrap()),
                description: None,
            },
            Book {
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1922, 2, 2).unwrap()),
                description: None,
            },
        ];
        let (drop_tx, drop_rx) = std::sync::mpsc::channel();
        let default_handler = DefaultHeadDragHandler {};
        let trigger = Trigger::new();
        let custom_handler = Arc::new(CustomHeadDragHandler {
            default_handler,
            drop_tx,
            trigger,
        });
        let columns = RwSignal::new(Vec::from(Book::columns()));

        // Drop watcher
        Effect::watch(
            move || {
                trigger.track();
            },
            move |_, _, _| {
                let drop_event = drop_rx
                    .try_recv()
                    .expect("trigger should only be called when drop_tx sent an event.");
                if drop_event.grabbed == drop_event.hovering_over {
                    return;
                }

                let mut cs = columns.get_untracked();
                let mut column_iter1 = cs.iter();

                if let Some(grab_pos) = column_iter1.position(|s| s == &drop_event.grabbed) {
                    cs.remove(grab_pos);
                    let mut column_iter2 = cs.iter();
                    let drop_pos = column_iter2
                        .position(|s| s == &drop_event.hovering_over)
                        .unwrap_or(grab_pos);

                    cs.insert(
                        match drop_event.hovering_side {
                            DragSide::Left => drop_pos,
                            DragSide::Right => std::cmp::min(drop_pos + 1, cs.len()),
                        },
                        drop_event.grabbed,
                    );
                }
                columns.set(cs);
            },
            false,
        );
        view! {
            <fieldset>
                <legend>Visible Columns:</legend>
                {Book::columns().iter().map(|book| view! {
                    <label>{ format!("{book:?}") }</label>
                    <input
                        type="checkbox"
                        checked
                        on:click=move |_| {
                            let mut columns_internal = columns.get();
                            if !columns_internal.contains(book) {
                                let idx = columns_internal.iter().filter(|c| *c < book).count();
                                columns_internal.insert(idx, *book);
                            } else {
                                columns_internal.retain(|c| c != book)
                            }
                            columns.set(columns_internal);
                        }/>
                    <br/>
                }).collect_view()}
            </fieldset>
            <div class="float-left m-10 rounded-md border dark:border-gray-700 overflow-clip">
                <table class="text-sm text-left text-gray-500 dark:text-gray-400 mb-[-1px]">
                    <TableContent rows scroll_container="html" columns drag_handler=custom_handler />
                </table>
            </div>
        }
    })
}

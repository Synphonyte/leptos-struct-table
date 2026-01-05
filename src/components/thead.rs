use std::sync::Arc;
use std::sync::mpsc::{self, Sender};

use crate::wrapper_render_fn;
use crate::{ColumnSort, TableHeadEvent};
use leptos::either::Either;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, HtmlTableCellElement};

wrapper_render_fn!(
    /// thead
    DefaultTableHeadRenderer,
    thead,
);

wrapper_render_fn!(
    /// thead row
    DefaultTableHeadRowRenderer,
    tr,
);

#[derive(Clone)]
pub struct DragManager<Column> {
    drag_handler: Option<Arc<dyn DragHandler<Column> + Send + Sync + 'static>>,
    drag_state: DragStateCarrier<Column>,
}

impl<Column: Send + Sync + 'static> DragManager<Column> {
    pub fn new(drag_handler: Option<Arc<dyn DragHandler<Column> + Send + Sync + 'static>>) -> Self {
        Self {
            drag_handler,
            drag_state: RwSignal::new(None),
        }
    }
}

/// Collection of event handlers needed to create a table-column dragging experience to reorder columns.
pub trait DragHandler<Column>: Send + Sync {
    /// Something fell out of the sky onto this column/table head element.
    fn received_drop(&self, drag_state: DragStateCarrier<Column>, column: Column, event: DragEvent);
    /// Something is above us still in the sky.
    fn dragging_over(&self, drag_state: DragStateCarrier<Column>, column: Column, event: DragEvent);
    /// Something exited our airspace.
    fn drag_leave(&self, drag_state: DragStateCarrier<Column>, column: Column, event: DragEvent);
    /// Started dragging column.
    fn drag_start(&self, drag_state: DragStateCarrier<Column>, column: Column, event: DragEvent);
    /// Dragging ended, could be that drop did not go off
    fn drag_end(&self, drag_state: DragStateCarrier<Column>, column: Column, event: DragEvent);

    /// Classes for columns.
    /// Intended to react to drag events to show highlights via classes.
    fn get_drag_classes(
        &self,
        drag_state: DragStateCarrier<Column>,
        column: Column,
    ) -> Signal<String>;
}

pub type DragStateCarrier<Column> = RwSignal<Option<DragState<Column>>>;
#[derive(Clone, PartialEq)]
pub struct DragState<Column> {
    /// Column which is being dragged.
    pub grabbed: Column,
    /// Last column the cursor was over
    pub hovering_over: Column,
    /// On which column side of [hovering_over] the cursor is on.
    /// Used for styling that side, e.g. as dropzone indicator.
    pub hovering_side: DragSide,
}

/// The side of a column the cursor is in while dragging another column over it.
/// Used for styling the matching side with a drop-zone highlight.
#[derive(Clone, Copy, PartialEq)]
pub enum DragSide {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct DefaultHeadDragHandler;

impl<Column: Clone + PartialEq + Send + Sync + 'static> DragHandler<Column>
    for DefaultHeadDragHandler
{
    fn received_drop(
        &self,
        drag_state: DragStateCarrier<Column>,
        _column: Column,
        _event: DragEvent,
    ) {
        drag_state.set(None);
    }

    fn dragging_over(
        &self,
        drag_state_carrier: DragStateCarrier<Column>,
        column: Column,
        event: DragEvent,
    ) {
        let Some(mut drag_state) = drag_state_carrier.get() else {
            return;
        };

        // Prevent default stop to allow drop.
        event.prevent_default();

        let hovering_side = if let Some(target) = event.target() {
            let Ok(thead) = target.dyn_into::<HtmlTableCellElement>() else {
                return;
            };
            let thead_rect = thead.get_bounding_client_rect();
            let thead_center_x = thead_rect.x() + thead_rect.width() / 2.0;
            let mouse_x = event.x();
            if (mouse_x as f64) < thead_center_x {
                DragSide::Left
            } else {
                DragSide::Right
            }
        } else {
            // fallback
            DragSide::Left
        };

        // Update state when the state changed.
        if drag_state.hovering_over != column || drag_state.hovering_side != hovering_side {
            drag_state.hovering_over = column;
            drag_state.hovering_side = hovering_side;
            drag_state_carrier.update(|mut_drag_state| {
                *mut_drag_state = Some(drag_state);
            });
        }
    }

    fn drag_leave(
        &self,
        _drag_state: DragStateCarrier<Column>,
        _column: Column,
        _event: DragEvent,
    ) {
    }

    fn drag_start(&self, drag_state: DragStateCarrier<Column>, column: Column, _event: DragEvent) {
        drag_state.set(Some(DragState {
            grabbed: column.clone(),
            hovering_over: column,
            hovering_side: DragSide::Left,
        }));
    }

    fn drag_end(&self, drag_state: DragStateCarrier<Column>, _column: Column, _event: DragEvent) {
        drag_state.set(None);
    }

    /// Compares column against the internal hovered element, if they match then css classes will be returned
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

/// The default table header renderer. Renders roughly
/// ```html
/// <th>
///    <span>Title</span>
/// </th>
/// ```
#[component]
pub fn DefaultTableHeaderCellRenderer<F, Column>(
    /// The class attribute for the head element. Generated by the classes provider.
    #[prop(into)]
    class: Signal<String>,
    /// The class attribute for the inner element. Generated by the classes provider.
    #[prop(into)]
    inner_class: String,
    /// The index of the column.
    /// Information on column indexes is available at: the [Column index type](crate#column-index-type) section.
    index: Column,
    /// The sort priority of the column. `None` if the column is not sorted. `0` means the column is the primary sort column.
    #[prop(into)]
    sort_priority: Signal<Option<usize>>,
    /// The sort direction of the column. See [`ColumnSort`].
    #[prop(into)]
    sort_direction: Signal<ColumnSort>,
    /// The event handler for the click event. Has to be called with [`TableHeadEvent`].
    on_click: F,
    /// Drag handlers + state
    drag_manager: DragManager<Column>,
    children: Children,
) -> impl IntoView
where
    F: Fn(TableHeadEvent<Column>) + 'static,
    Column: Copy + Send + Sync + 'static,
{
    let style = default_th_sorting_style(sort_priority, sort_direction);

    let drag_handler_attributes = if let Some(drag_handler) = drag_manager.drag_handler {
        let drag_state = drag_manager.drag_state;
        let drag_classes = drag_handler.get_drag_classes(drag_state, index);

        #[derive(Copy, Clone)]
        enum DragEventKind {
            Drop,
            Over,
            Leave,
            Start,
            End,
        }

        // Channel with trigger to make a reactive channel.
        let (tx, rx) = mpsc::channel();
        let trigger = Trigger::new();

        Effect::new(move || {
            trigger.track();
            if let Ok((event_type, drag_event)) = rx.try_recv() {
                match event_type {
                    DragEventKind::Drop => {
                        drag_handler.received_drop(drag_state, index, drag_event);
                    }
                    DragEventKind::Over => {
                        drag_handler.dragging_over(drag_state, index, drag_event);
                    }
                    DragEventKind::Leave => {
                        drag_handler.drag_leave(drag_state, index, drag_event);
                    }
                    DragEventKind::Start => {
                        drag_handler.drag_start(drag_state, index, drag_event);
                    }
                    DragEventKind::End => {
                        drag_handler.drag_end(drag_state, index, drag_event);
                    }
                }
            }
        });

        let create_event_transmitter = |tx: Sender<(DragEventKind, DragEvent)>, kind| {
            move |drag_event| {
                tx.send((kind, drag_event)).expect("msg");
                trigger.notify();
            }
        };

        Either::Left(view! {
            <{..}
                class=move || format!("{} {}", class.get(), drag_classes.get())
                on:drop=create_event_transmitter(tx.clone(), DragEventKind::Drop)
                on:dragover=create_event_transmitter(tx.clone(), DragEventKind::Over)
                on:dragleave=create_event_transmitter(tx.clone(), DragEventKind::Leave)
                on:dragstart=create_event_transmitter(tx.clone(), DragEventKind::Start)
                on:dragend=create_event_transmitter(tx.clone(), DragEventKind::End)
                draggable="true"
            />
        })
    } else {
        Either::Right(view! {
            <{..}
                class=move || class.get()
            />
        })
    };

    view! {
        <th
            style=style
            on:click=move |mouse_event| on_click(TableHeadEvent {
                index,
                mouse_event,
            })
            {..drag_handler_attributes}
        >
            <span class=inner_class>
                {children()}
            </span>
        </th>
    }
}

/// You can use this function to implement your own custom table header cell renderer.
///
/// See the implementation of [`DefaultTableHeaderCellRenderer`].
pub fn default_th_sorting_style(
    sort_priority: Signal<Option<usize>>,
    sort_direction: Signal<ColumnSort>,
) -> Signal<String> {
    Signal::derive(move || {
        let sort = match sort_direction.get() {
            ColumnSort::Ascending => "--sort-icon: '▲';",
            ColumnSort::Descending => "--sort-icon: '▼';",
            ColumnSort::None => "--sort-icon: '';",
        };

        let priority = match sort_priority.get() {
            Some(priority) => format!("--sort-priority: '{}';", priority + 1),
            None => "--sort-priority: '';".to_string(),
        };

        format!("{} {}", sort, &priority)
    })
}

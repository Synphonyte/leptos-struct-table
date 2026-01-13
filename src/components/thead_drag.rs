use std::{marker::PhantomData, sync::Arc};

use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct HeadDragHandler<Column>(pub(crate) Arc<dyn DragHandler<Column> + Send + Sync + 'static>);

impl<Column> HeadDragHandler<Column>
where
    Column: Clone + PartialEq + Send + Sync + 'static,
{
    pub fn new<H>(handler: H) -> Self
    where
        H: DragHandler<Column> + Send + Sync + 'static,
    {
        Self(Arc::new(handler))
    }
}

impl<Column> Default for HeadDragHandler<Column>
where
    Column: Clone + PartialEq + Send + Sync + 'static,
{
    fn default() -> Self {
        Self(Arc::new(DefaultDragHandler::<Column>::default()))
    }
}

/// Collection of event handlers needed to create a table-column dragging experience to reorder columns.
pub trait DragHandler<Column>: Send + Sync
where
    Column: Clone + PartialEq + Send + Sync + 'static,
{
    /// Cursor is above **column** and dropped the column it was dragging.
    fn received_drop(
        &self,
        drag_state: DragStateRwSignal<Column>,
        columns: RwSignal<Vec<Column>>,
        _column: Column,
        _event: web_sys::DragEvent,
    ) {
        drag_state.update(|drag_state| {
            if let Some(drag_state) = drag_state.take() {
                columns.update(|columns| drag_state.reorder_columns(columns))
            }
        });
    }

    /// Cursor is moving above **column** while dragging.
    fn dragging_over(
        &self,
        drag_state_carrier: DragStateRwSignal<Column>,
        column: Column,
        event: web_sys::DragEvent,
    ) {
        let Some(mut drag_state) = drag_state_carrier.get() else {
            return;
        };

        // Prevent default stop to allow drop.
        event.prevent_default();

        let hovering_side = if let Some(target) = event.target() {
            let Ok(thead) = target.dyn_into::<web_sys::HtmlTableCellElement>() else {
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
            *drag_state_carrier.write() = Some(drag_state);
        }
    }

    /// Cursor moves out of **column**
    fn drag_leave(
        &self,
        _drag_state: DragStateRwSignal<Column>,
        _column: Column,
        _event: web_sys::DragEvent,
    ) {
        // by default do nothing
    }

    /// Started dragging **column**.
    fn drag_start(
        &self,
        drag_state: DragStateRwSignal<Column>,
        column: Column,
        _event: web_sys::DragEvent,
    ) {
        drag_state.set(Some(DragState {
            grabbed: column.clone(),
            hovering_over: column,
            hovering_side: DragSide::Left,
        }));
    }

    /// Dragging ended.
    fn drag_end(
        &self,
        drag_state: DragStateRwSignal<Column>,
        columns: RwSignal<Vec<Column>>,
        _column: Column,
        _event: web_sys::DragEvent,
    ) {
        drag_state.update(|drag_state| {
            if let Some(drag_state) = drag_state.take() {
                columns.update(|columns| drag_state.reorder_columns(columns))
            }
        });
    }

    /// Classes for columns.
    /// Intended to react to drag events to show highlights via classes.
    fn get_drag_classes(
        &self,
        drag_state: DragStateRwSignal<Column>,
        column: Column,
        columns: RwSignal<Vec<Column>>,
    ) -> Signal<String> {
        let grabbed_class = self.grabbed_class();
        let hover_left_class = self.hover_left_class();
        let hover_right_class = self.hover_right_class();

        Signal::derive(move || {
            let Some(drag_state) = drag_state.get() else {
                return String::new();
            };

            if drag_state.grabbed == column {
                grabbed_class.to_string()
            } else if drag_state.hovering_over == column {
                let mut resorted_cols = columns.get();

                drag_state.reorder_columns(&mut resorted_cols);

                if &resorted_cols == &*columns.read() {
                    String::new()
                } else {
                    match drag_state.hovering_side {
                        DragSide::Left => hover_left_class.to_string(),
                        DragSide::Right => hover_right_class.to_string(),
                    }
                }
            } else {
                String::new()
            }
        })
    }

    fn grabbed_class(&self) -> &'static str {
        "grabbed"
    }

    fn hover_left_class(&self) -> &'static str {
        "hover-left"
    }

    fn hover_right_class(&self) -> &'static str {
        "hover-right"
    }
}

#[derive(Copy, Clone)]
pub struct DefaultDragHandler<C>(PhantomData<C>);

impl<C> DragHandler<C> for DefaultDragHandler<C> where C: Clone + PartialEq + Send + Sync + 'static {}

impl<C> Default for DefaultDragHandler<C>
where
    C: Clone + PartialEq + Send + Sync + 'static,
{
    fn default() -> Self {
        DefaultDragHandler(PhantomData)
    }
}

pub type DragStateRwSignal<Column> = RwSignal<Option<DragState<Column>>>;

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

impl<Column> DragState<Column>
where
    Column: Clone + PartialEq,
{
    pub fn reorder_columns(&self, columns: &mut Vec<Column>) {
        let index = columns
            .iter()
            .position(|c| *c == self.hovering_over)
            .unwrap();
        let grabbed_index = columns.iter().position(|c| *c == self.grabbed).unwrap();

        if grabbed_index == index {
            return;
        }

        columns.remove(grabbed_index);

        let mut index = match self.hovering_side {
            DragSide::Left => index,
            DragSide::Right => index + 1,
        };
        if index > grabbed_index {
            index -= 1;
        }

        columns.insert(index, self.grabbed.clone());
    }
}

/// The side of a column the cursor is in while dragging another column over it.
/// Used for styling the matching side with a drop-zone highlight.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DragSide {
    Left,
    Right,
}

use leptos::ev::MouseEvent;
use std::rc::Rc;

/// The event provided to the `on_change` prop of the table component
#[derive(Debug, Clone)]
pub struct ChangeEvent<Row: Clone> {
    /// The index of the table row that contains the cell that was changed. Starts at 0.
    pub row_index: usize,
    /// The index of the table column that contains the cell that was changed. Starts at 0.
    pub col_index: usize,
    /// The the row that was changed.
    pub changed_row: Row,
}

/// The event provided to the `on_selection_change` prop of the table component
#[derive(Debug, Clone)]
pub struct SelectionChangeEvent<Row: Clone> {
    /// `true` is the row was selected, `false` if it was de-selected.
    pub selected: bool,
    /// The index of the row that was de-/selected.
    pub row_index: usize,
    /// The row that was de-/selected.
    pub row: Row,
}

/// Event emitted when a table head cell is clicked.
#[derive(Debug)]
pub struct TableHeadEvent {
    /// The index of the column. Starts at 0 for the first column.
    /// The order of the columns is the same as the order of the fields in the struct.
    pub index: usize,
    /// The mouse event that triggered the event.
    pub mouse_event: MouseEvent,
}

macro_rules! impl_default_rc_fn {
    (
        $(#[$meta:meta])*
        $name:ident<$($ty:ident),*>($($arg_name:ident: $arg_ty:ty),*)
        $(-> $ret_ty:ty)?
        $({ default $default_return:expr })?
    ) => {
        $(#[$meta])*
        #[derive(Clone)]
        pub struct $name<$($ty),*>(Rc<dyn Fn($($arg_ty),*) $(-> $ret_ty)?>);

        impl<$($ty),*> Default for $name<$($ty),*> {
            fn default() -> Self {
                #[allow(unused_variables)]
                Self(Rc::new(|$($arg_name: $arg_ty),*| {
                    $($default_return)?
                }))
            }
        }

        impl<F, $($ty),*> From<F> for $name<$($ty),*>
            where F: Fn($($arg_ty),*) $(-> $ret_ty)? + 'static
        {
            fn from(f: F) -> Self { Self(Rc::new(f)) }
        }

        impl<$($ty),*> $name<$($ty),*> {
            pub fn run(&self, $($arg_name: $arg_ty),*) $(-> $ret_ty)? {
                (self.0)($($arg_name),*)
            }
        }
    }
}

impl_default_rc_fn!(
    /// New type wrapper of a closure that takes a parameter `T`. This allows the event handler props
    /// to be optional while being able to take a simple closure.
    EventHandler<T>(event: T)
);

pub(crate) use impl_default_rc_fn;

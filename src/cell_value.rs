use std::borrow::Cow;

use leptos::{view, Fragment, IntoView, View};

#[derive(Default)]
pub struct NumberRenderOptions {
    /// Specifies the number of digits to display after the decimal point
    pub precision: Option<usize>,
}

/// A value that can be rendered as part of a table, required for types if the [`crate::DefaultTableCellRenderer()`] is used
pub trait CellValue {
    /// Formatting options for this cell value type, needs to implement default and have public named fields,
    /// the empty tuple: () is fine if no formatting options can be accepted.
    type RenderOptions: Default;

    /// This is called to actually render the value. The parameter `options` is filled by the `#[table(format(...))]` macro attribute or `Default::default()` if omitted.
    fn render_value(self, options: &Self::RenderOptions) -> impl IntoView;
}

macro_rules! viewable_identity {
    ($($ty:ty),* $(,)?) => {
        $(
            impl CellValue for $ty {
                type RenderOptions = ();

                fn render_value(self, _options: &Self::RenderOptions) -> impl IntoView {
                    self
                }
            }
        )*
    };
}

viewable_identity![String, &'static str, Cow<'static, str>, View, Fragment];

macro_rules! viewable_primitive {
  ($($child_type:ty),* $(,)?) => {
    $(
      impl CellValue for $child_type {
        type RenderOptions = ();

        #[inline(always)]
        fn render_value(self, _options: &Self::RenderOptions) -> impl IntoView {
            self.to_string()
        }
      }
    )*
  };
}

viewable_primitive![
    &String,
    char,
    bool,
    std::net::IpAddr,
    std::net::SocketAddr,
    std::net::SocketAddrV4,
    std::net::SocketAddrV6,
    std::net::Ipv4Addr,
    std::net::Ipv6Addr,
    std::char::ToUppercase,
    std::char::ToLowercase,
    std::num::NonZeroI8,
    std::num::NonZeroU8,
    std::num::NonZeroI16,
    std::num::NonZeroU16,
    std::num::NonZeroI32,
    std::num::NonZeroU32,
    std::num::NonZeroI64,
    std::num::NonZeroU64,
    std::num::NonZeroI128,
    std::num::NonZeroU128,
    std::num::NonZeroIsize,
    std::num::NonZeroUsize,
    std::panic::Location<'_>,
];

macro_rules! viewable_number_primitive {
  ($($child_type:ty),* $(,)?) => {
    $(
      impl CellValue for $child_type {
        type RenderOptions = NumberRenderOptions;

        #[inline(always)]
        fn render_value(self, options: &Self::RenderOptions) -> impl IntoView {
        if let Some(value) = options.precision.as_ref() {
            view! {
                <>{format!("{:.value$}", self)}</>
            }
        }
        else {
            view! {
                <>{self.to_string()}</>
            }
        }
        }
      }
    )*
  };
}

viewable_number_primitive![
    usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64,
];

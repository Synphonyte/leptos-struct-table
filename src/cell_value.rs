use std::borrow::Cow;

use leptos::{view, IntoView};

#[derive(Default)]
pub struct RenderOptions {
    pub format_string: Option<String>,
    pub precision: Option<usize>,
}
// A value that can be rendered as part of a table
pub trait CellValue {
    fn render_value(self, options: &RenderOptions) -> impl IntoView;
}

impl CellValue for String {
    fn render_value(self, _options: &RenderOptions) -> impl IntoView {
        view! {
            <>{self}</>
        }
    }
}

impl CellValue for &'static str {
    fn render_value(self, _options: &RenderOptions) -> impl IntoView {
        view! {
            <>{self}</>
        }
    }
}
impl CellValue for Cow<'static, str> {
    fn render_value(self, _options: &RenderOptions) -> impl IntoView {
        view! {
            <>{self}</>
        }
    }
}

macro_rules! viewable_primitive {
  ($($child_type:ty),* $(,)?) => {
    $(
      impl CellValue for $child_type {
        #[inline(always)]
        fn render_value(self, _options: &RenderOptions) -> impl IntoView {
            view! {
                <>{self.to_string()}</>
            }
        }
      }
    )*
  };
}
macro_rules! viewable_number_primitive {
  ($($child_type:ty),* $(,)?) => {
    $(
      impl CellValue for $child_type {
        #[inline(always)]
        fn render_value(self, options: &RenderOptions) -> impl IntoView {
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

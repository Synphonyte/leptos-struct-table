//! Support for [uuid::Uuid] type.

use leptos::*;
use serde::{Deserialize, Serialize};

/// Newtype for [uuid::Uuid].
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize, Hash, Eq)]
pub struct Uuid(uuid::Uuid);

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for Uuid {
    type Err = <uuid::Uuid as TryFrom<&'static str>>::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::parse_str(s)?))
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl core::ops::Deref for Uuid {
    type Target = uuid::Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoView for Uuid {
    fn into_view(self, cx: Scope) -> View {
        view! {
            cx,
            <>{format!("{:?}", self.0)}</>
        }
        .into()
    }
}

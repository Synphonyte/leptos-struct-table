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
    fn into_view(self) -> View {
        view! {

            <>{format!("{:?}", self.0)}</>
        }
        .into()
    }
}

#[cfg(feature = "sqlx_postgres")]
impl sqlx::Type<sqlx::Postgres> for Uuid {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <uuid::Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx_postgres")]
impl sqlx::postgres::PgHasArrayType for Uuid {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <uuid::Uuid as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}

#[cfg(feature = "sqlx_postgres")]
impl sqlx::Encode<'_, sqlx::Postgres> for Uuid {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        <uuid::Uuid as sqlx::Encode<'_, sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}

#[cfg(feature = "sqlx_postgres")]
impl sqlx::Decode<'_, sqlx::Postgres> for Uuid {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        <uuid::Uuid as sqlx::Decode<'_, sqlx::Postgres>>::decode(value).map(|i| Uuid(i))
    }
}

#[cfg(feature = "sqlx_mysql")]
impl sqlx::Type<sqlx::MySql> for Uuid {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        <uuid::Uuid as sqlx::Type<sqlx::MySql>>::type_info()
    }

    fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
        <uuid::Uuid as sqlx::Type<sqlx::MySql>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx_mysql")]
impl sqlx::Encode<'_, sqlx::MySql> for Uuid {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> sqlx::encode::IsNull {
        <uuid::Uuid as sqlx::Encode<'_, sqlx::MySql>>::encode_by_ref(&self.0, buf)
    }
}

#[cfg(feature = "sqlx_mysql")]
impl sqlx::Decode<'_, sqlx::MySql> for Uuid {
    fn decode(value: sqlx::mysql::MySqlValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        <uuid::Uuid as sqlx::Decode<'_, sqlx::MySql>>::decode(value).map(|i| Uuid(i))
    }
}

#[cfg(feature = "sqlx_sqlite")]
impl sqlx::Type<sqlx::Sqlite> for Uuid {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <uuid::Uuid as sqlx::Type<sqlx::Sqlite>>::type_info()
    }

    fn compatible(ty: &sqlx::sqlite::SqliteTypeInfo) -> bool {
        <uuid::Uuid as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx_sqlite")]
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for Uuid {
    fn encode_by_ref(
        &self,
        args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>,
    ) -> sqlx::encode::IsNull {
        <uuid::Uuid as sqlx::Encode<'_, sqlx::Sqlite>>::encode_by_ref(&self.0, args)
    }
}

#[cfg(feature = "sqlx_sqlite")]
impl sqlx::Decode<'_, sqlx::Sqlite> for Uuid {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        <uuid::Uuid as sqlx::Decode<'_, sqlx::Sqlite>>::decode(value).map(|i| Uuid(i))
    }
}

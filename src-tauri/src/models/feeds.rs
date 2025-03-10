use enum_stringify::EnumStringify;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, prelude::*, serialize::{Output, ToSql}, sql_types::Text, sqlite::{Sqlite, SqliteValue}};

////////////////////////////////////////////////////////////////////////////////

// The article below explains the traits needed for using enums with diesel:
// https://www.matsimitsu.com/blog/2024-06-23-rust-enums-in-sqlite-with-diesel

#[derive(Serialize, specta::Type)]
#[derive(Debug, EnumStringify, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub enum FeedStatus {
  Subscribed,
  Unsubscribed
}

impl FromSql<Text, Sqlite> for FeedStatus {
  fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
      let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
      Ok(t.as_str().try_into()?)
  }
}

impl ToSql<Text, Sqlite> for FeedStatus {
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
      out.set_value(self.to_string());
      Ok(diesel::serialize::IsNull::No)
  }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, specta::Type)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::feeds)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Feed {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub status: FeedStatus,
    pub checked_at: NaiveDateTime,
    pub fetch_old_items: bool
}

#[derive(Deserialize, specta::Type)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::feeds)]
pub struct CreateFeed {
  pub title: String,
  pub url: String,
  pub fetch_old_items: bool,
}
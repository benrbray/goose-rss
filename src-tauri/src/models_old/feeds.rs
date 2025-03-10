// use core::fmt;
// use std::fmt::{Display, Formatter};
// use std::str::FromStr;

// use chrono::{DateTime, FixedOffset, Utc};
// use rusqlite::{Connection, Row};
// use sea_query::{Expr, Query, SqliteQueryBuilder};
// use sea_query_rusqlite::RusqliteBinder;
// use serde::{Deserialize, Serialize};

// use crate::error::{Error, Result};

// use crate::models_old::database::Feeds;

// ////////////////////////////////////////////////////////////////////////////////

// #[derive(Serialize, Deserialize, specta::Type, Debug, PartialEq)]
// pub enum FeedStatus {
//   Subscribed,
//   Unsubscribed,
// }

// impl Display for FeedStatus {
//   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//     match self {
//       FeedStatus::Subscribed => write!(f, "subscribed"),
//       FeedStatus::Unsubscribed => write!(f, "unsubscribed"),
//     }
//   }
// }

// impl FromStr for FeedStatus {
//   type Err = Error;

//   fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
//     match x {
//       "subscribed" => Ok(Self::Subscribed),
//       "unsubscribed" => Ok(Self::Unsubscribed),
//       _ => Err(Error::InvalidEnumKey(
//         x.to_string(),
//         "FeedStatus".to_string(),
//       )),
//     }
//   }
// }

// #[derive(Serialize, Debug, specta::Type)]
// pub struct Feed {
//   pub id: i32,
//   pub title: String,
//   pub link: String,
//   pub status: FeedStatus,
//   pub checked_at: DateTime<FixedOffset>,
//   pub fetch_old_items: bool,
// }

// impl From<&Row<'_>> for Feed {
//   fn from(row: &Row) -> Self {
//     Self {
//       id: row.get_unwrap("id"),
//       title: row.get_unwrap("title"),
//       link: row.get_unwrap("link"),
//       status: FeedStatus::from_str(&row.get_unwrap::<&str, String>("status")).unwrap(),
//       checked_at: row.get_unwrap("checked_at"),
//       fetch_old_items: row.get_unwrap("fetch_old_items"),
//     }
//   }
// }

// ////////////////////////////////////////////////////////////////////////////////

// #[derive(Deserialize, specta::Type)]
// pub struct FeedToCreate {
//   pub title: String,
//   pub link: String,
//   pub fetch_old_items: bool,
// }

// pub fn create(db: &Connection, arg: &FeedToCreate) -> Result<usize> {
//   let (sql, values) = Query::insert()
//     .into_table(Feeds::Table)
//     .columns([
//       Feeds::Title,
//       Feeds::Link,
//       Feeds::CheckedAt,
//       Feeds::FetchOldItems,
//     ])
//     .values_panic([
//       (*arg.title).into(),
//       (*arg.link).into(),
//       Utc::now().into(),
//       arg.fetch_old_items.into(),
//     ])
//     .build_rusqlite(SqliteQueryBuilder);

//   Ok(db.execute(sql.as_str(), &*values.as_params())?)
// }

// pub fn read_all(db: &Connection) -> Result<Vec<Feed>> {
//   let (sql, values) = Query::select()
//     .columns([
//       Feeds::Id,
//       Feeds::Title,
//       Feeds::Link,
//       Feeds::Status,
//       Feeds::CheckedAt,
//       Feeds::FetchOldItems,
//     ])
//     .from(Feeds::Table)
//     .build_rusqlite(SqliteQueryBuilder);

//   let mut stmt = db.prepare(sql.as_str())?;
//   let rows = stmt.query_map(&*values.as_params(), |x| Ok(Feed::from(x)))?;

//   Ok(rows.map(std::result::Result::unwrap).collect::<Vec<Feed>>())
// }

// pub fn read(db: &Connection, id: i32) -> Result<Option<Feed>> {
//   let (sql, values) = Query::select()
//     .columns([
//       Feeds::Id,
//       Feeds::Title,
//       Feeds::Link,
//       Feeds::Status,
//       Feeds::CheckedAt,
//       Feeds::FetchOldItems,
//     ])
//     .from(Feeds::Table)
//     .and_where(Expr::col(Feeds::Id).eq(id))
//     .limit(1)
//     .build_rusqlite(SqliteQueryBuilder);

//   let mut stmt = db.prepare(sql.as_str())?;
//   let mut rows = stmt.query(&*values.as_params())?;

//   Ok(rows.next()?.map(Feed::from))
// }

// pub fn delete(db: &Connection, id: i32) -> Result<usize> {
//   let (sql, values) = Query::delete()
//     .from_table(Feeds::Table)
//     .and_where(Expr::col(Feeds::Id).eq(id))
//     .build_rusqlite(SqliteQueryBuilder);

//   Ok(db.execute(sql.as_str(), &*values.as_params())?)
// }

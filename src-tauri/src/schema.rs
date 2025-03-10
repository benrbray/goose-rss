diesel::table! {
  feeds (id) {
    id -> Integer,
    title -> Text,
    url -> Text,
    status -> Text,
    checked_at -> Timestamp,
    fetch_old_items -> Bool
  }
}
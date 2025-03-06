use serde::Deserialize;

use crate::error::Result;

#[derive(Deserialize)]
pub struct FeedToCreate {
    pub title: String,
    pub link: String,
    pub fetch_old_items: bool,
}

pub fn create(data: &FeedToCreate) -> Result<()> {
  
  
  Ok(())
}
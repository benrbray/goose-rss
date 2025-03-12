use crate::error::Result;

// #[cfg(test)]
// pub fn fetch_content(link: &str) -> Result<String> {
//     use std::fs;
//     Ok(fs::read_to_string(link)?)
// }

// #[cfg(not(test))]
pub async fn fetch_content(link: &str) -> Result<String> {
  // collie-reader configures the HTTP client with an optional proxy setting,
  // but I have no need for this feature so I just use the default client
  let client = reqwest::Client::new();
  let result = client
    .get(link)
    .header("User-Agent", "Mozilla/5.0")
    .send()
    .await?;

  let x = result.text().await?;

  return Ok(x);
}
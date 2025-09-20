use crate::browser::Browser;

/// Modul scraping logika
pub struct Scraper;

impl Scraper {
    pub fn new() -> Self {
        Self {}
    }

    /// Ambil trending hashtags dari halaman Discover TikTok
    pub async fn extract_trending_tags(
        &self,
        browser: &mut Browser,
    ) -> Result<Vec<String>, fantoccini::error::CmdError> {
        let mut tags = Vec::new();

        let elements = browser.find_all("a[href*='/tag/']").await?;
        for el in elements {
            if let Ok(text) = el.text().await {
                if !text.trim().is_empty() {
                    tags.push(text);
                }
            }
        }

        Ok(tags)
    }
}

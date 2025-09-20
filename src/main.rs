mod browser;
mod scraper;
mod storage;

use crate::browser::Browser;
use crate::scraper::Scraper;
use crate::storage::Storage;

#[tokio::main]
async fn main() {
    // Inisialisasi browser
    let mut browser = match Browser::connect("http://localhost:9515").await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("❌ Gagal konek ke browser: {}", e);
            return;
        }
    };

    // Buka halaman discover TikTok
    if let Err(e) = browser.goto("https://www.tiktok.com/discover").await {
        eprintln!("❌ Gagal buka halaman: {}", e);
        return;
    }

    // Scraping trending tags
    let scraper = Scraper::new();
    let tags = match scraper.extract_trending_tags(&mut browser).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("❌ Error scraping: {}", e);
            return;
        }
    };

    println!("✅ Trending tags ditemukan:");
    for tag in &tags {
        println!("- {}", tag);
    }

    // Simpan hasil ke JSON
    let storage = Storage::new("trending.json");
    if let Err(e) = storage.save(&tags) {
        eprintln!("❌ Gagal simpan file: {}", e);
    } else {
        println!("💾 Hasil tersimpan di trending.json");
    }

    // Tutup browser
    let _ = browser.close().await;
}

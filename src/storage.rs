use serde::Serialize;
use std::fs::File;
use std::io::Write;

/// Modul penyimpanan hasil scraping
pub struct Storage {
    filename: String,
}

impl Storage {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }

    /// Simpan vector string ke file JSON
    pub fn save(&self, data: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct TrendingData<'a> {
            tags: &'a Vec<String>,
        }

        let wrapped = TrendingData { tags: data };
        let json = serde_json::to_string_pretty(&wrapped)?;

        let mut file = File::create(&self.filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

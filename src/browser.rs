use fantoccini::{Client, Locator};

/// Abstraksi untuk komunikasi dengan WebDriver
pub struct Browser {
    client: Client,
}

impl Browser {
    /// Hubungkan ke WebDriver (chromedriver / geckodriver)
    pub async fn connect(url: &str) -> Result<Self, fantoccini::error::CmdError> {
        let client = Client::new(url).await?;
        Ok(Self { client })
    }

    /// Buka URL tertentu
    pub async fn goto(&mut self, url: &str) -> Result<(), fantoccini::error::CmdError> {
        self.client.goto(url).await
    }

    /// Cari semua elemen dengan selector CSS
    pub async fn find_all(
        &mut self,
        selector: &str,
    ) -> Result<Vec<fantoccini::elements::Element>, fantoccini::error::CmdError> {
        self.client.find_all(Locator::Css(selector)).await
    }

    /// Tutup sesi browser
    pub async fn close(mut self) -> Result<(), fantoccini::error::CmdError> {
        self.client.close().await
    }
}

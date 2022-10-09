use std::error::Error;

use crate::data::json::PhotonFeatureCollection;
use crate::data::PhotonFeature;

pub struct Client<'a> {
    base_url: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(base_url: Option<&str>) -> Client {
        if let Some(base_url) = base_url {
            // TODO remove trailing slash (/)
            return Client { base_url: base_url };
        }
        Client {
            base_url: "https://photon.komoot.io",
        }
    }

    #[tokio::main]
    pub async fn forward_search(self, query: &str) -> Result<Vec<PhotonFeature>, Box<dyn Error>> {
        let url = String::from(self.base_url) + "/api?q=" + query;
        // println!("URL is {url}");

        let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;

        let features: Vec<PhotonFeature> =
            serde_json::from_value::<PhotonFeatureCollection>(response)?
                .features()
                .into_iter()
                .map(|f| f.into())
                .collect();

        // println!("{:#?}", features);

        Ok(features)
    }
}

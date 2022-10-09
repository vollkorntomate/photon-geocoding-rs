use std::error::Error;

use reqwest::Client as ReqwestClient;

use crate::data::json::PhotonFeatureCollection;
use crate::data::{LatLon, PhotonFeature};

type PhotonResult = Result<Vec<PhotonFeature>, Box<dyn Error>>;

pub struct Client {
    forward_url: String,
    reverse_url: String,
    reqwest_client: ReqwestClient,
}

impl Default for Client {
    fn default() -> Self {
        Client::new(&"https://photon.komoot.io")
    }
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        // TODO remove trailing slash (/)
        Client {
            forward_url: String::from(base_url) + "/api",
            reverse_url: String::from(base_url) + "/reverse",
            reqwest_client: ReqwestClient::new(),
        }
    }

    #[tokio::main]
    pub async fn forward_search(self, query: &str) -> PhotonResult {
        let request = self
            .reqwest_client
            .get(&self.forward_url)
            .query(&[("q", query)]);

        let response = request.send().await?.json::<serde_json::Value>().await?;

        self.parse_response(response)
    }

    #[tokio::main]
    pub async fn reverse_search(self, coords: LatLon) -> PhotonResult {
        let request = self
            .reqwest_client
            .get(&self.reverse_url)
            .query(&[("lon", coords.lon), ("lat", coords.lat)]);

        let response = request.send().await?.json::<serde_json::Value>().await?;

        self.parse_response(response)
    }

    fn parse_response(self, response: serde_json::Value) -> PhotonResult {
        let features: Vec<PhotonFeature> =
            serde_json::from_value::<PhotonFeatureCollection>(response)?
                .features()
                .into_iter()
                .map(PhotonFeature::from)
                .collect();

        Ok(features)
    }
}

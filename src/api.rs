use std::error::Error;

use reqwest::Client as ReqwestClient;

use crate::data::json::PhotonFeatureCollection;
use crate::data::{LatLon, PhotonFeature};

type PhotonResult = Result<Vec<PhotonFeature>, Box<dyn Error>>;

pub struct Client {
    base_url: String,
    reqwest_client: ReqwestClient,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            base_url: String::from("https://photon.komoot.io"),
            reqwest_client: ReqwestClient::new(),
        }
    }
}

impl Client {
    pub fn new(base_url: Option<&str>) -> Self {
        let req_client = ReqwestClient::new();
        if let Some(base_url) = base_url {
            // TODO remove trailing slash (/)
            return Client {
                base_url: String::from(base_url),
                reqwest_client: req_client,
            };
        } else {
            Self::default()
        }
    }

    #[tokio::main]
    pub async fn forward_search(self, query: &str) -> PhotonResult {
        let url = self.base_url + "/api";

        let request = self
            .reqwest_client
            .get(url)
            .query(&[("q", query)]);

        let response = request.send().await?.json::<serde_json::Value>().await?;

        let features: Vec<PhotonFeature> =
            serde_json::from_value::<PhotonFeatureCollection>(response)?
                .features()
                .into_iter()
                .map(PhotonFeature::from)
                .collect();

        Ok(features)
    }

    #[tokio::main]
    pub async fn reverse_search(self, coords: LatLon) -> PhotonResult {
        let url = self.base_url + "/reverse";
        let request = self
            .reqwest_client
            .get(url)
            .query(&[("lon", coords.lon), ("lat", coords.lat)]);

        let response = request.send().await?.json::<serde_json::Value>().await?;

        let features: Vec<PhotonFeature> =
            serde_json::from_value::<PhotonFeatureCollection>(response)?
                .features()
                .into_iter()
                .map(PhotonFeature::from)
                .collect();

        Ok(features)
    }
}

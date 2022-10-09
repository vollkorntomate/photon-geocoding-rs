use std::error::Error;

use reqwest::{Client as ReqwestClient, RequestBuilder};

use crate::data::filter::{ForwardFilter, ReverseFilter};
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
    pub async fn forward_search(&self, query: &str, filter: Option<ForwardFilter>) -> PhotonResult {
        let mut request = self
            .reqwest_client
            .get(&self.forward_url)
            .query(&[("q", query)]);

        if let Some(filter) = filter {
            request = filter.append_to(request);
        }

        let response = request.send().await?.json::<serde_json::Value>().await?;

        self.parse_response(response)
    }

    #[tokio::main]
    pub async fn reverse_search(
        &self,
        coords: LatLon,
        filter: Option<ReverseFilter>,
    ) -> PhotonResult {
        let mut request = self
            .reqwest_client
            .get(&self.reverse_url)
            .query(&[("lon", coords.lon), ("lat", coords.lat)]);

        if let Some(filter) = filter {
            request = filter.append_to(request)
        }

        let response = request.send().await?.json::<serde_json::Value>().await?;

        self.parse_response(response)
    }

    fn parse_response(&self, response: serde_json::Value) -> PhotonResult {
        let features: Vec<PhotonFeature> =
            serde_json::from_value::<PhotonFeatureCollection>(response)?
                .features()
                .into_iter()
                .map(PhotonFeature::from)
                .collect();

        Ok(features)
    }
}

pub trait RequestAppend {
    fn append_to(self, request: RequestBuilder) -> RequestBuilder;
}

impl RequestAppend for ForwardFilter {
    fn append_to(self, request: RequestBuilder) -> RequestBuilder {
        let mut request = request;
        if let Some(bias) = self.location_bias {
            request = request.query(&[("lat", bias.lat), ("lon", bias.lon)]);
        }
        if let Some(bbox) = self.bounding_box {
            let format = format!(
                "{},{},{},{}",
                bbox.south_west.lon, bbox.south_west.lat, bbox.north_east.lon, bbox.north_east.lat
            );
            request = request.query(&[("bbox", format)]);
        }
        if let Some(limit) = self.limit {
            request = request.query(&[("limit", limit)]);
        }
        if let Some(lang) = self.lang {
            request = request.query(&[("lang", lang)]);
        }
        if let Some(layers) = self.layer {
            for layer in layers {
                request = request.query(&[("layer", layer)]);
            }
        }
        if let Some(query) = self.additional_query {
            request = request.query(&query);
        }
        request
    }
}

impl RequestAppend for ReverseFilter {
    fn append_to(self, request: RequestBuilder) -> RequestBuilder {
        let mut request = request;
        if let Some(radius) = self.radius {
            request = request.query(&[("radius", radius)]);
        }
        if let Some(limit) = self.limit {
            request = request.query(&[("limit", limit)]);
        }
        if let Some(lang) = self.lang {
            request = request.query(&[("lang", lang)]);
        }
        if let Some(layers) = self.layer {
            for layer in layers {
                request = request.query(&[("layer", layer)]);
            }
        }
        if let Some(query) = self.additional_query {
            request = request.query(&query);
        }
        request
    }
}

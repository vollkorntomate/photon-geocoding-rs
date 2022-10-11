use std::error::Error;

use serde::Deserialize;
use ureq::{Agent, AgentBuilder, Request};

use crate::data::filter::{ForwardFilter, ReverseFilter};
use crate::data::json::PhotonFeatureCollection;
use crate::data::{LatLon, PhotonFeature};
use crate::error::PhotonError;

type PhotonResult = Result<Vec<PhotonFeature>, Box<dyn Error>>;

pub struct Client {
    forward_url: String,
    reverse_url: String,
    client: Agent,
}

impl Default for Client {
    /// Default Photon client, using https://photon.komoot.io for requests.
    fn default() -> Self {
        Client::new(&"https://photon.komoot.io")
    }
}

impl Client {
    /// Creates a new API client with the specified `base_url`.
    /// 
    /// `base_url` must begin with `http://` or `https://`.
    pub fn new(base_url: &str) -> Self {
        let mut base_url = base_url;
        if base_url.ends_with("/") {
            base_url = &base_url[..base_url.len() - 1]
        }
        Client {
            forward_url: String::from(base_url) + "/api",
            reverse_url: String::from(base_url) + "/reverse",
            client: AgentBuilder::new().build(),
        }
    }

    /// Performs a forward search for the provided `query`.
    /// 
    /// Results can be filtered by the optional `filter`. Pass `None` for no filter.
    /// 
    /// This function is blocking, so no async features are involved here. It is, however, safe to
    /// call this function in parallel, since the entire API client is thread-safe.
    pub fn forward_search(&self, query: &str, filter: Option<ForwardFilter>) -> PhotonResult {
        let mut request = self.client.get(&self.forward_url).query("q", query);

        if let Some(filter) = filter {
            request = filter.append_to(request);
        }

        let response = request.call()?.into_json()?;

        self.parse_response(response)
    }

    /// Performs a reverse search for objects at the specified `coords`.
    /// 
    /// Results can be filtered by the optional `filter`. Pass `None` for no filter.
    /// 
    /// This function is blocking, so no async features are involved here. It is, however, safe to
    /// call this function in parallel, since the entire API client is thread-safe.
    pub fn reverse_search(
        &self,
        coords: LatLon,
        filter: Option<ReverseFilter>,
    ) -> PhotonResult {
        let mut request = self
            .client
            .get(&self.reverse_url)
            .query("lon", &coords.lon.to_string())
            .query("lat", &coords.lat.to_string());

        if let Some(filter) = filter {
            request = filter.append_to(request)
        }

        let response = request.call()?.into_json()?;

        self.parse_response(response)
    }

    fn parse_response(&self, response: serde_json::Value) -> PhotonResult {
        let deserialize_result = PhotonFeatureCollection::deserialize(&response);
        match deserialize_result {
            Ok(features) => {
                return Ok(features
                    .features()
                    .into_iter()
                    .map(PhotonFeature::from)
                    .collect());
            }
            Err(error) => {
                let message = self.try_parse_error(response);
                match message {
                    Some(error) => Err(Box::new(error)),
                    None => Err(Box::new(error)),
                }
            }
        }
    }

    fn try_parse_error(&self, response: serde_json::Value) -> Option<PhotonError> {
        let message = response.get("message")?.to_string();
        Some(PhotonError::new(&message))
    }
}

#[test]
fn test_base_url_trailing_slash() {
    let base_url_with_trailing_slash = "https://example.com/";
    let base_url_without_trailing_slash = "https://example.com";

    let client_with = Client::new(base_url_with_trailing_slash);
    let client_without = Client::new(base_url_without_trailing_slash);

    assert_eq!(client_with.forward_url, client_without.forward_url);
    assert_eq!(client_with.reverse_url, client_without.reverse_url);
}

pub trait RequestAppend {
    fn append_to(self, request: Request) -> Request;
}

impl RequestAppend for ForwardFilter {
    fn append_to(self, request: Request) -> Request {
        let mut request = request;
        if let Some(bias) = self.location_bias {
            request = request
                .query("lat", &bias.lat.to_string())
                .query("lon", &bias.lon.to_string());

            if let Some(zoom) = self.location_bias_zoom {
                request = request.query("zoom", &zoom.to_string());
            }
            if let Some(scale) = self.location_bias_scale {
                request = request.query("location_bias_scale", &scale.to_string());
            }
        }
        if let Some(bbox) = self.bounding_box {
            let format = format!(
                "{},{},{},{}",
                bbox.south_west.lon, bbox.south_west.lat, bbox.north_east.lon, bbox.north_east.lat
            );
            request = request.query("bbox", &format);
        }
        if let Some(limit) = self.limit {
            request = request.query("limit", &limit.to_string());
        }
        if let Some(lang) = self.lang {
            request = request.query("lang", &lang);
        }
        if let Some(layers) = self.layer {
            for layer in layers {
                request = request.query("layer", &layer.to_string());
            }
        }
        if let Some(query) = self.additional_query {
            for (param, value) in query {
                request = request.query(&param, &value);
            }
        }
        request
    }
}

impl RequestAppend for ReverseFilter {
    fn append_to(self, request: Request) -> Request {
        let mut request = request;
        if let Some(radius) = self.radius {
            request = request.query("radius", &radius.to_string());
        }
        if let Some(limit) = self.limit {
            request = request.query("limit", &limit.to_string());
        }
        if let Some(lang) = self.lang {
            request = request.query("lang", &lang);
        }
        if let Some(layers) = self.layer {
            for layer in layers {
                request = request.query("layer", &layer.to_string());
            }
        }
        if let Some(query) = self.additional_query {
            for (param, value) in query {
                request = request.query(&param, &value);
            }
        }
        request
    }
}

use std::fmt;

use crate::{BoundingBox, LatLon};

pub enum PhotonLayer {
    House,
    Street,
    Locality,
    District,
    City,
    County,
    State,
    Country,
}

impl fmt::Display for PhotonLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::House => write!(f, "house"),
            Self::Street => write!(f, "street"),
            Self::Locality => write!(f, "locality"),
            Self::District => write!(f, "district"),
            Self::City => write!(f, "city"),
            Self::County => write!(f, "county"),
            Self::State => write!(f, "state"),
            Self::Country => write!(f, "country"),
        }
    }
}

/// Filtering options for forward searches. This struct implements a builder pattern, so filters
/// can be easily constructed.
pub struct ForwardFilter {
    pub location_bias: Option<LatLon>,
    pub location_bias_zoom: Option<u64>,
    pub location_bias_scale: Option<f64>,
    pub bounding_box: Option<BoundingBox>,
    pub limit: Option<u64>,
    pub lang: Option<String>,
    pub layer: Option<Vec<PhotonLayer>>,
    pub additional_query: Option<Vec<(String, String)>>,
}

impl Default for ForwardFilter {
    fn default() -> Self {
        ForwardFilter {
            location_bias: None,
            location_bias_zoom: None,
            location_bias_scale: None,
            bounding_box: None,
            limit: None,
            lang: None,
            layer: None,
            additional_query: None,
        }
    }
}

impl ForwardFilter {
    /// Construct a new `ForwardFilter`. All fields are set to `None` in the beginning.
    pub fn new() -> Self {
        Self::default()
    }

    /// Concentrate the search around a specific coordinate.
    ///
    /// `zoom` describes the radius around the coordinate to focus on.
    /// `scale` describes how much the prominence of a result should still be taken into account.
    /// See [Photon documentation](https://github.com/komoot/photon#search-with-location-bias) for details
    pub fn location_bias(mut self, coords: LatLon, zoom: Option<u64>, scale: Option<f64>) -> Self {
        self.location_bias = Some(coords);
        self.location_bias_zoom = zoom;
        self.location_bias_scale = scale;
        self
    }

    /// Concentrate the search in a specific rectangular area.
    pub fn bounding_box(mut self, bbox: BoundingBox) -> Self {
        self.bounding_box = Some(bbox);
        self
    }

    /// Limit the number of search results.
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Return results in a specific language. Photon currently supports `DE`, `EN` and `FR`.
    /// Defaults to the local language of a search result.
    pub fn language(mut self, lang: &str) -> Self {
        self.lang = Some(String::from(lang.to_lowercase()));
        self
    }

    /// Filter results by layer. See [Photon documentation](https://github.com/komoot/photon#filter-results-by-layer)
    pub fn layer(mut self, layer: Vec<PhotonLayer>) -> Self {
        self.layer = Some(layer);
        self
    }

    /// Add additional query strings to the request. Example: [Filtering by tags and values](https://github.com/komoot/photon#filter-results-by-tags-and-values)
    pub fn additional_query(mut self, query: Vec<(&str, &str)>) -> Self {
        self.additional_query = Some(
            query
                .iter()
                .map(|(s, t)| (s.to_string(), t.to_string()))
                .collect(),
        );
        self
    }
}

/// Filtering options for reverse searches. This struct implements a builder pattern, so filters
/// can be easily constructed.
pub struct ReverseFilter {
    pub radius: Option<u64>,
    pub limit: Option<u64>,
    pub lang: Option<String>,
    pub layer: Option<Vec<PhotonLayer>>,
    pub additional_query: Option<Vec<(String, String)>>,
}

impl Default for ReverseFilter {
    fn default() -> Self {
        ReverseFilter {
            radius: None,
            limit: None,
            lang: None,
            layer: None,
            additional_query: None,
        }
    }
}

impl ReverseFilter {
    /// Construct a new `ReverseFilter`. All fields are set to `None` in the beginning.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn radius(mut self, radius: u64) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Limit the number of search results.
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Return results in a specific language. Photon currently supports `DE`, `EN` and `FR`.
    /// Defaults to the local language of a search result.
    pub fn language(mut self, lang: &str) -> Self {
        self.lang = Some(String::from(lang.to_lowercase()));
        self
    }

    /// Filter results by layer. See [Photon documentation](https://github.com/komoot/photon#filter-results-by-layer)
    pub fn layer(mut self, layer: Vec<PhotonLayer>) -> Self {
        self.layer = Some(layer);
        self
    }

    /// Add additional query strings to the request. Example: [Filtering by tags and values](https://github.com/komoot/photon#filter-results-by-tags-and-values)
    pub fn additional_query(mut self, query: Vec<(&str, &str)>) -> Self {
        self.additional_query = Some(
            query
                .iter()
                .map(|(s, t)| (s.to_string(), t.to_string()))
                .collect(),
        );
        self
    }
}

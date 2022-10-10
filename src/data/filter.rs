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

pub struct ForwardFilter {
    pub location_bias: Option<LatLon>,
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
            bounding_box: None,
            limit: None,
            lang: None,
            layer: None,
            additional_query: None,
        }
    }
}

impl ForwardFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn location_bias(mut self, coords: LatLon) -> Self {
        self.location_bias = Some(coords);
        self
    }

    pub fn bounding_box(mut self, bbox: BoundingBox) -> Self {
        self.bounding_box = Some(bbox);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn language(mut self, lang: &str) -> Self {
        self.lang = Some(String::from(lang.to_lowercase()));
        self
    }

    pub fn layer(mut self, layer: Vec<PhotonLayer>) -> Self {
        self.layer = Some(layer);
        self
    }

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn radius(mut self, radius: u64) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn language(mut self, lang: &str) -> Self {
        self.lang = Some(String::from(lang.to_lowercase()));
        self
    }

    pub fn layer(mut self, layer: Vec<PhotonLayer>) -> Self {
        self.layer = Some(layer);
        self
    }

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

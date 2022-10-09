use crate::{BoundingBox, LatLon};

pub struct ForwardFilter {
    pub location_bias: Option<LatLon>,
    pub bounding_box: Option<BoundingBox>,
    pub limit: Option<u64>,
    pub lang: Option<String>,
    pub layer: Option<Vec<String>>,
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

    pub fn language(mut self, lang: String) -> Self {
        self.lang = Some(lang);
        self
    }

    pub fn layer(mut self, layer: Vec<String>) -> Self {
        self.layer = Some(layer);
        self
    }

    pub fn additional_query(mut self, query: Vec<(String, String)>) -> Self {
        self.additional_query = Some(query);
        self
    }
}

pub struct ReverseFilter {
    pub radius: Option<u64>,
    pub limit: Option<u64>,
    pub lang: Option<String>,
    pub layer: Option<Vec<String>>,
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

    pub fn language(mut self, lang: String) -> Self {
        self.lang = Some(lang);
        self
    }

    pub fn layer(mut self, layer: Vec<String>) -> Self {
        self.layer = Some(layer);
        self
    }

    pub fn additional_query(mut self, query: Vec<(String, String)>) -> Self {
        self.additional_query = Some(query);
        self
    }
}

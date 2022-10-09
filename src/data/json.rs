use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PhotonFeatureCollection {
    features: Vec<PhotonFeatureRaw>,
}

impl PhotonFeatureCollection {
    pub fn features(self) -> Vec<PhotonFeatureRaw> {
        self.features
    }
}

#[derive(Debug, Deserialize)]
pub struct PhotonFeatureRaw {
    pub geometry: Geometry,
    pub r#type: String,
    pub properties: Properties,
}

#[derive(Debug, Deserialize)]
pub struct Geometry {
    pub coordinates: Vec<f64>,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub osm_id: u64,
    pub osm_type: String,
    pub osm_key: String,
    pub osm_value: String,
    pub r#type: String,

    pub extent: Option<Vec<f64>>,
    pub name: Option<String>,

    pub country: Option<String>,
    pub countrycode: Option<String>,
    pub state: Option<String>,
    pub county: Option<String>,
    pub city: Option<String>,
    pub locality: Option<String>,
    pub postcode: Option<String>,
    pub district: Option<String>,
    pub street: Option<String>,
    pub housenumber: Option<String>,
}

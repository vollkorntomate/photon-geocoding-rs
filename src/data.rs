pub mod json;

use self::json::PhotonFeatureRaw;

#[derive(Debug)]
pub struct LatLng {
    pub lat: f64,
    pub lon: f64,
}

impl LatLng {
    fn from_vec(vec: &[f64]) -> LatLng {
        assert!(vec.len() >= 2);
        LatLng {
            lat: vec[1], // API format is [lon,lat]
            lon: vec[0],
        }
    }
}

#[derive(Debug)]
pub enum OsmType {
    Relation,
    Way,
    Node,
}

impl From<String> for OsmType {
    fn from(str: String) -> Self {
        match str.as_str() {
            "R" => Self::Relation,
            "W" => Self::Way,
            "N" => Self::Node,
            _ => panic!("Unexpected OSM Type"),
        }
    }
}

#[derive(Debug)]
pub struct BoundingBox {
    pub south_west: LatLng,
    pub north_east: LatLng,
}

impl From<Vec<f64>> for BoundingBox {
    fn from(vec: Vec<f64>) -> Self {
        assert!(vec.len() >= 4);

        BoundingBox {
            south_west: LatLng::from_vec(&vec[0..2]),
            north_east: LatLng::from_vec(&vec[2..4]),
        }
    }
}

#[derive(Debug)]
pub struct PhotonFeature {
    pub coords: LatLng,

    pub osm_id: u64,
    pub osm_key: String,

    pub osm_type: OsmType,
    pub osm_value: String,
    pub r#type: String,

    pub extent: Option<BoundingBox>,
    pub name: Option<String>,

    pub country: Option<String>,
    pub country_iso_code: Option<String>,
    pub state: Option<String>,
    pub county: Option<String>,
    pub city: Option<String>,
    pub postcode: Option<String>,
    pub district: Option<String>,
    pub street: Option<String>,
    pub house_number: Option<String>,
}

impl From<PhotonFeatureRaw> for PhotonFeature {
    fn from(raw: PhotonFeatureRaw) -> Self {
        PhotonFeature {
            coords: LatLng::from_vec(&raw.geometry.coordinates),
            osm_id: raw.properties.osm_id,
            osm_key: raw.properties.osm_key,
            osm_type: OsmType::from(raw.properties.osm_type),
            osm_value: raw.properties.osm_value,
            r#type: raw.r#type,
            extent: raw.properties.extent.map(BoundingBox::from),
            name: raw.properties.name,
            country: raw.properties.country,
            country_iso_code: raw.properties.countrycode,
            state: raw.properties.state,
            county: raw.properties.county,
            city: raw.properties.city,
            postcode: raw.properties.postcode,
            district: raw.properties.district,
            street: raw.properties.street,
            house_number: raw.properties.housenumber,
        }
    }
}

#[derive(Debug)]
pub struct PhotonApiRequest {}
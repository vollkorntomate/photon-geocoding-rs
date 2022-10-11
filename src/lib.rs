#![forbid(unsafe_code)]

mod api;
mod data;

pub mod error;

pub use api::Client as PhotonApiClient;
pub use data::filter;
pub use data::{BoundingBox, LatLon, PhotonFeature};

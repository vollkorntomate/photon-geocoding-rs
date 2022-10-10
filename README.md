# photon-geocoding-rs

[![version](https://img.shields.io/badge/version-1.0.0-green.svg)](https://codeberg.org/vollkorntomate/photon-geocoding-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![crates.io](https://img.shields.io/badge/crates.io-v1.0.0-orange.svg?logo=rust)](https://crates.io)

An API client for Komoot's Photon API written in and for Rust.

It supports forward and reverse geocoding as well as search-as-you-type.

---

<a href="https://codeberg.org/vollkorntomate/photon-geocoding-rs">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="60">
</a>

The main repository is hosted on [codeberg.org](https://codeberg.org/vollkorntomate/photon-geocoding-rs). Issues and Pull Requests are preferred there, but you can still open one on GitHub.

---

## Photon

Photon is a free and open-source API hosted by Komoot and powered by ElasticSearch. It returns data from the OpenStreetMap project,
which is licensed under the [ODbL License](https://opendatacommons.org/licenses/odbl/).

The API is available at [photon.komoot.io](https://photon.komoot.io)
and licensed under the [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0).

**Important:** Please be aware of the Terms and Use of Photon! It is free to use, so please be fair and avoid excessive requests!

## Usage

In your `cargo.toml`, include this:
```toml
[dependencies]
photon-geocoding = { version = "1.0" }
```

Forward geocoding:
```rust
use photon_geocoding::{PhotonApiClient, PhotonFeature};

let api: PhotonApiClient = PhotonApiClient::default();
let result: Vec<PhotonFeature> = api.forward_search("munich", None).unwrap();
```

Reverse geocoding:
```rust
use photon_geocoding::{PhotonApiClient, PhotonFeature};

let api: PhotonApiClient = PhotonApiClient::default();
let result: Vec<PhotonFeature> = api.reverse_search(LatLon::new(48.123, 11.321), None).unwrap();
```

Self-hosted instances (custom URL):
```rust
use photon_geocoding::PhotonApiClient;

let api: PhotonApiClient = PhotonApiClient::new("https://example.com");
// requests will now go to https://example.com/api and https://example.com/reverse
```

Filters:
```rust
use photon_geocoding::filter::{ForwardFilter, PhotonLayer};
use photon_geocoding::{BoundingBox, LatLon, PhotonApiClient};

let api: PhotonApiClient = PhotonApiClient::default();
let filter = ForwardFilter::new()
    .language("FR")
    .bounding_box(BoundingBox {
        south_west: LatLon::new(40.0, 10.0),
        north_east: LatLon::new(50.0, 15.0),
    })
    .layer(vec![PhotonLayer::City, PhotonLayer::State])
    .additional_query(vec![("osm_tag", "!key:value")]);

let results = api.forward_search("munich", Some(filter)).unwrap();

// resulting query string: "q=munich&bbox=10%2C40%2C15%2C50&lang=fr&layer=city&layer=state&osm_tag=%21key%3Avalue"
```

## Features and Bugs

Feel free to open a new issue! I am always happy to improve this package.

As I am fairly new to Rust, please also don't hesitate to suggest improvements on code style and/or usability (especially regarding ownership, borrowing etc.)!

## Contribution

Feel free to open pull requests and help to improve this package!
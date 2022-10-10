mod forward_search {
    use photon_geocoding::filter::{ForwardFilter, PhotonLayer};
    use photon_geocoding::{BoundingBox, LatLon, PhotonApiClient};

    #[test]
    fn result_is_not_empty() {
        let api = PhotonApiClient::default();
        let result = api.forward_search("munich", None);

        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty())
    }

    #[test]
    fn returns_elements_for_oceans() {
        // This test originates from this issue: https://github.com/vollkorntomate/flutter-photon/issues/8
        let api = PhotonApiClient::default();
        let result = api.forward_search("Atlantic Ocean", None);

        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty())
    }

    #[test]
    fn limits_results() {
        let api = PhotonApiClient::default();
        let filter = ForwardFilter::new().limit(2);
        let results_without_filter = api.forward_search("munich", None).unwrap();
        let results_with_filter = api.forward_search("munich", Some(filter)).unwrap();

        assert_ne!(results_without_filter.len(), results_with_filter.len());
        assert_eq!(results_with_filter.len(), 2);
    }

    #[test]
    fn respects_location_bias() {
        let api = PhotonApiClient::default();
        let filter = ForwardFilter::new().location_bias(LatLon::new(48.6701, -98.8485));
        let results = api.forward_search("munich", Some(filter)).unwrap();

        assert!(!results.is_empty());
        assert_eq!(
            results.first().unwrap().country_iso_code,
            Some(String::from("US"))
        );
    }

    #[test]
    fn uses_lang_code() {
        let api = PhotonApiClient::default();
        let filter = ForwardFilter::new().language("FR");
        let results = api.forward_search("münchen", Some(filter)).unwrap();

        assert!(!results.is_empty());
        assert_eq!(
            results.first().unwrap().country,
            Some(String::from("Allemagne"))
        );
    }

    #[test]
    fn uses_bounding_box() {
        let api = PhotonApiClient::default();
        let bbox_bavaria = BoundingBox {
            south_west: LatLon::new(46.0, 10.0),
            north_east: LatLon::new(48.0, 12.0),
        };
        let bbox_thuringia = BoundingBox {
            south_west: LatLon::new(50.0, 11.0),
            north_east: LatLon::new(51.0, 12.0),
        };

        let filter_bavaria = ForwardFilter::new()
            .bounding_box(bbox_bavaria)
            .language("DE");
        let filter_thuringia = ForwardFilter::new()
            .bounding_box(bbox_thuringia)
            .language("DE");

        let results_bavaria = api.forward_search("münchen", Some(filter_bavaria)).unwrap();
        let results_thuringia = api
            .forward_search("münchen", Some(filter_thuringia))
            .unwrap();

        assert!(!results_bavaria.is_empty());
        assert!(!results_thuringia.is_empty());
        assert_eq!(
            results_bavaria.first().unwrap().state,
            Some(String::from("Bayern"))
        );
        assert_eq!(
            results_thuringia.first().unwrap().state,
            Some(String::from("Thüringen"))
        );
    }

    #[test]
    fn uses_layers() {
        let api = PhotonApiClient::default();

        let filter = ForwardFilter::new().layer(vec![PhotonLayer::State]);

        let results_without_filter = api.forward_search("bayern", None).unwrap();
        let results_with_filter = api.forward_search("bayern", Some(filter)).unwrap();

        assert_eq!(results_with_filter.len(), 1);
        assert_eq!(
            results_with_filter.first().unwrap().r#type,
            String::from("state")
        );
        assert_ne!(results_with_filter.len(), results_without_filter.len())
    }
}

mod reverse_search {
    use photon_geocoding::filter::{PhotonLayer, ReverseFilter};
    use photon_geocoding::{LatLon, PhotonApiClient};

    #[test]
    fn gives_at_least_one_result_for_a_place() {
        let api = PhotonApiClient::default();
        let results = api
            .reverse_search(LatLon::new(48.14368, 11.58775), None)
            .unwrap();

        assert!(!results.is_empty())
    }

    #[test]
    fn gives_no_result_for_a_place_with_no_data() {
        let api = PhotonApiClient::default();
        let results = api.reverse_search(LatLon::new(1.0, 1.0), None).unwrap();

        assert!(results.is_empty())
    }

    #[test]
    fn gives_a_result_for_a_place_with_no_data_and_radius() {
        let api = PhotonApiClient::default();
        let filter = ReverseFilter::new().radius(8);
        let results = api
            .reverse_search(LatLon::new(47.8912, 12.4639), Some(filter))
            .unwrap();

        assert!(!results.is_empty())
    }

    #[test]
    fn uses_lang_code() {
        let api = PhotonApiClient::default();
        let filter = ReverseFilter::new().language("FR");
        let results = api
            .reverse_search(LatLon::new(48.14368, 11.58775), Some(filter))
            .unwrap();

        assert!(!results.is_empty());
        assert_eq!(
            results.first().unwrap().country,
            Some(String::from("Allemagne"))
        );
    }

    #[test]
    fn uses_layers() {
        let api = PhotonApiClient::default();

        let filter = ReverseFilter::new().layer(vec![PhotonLayer::City]);

        let results_without_filter = api
            .reverse_search(LatLon::new(48.1379, 11.5734), None)
            .unwrap();
        let results_with_filter = api
            .reverse_search(LatLon::new(48.1379, 11.5734), Some(filter))
            .unwrap();

        assert_eq!(results_with_filter.len(), 1);
        assert_eq!(
            results_with_filter.first().unwrap().r#type,
            String::from("city")
        );
        assert_ne!(
            results_with_filter.first().unwrap().r#type,
            results_without_filter.first().unwrap().r#type
        )
    }
}

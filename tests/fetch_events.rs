//! Tests related to fetching the API `/events/rotation` and `/gamemodes` endpoints.
#[cfg(test)]
mod common;

#[cfg(test)]
mod tests {
    use brawl_api::prelude::*;
    use super::common;

    fn deterministic_gamemode_query_variants() -> Vec<(Option<u8>, Option<&'static str>, Option<&'static str>)> {
        vec![
            (None, None, None),
            (Some(5), None, None),
            (Some(25), Some("before_marker"), None),
            (Some(50), None, Some("after_marker")),
            (Some(100), Some("before_marker_2"), Some("after_marker_2")),
        ]
    }

    /// Checks if events rotation fetching does not error.
    #[test]
    fn events_rotation_fetch() {
        let client = common::create_test_client();
        ScheduledEvents::fetch(&client).unwrap();
    }

    /// Checks if gamemode fetching does not error with common limits.
    #[test]
    fn gamemodes_fetch_limit_samples() {
        let client = common::create_test_client();

        EventTypeList::fetch(&client, None, None, None).unwrap();
        EventTypeList::fetch(&client, Some(10), None, None).unwrap();
        EventTypeList::fetch(&client, Some(25), None, None).unwrap();
    }

    /// Checks route generation for random-like query parameter combinations.
    #[test]
    fn gamemodes_route_query_matrix() {
        for (limit, before, after) in deterministic_gamemode_query_variants() {
            let route = EventTypeList::get_route(limit, before, after).to_url_str();
            assert!(route.contains("/gamemodes"));

            match limit {
                Some(v) => assert!(route.contains(&format!("limit={}", v))),
                None => assert!(!route.contains("limit=")),
            }

            match before {
                Some(v) => assert!(route.contains(&format!("before={}", v))),
                None => assert!(!route.contains("before=")),
            }

            match after {
                Some(v) => assert!(route.contains(&format!("after={}", v))),
                None => assert!(!route.contains("after=")),
            }
        }
    }
}

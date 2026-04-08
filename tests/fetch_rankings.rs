//! Tests related to fetching the API `/rankings/` endpoint.
#[cfg(test)]
mod common;

#[cfg(test)]
mod tests {
    use brawl_api::prelude::*;
    use super::common;

    const TEST_RANK_REGION: &str = "global";
    const TEST_BRAWLER_ID: usize = Brawlers::Shelly as usize;
    const DEFAULT_COUNTRY_SAMPLE_SIZE: usize = 12;
    const DEFAULT_COUNTRY_SAMPLE_SEED: usize = 17;
    const COUNTRY_SAMPLE_MULTIPLIER: usize = 73;
    const COUNTRY_SAMPLE_INCREMENT: usize = 19;
    const DEFAULT_CURSOR_VARIANT_COUNT: usize = 4;
    const DEFAULT_CURSOR_VARIANT_SEED: usize = 11;

    // Large ISO-3166 alpha-2 coverage list for route/query matrix tests.
    const COUNTRY_CODES_ALPHA2: &[&str] = &[
        "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AO", "AQ", "AR", "AS", "AT", "AU", "AW", "AX", "AZ",
        "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL", "BM", "BN", "BO", "BQ", "BR", "BS", "BT", "BV", "BW", "BY", "BZ",
        "CA", "CC", "CD", "CF", "CG", "CH", "CI", "CK", "CL", "CM", "CN", "CO", "CR", "CU", "CV", "CW", "CX", "CY", "CZ",
        "DE", "DJ", "DK", "DM", "DO", "DZ",
        "EC", "EE", "EG", "EH", "ER", "ES", "ET",
        "FI", "FJ", "FK", "FM", "FO", "FR",
        "GA", "GB", "GD", "GE", "GF", "GG", "GH", "GI", "GL", "GM", "GN", "GP", "GQ", "GR", "GS", "GT", "GU", "GW", "GY",
        "HK", "HM", "HN", "HR", "HT", "HU",
        "ID", "IE", "IL", "IM", "IN", "IO", "IQ", "IR", "IS", "IT",
        "JE", "JM", "JO", "JP",
        "KE", "KG", "KH", "KI", "KM", "KN", "KP", "KR", "KW", "KY", "KZ",
        "LA", "LB", "LC", "LI", "LK", "LR", "LS", "LT", "LU", "LV", "LY",
        "MA", "MC", "MD", "ME", "MF", "MG", "MH", "MK", "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS", "MT", "MU", "MV", "MW", "MX", "MY", "MZ",
        "NA", "NC", "NE", "NF", "NG", "NI", "NL", "NO", "NP", "NR", "NU", "NZ",
        "OM",
        "PA", "PE", "PF", "PG", "PH", "PK", "PL", "PM", "PN", "PR", "PS", "PT", "PW", "PY",
        "QA",
        "RE", "RO", "RS", "RU", "RW",
        "SA", "SB", "SC", "SD", "SE", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS", "ST", "SV", "SX", "SY", "SZ",
        "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO", "TR", "TT", "TV", "TW", "TZ",
        "UA", "UG", "UM", "US", "UY", "UZ",
        "VA", "VC", "VE", "VG", "VI", "VN", "VU",
        "WF", "WS",
        "YE", "YT",
        "ZA", "ZM", "ZW",
    ];

    fn deterministic_country_subset(sample_size: usize, seed: usize) -> Vec<&'static str> {
        let mut idx: usize = seed % COUNTRY_CODES_ALPHA2.len();
        let mut out = Vec::with_capacity(sample_size);

        while out.len() < sample_size {
            idx = (idx
                .wrapping_mul(COUNTRY_SAMPLE_MULTIPLIER)
                .wrapping_add(COUNTRY_SAMPLE_INCREMENT))
                % COUNTRY_CODES_ALPHA2.len();
            let code = COUNTRY_CODES_ALPHA2[idx];
            if !out.contains(&code) {
                out.push(code);
            }
        }

        out
    }

    fn deterministic_cursor_variants(count: usize, seed: usize) -> Vec<(Option<&'static str>, Option<&'static str>)> {
        let variants = [
            (None, None),
            (Some("before_cursor_a"), None),
            (None, Some("after_cursor_b")),
            (Some("before_cursor_c"), Some("after_cursor_d")),
        ];

        let mut idx = seed % variants.len();
        let mut out = Vec::with_capacity(count);
        while out.len() < count {
            idx = (idx.wrapping_mul(5).wrapping_add(1)) % variants.len();
            let candidate = variants[idx];
            if !out.contains(&candidate) {
                out.push(candidate);
            }
        }

        out
    }

    /// Checks if player ranking fetching does not error.
    #[test]
    fn rankings_players_fetch() {
        let client = common::create_test_client();

        PlayerLeaderboard::fetch(&client, TEST_RANK_REGION, 5).unwrap();
    }

    /// Checks if club ranking fetching does not error.
    #[test]
    fn rankings_clubs_fetch() {
        let client = common::create_test_client();

        ClubLeaderboard::fetch(&client, TEST_RANK_REGION, 5).unwrap();
    }

    /// Checks if brawler ranking fetching does not error.
    #[test]
    fn rankings_brawlers_fetch() {
        let client = common::create_test_client();

        BrawlerLeaderboard::fetch(&client, TEST_RANK_REGION, TEST_BRAWLER_ID, 5)
            .unwrap();
    }

    /// Checks if ranking fetches work for a deterministic sample of country codes.
    #[test]
    fn rankings_fetch_country_samples() {
        let client = common::create_test_client();
        let (country_sample_size, country_sample_seed, _, _) = rankings_matrix_params();

        for country in deterministic_country_subset(country_sample_size, country_sample_seed) {
            PlayerLeaderboard::fetch_with(&client, country, 5, None, None).unwrap();
            ClubLeaderboard::fetch_with(&client, country, 5, None, None).unwrap();
            BrawlerLeaderboard::fetch_with(&client, country, TEST_BRAWLER_ID, 5, None, None).unwrap();
        }
    }

    /// Checks route generation across many alpha-2 countries and cursor combinations.
    #[test]
    fn rankings_route_country_query_matrix() {
        let (_, _, cursor_variant_count, cursor_variant_seed) = rankings_matrix_params();

        for country in COUNTRY_CODES_ALPHA2 {
            for (before, after) in deterministic_cursor_variants(cursor_variant_count, cursor_variant_seed) {
                let player_route = PlayerLeaderboard::get_route_with(country, 5, before, after).to_url_str();
                let club_route = ClubLeaderboard::get_route_with(country, 5, before, after).to_url_str();
                let brawler_route = BrawlerLeaderboard::get_route_with(
                    country,
                    TEST_BRAWLER_ID,
                    5,
                    before,
                    after,
                )
                .to_url_str();

                assert!(player_route.contains(&format!("/rankings/{}/players?limit=5", country)));
                assert!(club_route.contains(&format!("/rankings/{}/clubs?limit=5", country)));
                assert!(brawler_route.contains(&format!("/rankings/{}/brawlers/{}?limit=5", country, TEST_BRAWLER_ID)));

                match before {
                    Some(v) => {
                        assert!(player_route.contains(&format!("before={}", v)));
                        assert!(club_route.contains(&format!("before={}", v)));
                        assert!(brawler_route.contains(&format!("before={}", v)));
                    }
                    None => {
                        assert!(!player_route.contains("before="));
                        assert!(!club_route.contains("before="));
                        assert!(!brawler_route.contains("before="));
                    }
                }

                match after {
                    Some(v) => {
                        assert!(player_route.contains(&format!("after={}", v)));
                        assert!(club_route.contains(&format!("after={}", v)));
                        assert!(brawler_route.contains(&format!("after={}", v)));
                    }
                    None => {
                        assert!(!player_route.contains("after="));
                        assert!(!club_route.contains("after="));
                        assert!(!brawler_route.contains("after="));
                    }
                }
            }
        }
    }

    fn rankings_matrix_params() -> (usize, usize, usize, usize) {
        let cfg = common::open_test_config_panic().rankings_matrix;
        let country_sample_size = cfg
            .country_sample_size
            .unwrap_or(DEFAULT_COUNTRY_SAMPLE_SIZE)
            .clamp(1, COUNTRY_CODES_ALPHA2.len());
        let country_sample_seed = cfg
            .country_sample_seed
            .unwrap_or(DEFAULT_COUNTRY_SAMPLE_SEED);
        let cursor_variant_count = cfg
            .cursor_variant_count
            .unwrap_or(DEFAULT_CURSOR_VARIANT_COUNT)
            .clamp(1, 4);
        let cursor_variant_seed = cfg
            .cursor_variant_seed
            .unwrap_or(DEFAULT_CURSOR_VARIANT_SEED);

        (
            country_sample_size,
            country_sample_seed,
            cursor_variant_count,
            cursor_variant_seed,
        )
    }
}
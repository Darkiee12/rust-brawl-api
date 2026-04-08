//! Contains the `Route` enum, responsible for listing the available API endpoints and parsing
//! the given values into a valid URL.

use crate::b_api_concat;


/// An enum representing the possible Brawl API routes.
#[non_exhaustive]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Route {
    /// Route for the `/players/:tag` endpoint. (`tag` must begin with a `#` (`%23`) for correct
    /// results.)
    ///
    /// This fetches a player's data.
    Player(String),

    /// Route for the `/players/:tag/battlelog` endpoint. (`tag` must begin with a `#` (`%23`) for
    /// correct results.)
    ///
    /// This fetches the player's recently-played battles.
    PlayerBattlelogs(String),

    /// Route for the `/clubs/:tag` endpoint. (`tag` must begin with a `#` (`%23`) for correct
    /// results.)
    ///
    /// This fetches a club's data.
    Club(String),

    /// Route for the `/clubs/:tag/members` endpoint.
    /// (`tag` must begin with a `#` (`%23`) for correct results.)
    ///
    /// This fetches a club's members.
    ClubMembers(String),

    /// Route for the `/rankings/:country_code/players?limit=x` endpoint (shows the top `x` players
    /// with most trophies in said country code).
    ///
    /// The limit can be up to 200. Specifying higher than that simply works the same way as
    /// specifying 200, thus returning up to 200 entries.
    PlayerRankings {
        /// The two-letter country code whose leaderboard should be fetched (e.g. BR for Brazil,
        /// ZW for Zimbabwe...), or `"global"` for the global leaderboard.
        country_code: String,

        /// The limit of rankings to get (i.e., to get the top `limit` players, sorted by trophies).
        limit: u8,

        /// Optional marker to return items before.
        before: Option<String>,

        /// Optional marker to return items after.
        after: Option<String>,
    },

    /// Route for the `/rankings/:country_code/clubs?limit=x` endpoint.
    ///
    /// The limit can be up to 200. Specifying higher than that simply works the same way as
    /// specifying 200, thus returning up to 200 entries.
    ClubRankings {
        /// The two-letter country code whose leaderboard should be fetched (e.g. BR for Brazil,
        /// ZW for Zimbabwe...), or `"global"` for the global leaderboard.
        country_code: String,

        /// The limit of rankings to get (i.e., to get the top `limit` clubs, sorted by trophies).
        limit: u8,

        /// Optional marker to return items before.
        before: Option<String>,

        /// Optional marker to return items after.
        after: Option<String>,
    },

    /// Route for the `/rankings/:country_code/brawlers/:brawler_id?limit=x` endpoint.
    ///
    /// The limit can be up to 200. Specifying higher than that simply works the same way as
    /// specifying 200, thus returning up to 200 entries.
    BrawlerRankings {
        /// The two-letter country code whose leaderboard should be fetched (e.g. BR for Brazil,
        /// ZW for Zimbabwe...), or `"global"` for the global leaderboard.
        country_code: String,

        /// The ID of the brawler whose rankings should be fetched. To obtain this,
        /// use the `/brawlers/` endpoint.
        brawler_id: usize,

        /// The limit of rankings to get (i.e., to get the top `limit` players, sorted by trophies
        /// on this specific brawler).
        limit: u8,

        /// Optional marker to return items before.
        before: Option<String>,

        /// Optional marker to return items after.
        after: Option<String>,
    },

    /// Route for the `/brawlers/` endpoint, which returns data for all brawlers in the game.
    Brawlers,

    /// Route for the `/gamemodes` endpoint.
    ///
    /// Supports optional `limit`, `before`, and `after` query parameters.
    GameModes {
        /// Optional limit of items to fetch.
        limit: Option<u8>,

        /// Optional marker to return items before.
        before: Option<String>,

        /// Optional marker to return items after.
        after: Option<String>,
    },

    /// Route for the `/events/rotation` endpoint.
    EventRotation,

    /// Route for the `/brawlers/:id` endpoint, which returns data for a specific brawler, given
    /// that brawler's ID.
    Brawler(usize),
}

impl Route {

    /// Evaluates the `Route` instance into a full URL path string.
    ///
    /// # Examples
    /// ```rs
    /// use brawl_api::Route;
    /// assert_eq!(Route::Player("tag"), "https://api.brawlstars.com/v1/players/tag")
    /// assert_eq!(
    ///     Route::PlayerBattlelogs("tag"), "https://api.brawlstars.com/v1/players/tag/battlelogs"
    /// )
    /// assert_eq!(Route::Club("tag"), "https://api.brawlstars.com/v1/clubs/tag")
    /// assert_eq!(Route::ClubMembers("tag"), "https://api.brawlstars.com/v1/clubs/tag/members")
    /// ```
    pub fn to_url_str(&self) -> String {
        match self {
            Route::Player(s) => format!("{}{}", b_api_concat!("players/"), s),

            Route::PlayerBattlelogs(s) => format!(
                "{}{}/battlelog", b_api_concat!("players/"), s
            ),

            Route::Club(s) => format!("{}{}", b_api_concat!("clubs/"), s),

            Route::ClubMembers(s) => format!(
                "{}{}/members", b_api_concat!("clubs/"), s
            ),

            Route::PlayerRankings {
                country_code,
                limit,
                before,
                after,
            } => {
                let mut route = format!(
                    "{}{}/players?limit={}",
                    b_api_concat!("rankings/"),
                    country_code,
                    limit
                );

                if let Some(before) = before {
                    route.push_str("&before=");
                    route.push_str(before);
                }

                if let Some(after) = after {
                    route.push_str("&after=");
                    route.push_str(after);
                }

                route
            }

            Route::ClubRankings {
                country_code,
                limit,
                before,
                after,
            } => {
                let mut route = format!(
                    "{}{}/clubs?limit={}",
                    b_api_concat!("rankings/"),
                    country_code,
                    limit
                );

                if let Some(before) = before {
                    route.push_str("&before=");
                    route.push_str(before);
                }

                if let Some(after) = after {
                    route.push_str("&after=");
                    route.push_str(after);
                }

                route
            }

            Route::BrawlerRankings {
                country_code,
                brawler_id,
                limit,
                before,
                after,
            } => {
                let mut route = format!(
                    "{}{}/brawlers/{}?limit={}",
                    b_api_concat!("rankings/"),
                    country_code,
                    brawler_id,
                    limit
                );

                if let Some(before) = before {
                    route.push_str("&before=");
                    route.push_str(before);
                }

                if let Some(after) = after {
                    route.push_str("&after=");
                    route.push_str(after);
                }

                route
            }

            Route::Brawlers => String::from(b_api_concat!("brawlers/")),

            Route::GameModes {
                limit,
                before,
                after,
            } => {
                let mut route = String::from(b_api_concat!("gamemodes"));
                let mut first_query = true;

                if let Some(limit) = limit {
                    route.push(if first_query { '?' } else { '&' });
                    first_query = false;
                    route.push_str(&format!("limit={}", limit));
                }

                if let Some(before) = before {
                    route.push(if first_query { '?' } else { '&' });
                    first_query = false;
                    route.push_str("before=");
                    route.push_str(before);
                }

                if let Some(after) = after {
                    route.push(if first_query { '?' } else { '&' });
                    route.push_str("after=");
                    route.push_str(after);
                }

                route
            }

            Route::EventRotation => String::from(b_api_concat!("events/rotation")),

            Route::Brawler(id) => format!(
                "{}/{}",
                b_api_concat!("brawlers"),
                id,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Route;

    #[test]
    fn player_rankings_route_with_cursors() {
        let route = Route::PlayerRankings {
            country_code: String::from("global"),
            limit: 50,
            before: Some(String::from("abc")),
            after: Some(String::from("def")),
        };

        assert_eq!(
            route.to_url_str(),
            "https://api.brawlstars.com/v1/rankings/global/players?limit=50&before=abc&after=def"
        );
    }

    #[test]
    fn club_rankings_route_with_cursors() {
        let route = Route::ClubRankings {
            country_code: String::from("ZW"),
            limit: 100,
            before: Some(String::from("cursor_before")),
            after: None,
        };

        assert_eq!(
            route.to_url_str(),
            "https://api.brawlstars.com/v1/rankings/ZW/clubs?limit=100&before=cursor_before"
        );
    }

    #[test]
    fn brawler_rankings_route_with_cursors() {
        let route = Route::BrawlerRankings {
            country_code: String::from("BR"),
            brawler_id: 16000000,
            limit: 25,
            before: None,
            after: Some(String::from("cursor_after")),
        };

        assert_eq!(
            route.to_url_str(),
            "https://api.brawlstars.com/v1/rankings/BR/brawlers/16000000?limit=25&after=cursor_after"
        );
    }
}
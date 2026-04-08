//! Contains models related to the `/players/:tag/battlelog` endpoint of the Brawl Stars API.
//! Included by the feature `"players"`; removing that feature will disable the usage of this module.

use crate::error::Result;
use crate::http::routes::Route;
use crate::serde::one_default;
use crate::traits::{FetchFrom, GetFetchProp, PropFetchable};
use crate::util::{a_fetch_route, auto_hashtag, fetch_route};
use serde::{self, Deserialize, Deserializer, Serialize};
use std::ops::{Deref, DerefMut};

use crate::http::Client;
#[cfg(feature = "async")]
use async_trait::async_trait;

use super::player::Player;
use crate::TimeLike;

fn deserialize_null_string_as_default<'de, D>(
    deserializer: D,
) -> ::std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(value.unwrap_or_else(|| String::from("UNKNOWN")))
}

fn deserialize_null_string_as_empty<'de, D>(
    deserializer: D,
) -> ::std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(value.unwrap_or_default())
}

// Sentinel value for trophies and power of a brawler in friendly mode returns -1.
fn deserialize_u8_allow_negative_default_one<'de, D>(
    deserializer: D,
) -> ::std::result::Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<i64>::deserialize(deserializer)?;
    match value {
        Some(v) if v >= 0 => Ok(u8::try_from(v).unwrap_or_default()),
        _ => Ok(0),
    }
}

fn deserialize_usize_allow_negative_default_zero<'de, D>(
    deserializer: D,
) -> ::std::result::Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<i64>::deserialize(deserializer)?;
    match value {
        Some(v) if v >= 0 => Ok(usize::try_from(v).unwrap_or_default()),
        _ => Ok(0),
    }
}

// region:BattleLog

/// Represents a list of a Player's most recent battles.
/// (NOTE: It may take up to 30 minutes for a new battle to appear in the battlelog.)
///
/// Use [`BattleLog::fetch`] to fetch the battle logs for a given player tag.
/// One may also [`BattleLog::fetch_from`] with an existing [`Player`] instance in order to use its
/// tag.
///
/// [`BattleLog::fetch`]: #method.fetch
/// [`BattleLog::fetch_from`]: #method.fetch_from
/// [`Player`]: model/players/player/struct.Player.html
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleLog {
    /// The tag of the player whose BattleLog (most recent battles) was fetched.
    #[serde(skip)] // artificial
    pub tag: String,

    /// The items (battles) of this battle log.
    #[serde(default)]
    pub items: Vec<Battle>,
}

impl Deref for BattleLog {
    type Target = Vec<Battle>;

    /// Obtain the player's battles - dereferencing returns the [`items`] field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, BattleLog, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth token");
    /// let battlelog = BattleLog::fetch(
    ///     &client,            // <- the client containing the auth key
    ///     "#PLAYER_TAG_HERE"  // <- the player whose battlelog should be fetched
    /// )?;
    ///
    /// assert_eq!(battlelog.items, *battlelog);
    ///
    /// #     Ok(())
    /// # }
    ///
    /// ```
    ///
    /// [`items`]: #structfield.items
    fn deref(&self) -> &Vec<Battle> {
        &self.items
    }
}

impl DerefMut for BattleLog {
    /// Obtain the player's battles - dereferencing returns the [`items`] field.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, BattleLog, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let client = Client::new("my auth token");
    /// let battlelog = BattleLog::fetch(
    ///     &client,            // <- the client containing the auth key
    ///     "#PLAYER_TAG_HERE"  // <- the player whose battlelog should be fetched
    /// )?;
    ///
    /// assert_eq!(battlelog.items, *battlelog);
    ///
    /// #     Ok(())
    /// # }
    ///
    /// ```
    ///
    /// [`items`]: #structfield.items
    fn deref_mut(&mut self) -> &mut Vec<Battle> {
        &mut self.items
    }
}

impl GetFetchProp for BattleLog {
    type Property = str;

    fn get_fetch_prop(&self) -> &str {
        &*self.tag
    }

    fn get_route(tag: &str) -> Route {
        Route::PlayerBattlelogs(auto_hashtag(tag))
    }
}

#[cfg_attr(feature = "async", async_trait)]
impl FetchFrom<Player> for BattleLog {
    /// (Sync) Fetches a given player's battlelog (a `BattleLog` instance) by using data from
    /// an existing [`Player`] instance. (See [`BattleLog::fetch`] for more details.)
    ///
    /// Note that this is simply to minimize efforts when a player was already fetched. If
    /// no `Player` instance was previously present, it is recommended to simply `BattleLog::fetch`
    /// with the specific player's tag.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, BattleLog, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let player = Player::fetch(&my_client, "#PLAYERTAGHERE")?;
    /// // do stuff with player...
    /// let player_battlelog = BattleLog::fetch_from(&my_client, &player)?;
    /// // now the player's battlelog is available for use
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Player`]: ../player/struct.Player.html
    /// [`BattleLog::fetch`]: #method.fetch
    fn fetch_from(client: &Client, player: &Player) -> Result<BattleLog> {
        BattleLog::fetch(client, &player.tag)
    }

    /// (Async) Fetches a given player's battlelog (a `BattleLog` instance) by using data from
    /// an existing [`Player`] instance. (See [`BattleLog::fetch`] for more details.)
    ///
    /// Note that this is simply to minimize efforts when a player was already fetched. If
    /// no `Player` instance was previously present, it is recommended to simply `BattleLog::fetch`
    /// with the specific player's tag.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, BattleLog, traits::*};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let player = Player::a_fetch(&my_client, "#PLAYERTAGHERE").await?;
    /// // do stuff with player...
    /// let player_battlelog = BattleLog::a_fetch_from(&my_client, &player).await?;
    /// // now the player's battlelog is available for use
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Player`]: ../player/struct.Player.html
    /// [`BattleLog::fetch`]: #method.fetch
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, player: &Player) -> Result<BattleLog> {
        BattleLog::a_fetch(client, &player.tag).await
    }
}

#[cfg_attr(feature = "async", async_trait)]
impl PropFetchable for BattleLog {
    type Property = str;

    /// (Sync) Fetches a player's battlelog (most recent battles), given its tag.
    ///
    /// # Errors
    ///
    /// This function may error:
    /// - While requesting (will return an [`Error::Request`]);
    /// - After receiving a bad status code (API or other error - returns an [`Error::Status`]);
    /// - After a ratelimit is indicated by the API, while also specifying when it is lifted ([`Error::Ratelimited`]);
    /// - While parsing incoming JSON (will return an [`Error::Json`]).
    ///
    /// (All of those, of course, wrapped inside an `Err`.)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, BattleLog, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let player_battlelog = BattleLog::fetch(&my_client, "#PLAYERTAGHERE")?;
    /// // now the player's battlelog is available for use
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    fn fetch(client: &Client, tag: &str) -> Result<BattleLog> {
        let route = Self::get_route(tag);
        let mut battle_log = fetch_route::<BattleLog>(client, &route)?;
        battle_log.tag = tag.to_owned();
        Ok(battle_log)
    }

    /// (Async) Fetches a player's battlelog (most recent battles), given its tag.
    ///
    /// # Errors
    ///
    /// This function may error:
    /// - While requesting (will return an [`Error::Request`]);
    /// - After receiving a bad status code (API or other error - returns an [`Error::Status`]);
    /// - After a ratelimit is indicated by the API, while also specifying when it is lifted ([`Error::Ratelimited`]);
    /// - While parsing incoming JSON (will return an [`Error::Json`]).
    ///
    /// (All of those, of course, wrapped inside an `Err`.)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, BattleLog, traits::*};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let player_battlelog = BattleLog::a_fetch(&my_client, "#PLAYERTAGHERE").await?;
    /// // now the player's battlelog is available for use
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    #[cfg(feature = "async")]
    async fn a_fetch(client: &Client, tag: &'async_trait str) -> Result<BattleLog>
    where
        Self: 'async_trait,
        Self::Property: 'async_trait,
    {
        let route = BattleLog::get_route(tag);
        let mut battle_log = a_fetch_route::<BattleLog>(client, &route).await?;
        battle_log.tag = tag.to_owned();
        Ok(battle_log)
    }
}

// endregion:BattleLog

/// Represents a Battle in a player's [`BattleLog`].
///
/// [`BattleLog`]: struct.BattleLog.html
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battle {
    /// The time at which this battle occurred, in ISO format.
    #[serde(default)]
    pub battle_time: TimeLike,

    /// Data about the event in which this battle occurred.
    #[serde(default)]
    pub event: BattleEvent,

    /// Data about the battle itself and its outcome.
    #[serde(default)]
    #[serde(rename = "battle")]
    pub result: BattleResultInfo,
}

impl Default for Battle {
    /// Returns a default `Battle` instance, with all default values initialized.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::{Battle, BattleEvent, BattleResultInfo, TimeLike};
    ///
    /// assert_eq!(
    ///     Battle::default(),
    ///     Battle {
    ///         battle_time: TimeLike::default(),
    ///         event: BattleEvent::default(),
    ///         result: BattleResultInfo::default()
    ///     }
    /// )
    /// ```
    fn default() -> Battle {
        Battle {
            battle_time: TimeLike::default(),
            event: BattleEvent::default(),
            result: BattleResultInfo::default(),
        }
    }
}

/// Contains data about the event played during a [`Battle`].
///
/// [`Battle`]: struct.Battle.html
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleEvent {
    /// The id of the event (an arbitrary number).
    #[serde(default)]
    pub id: usize,

    /// The event mode (e.g. "brawlBall", "soloShowdown"...).
    #[serde(default)]
    pub mode: String,

    /// The numeric mode id.
    #[serde(default)]
    pub mode_id: Option<usize>,

    /// The name of the map where this battle happened.
    #[serde(default, deserialize_with = "deserialize_null_string_as_empty")]
    pub map: String,
}

impl BattleEvent {
    /// Returns a default BattleEvent - see [`BattleEvent::default`].
    ///
    /// [`BattleEvent::default`]: #method.default
    pub fn new() -> BattleEvent {
        BattleEvent::default()
    }
}

impl Default for BattleEvent {
    /// Returns a default `BattleEvent` instance, with all default values initialized.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::BattleEvent;
    ///
    /// assert_eq!(
    ///     BattleEvent::default(),
    ///     BattleEvent { id: 0, mode: String::from(""), mode_id: None, map: String::from("") }
    /// )
    /// ```
    fn default() -> BattleEvent {
        BattleEvent {
            id: 0,
            mode: String::from(""),
            mode_id: None,
            map: String::from(""),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BattleOutcome {
    Victory,
    Defeat,
    Draw,
}

impl ::std::fmt::Display for BattleOutcome {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                BattleOutcome::Victory => "Victory",
                BattleOutcome::Defeat => "Defeat",
                BattleOutcome::Draw => "Draw",
            }
        )
    }
}

/// Represents the result of a battle in a [`Battle`] object, including details, outcome,
/// players/teams etc.
///
/// There are three general models of fields here:
///
/// - **Team modes** (Bounty, Gem Grab, Duo Showdown...): fields `mode`, `battle_type`, `duration`,
/// `trophy_change`, `result`, `star_player`, `teams`
/// - **Solo modes** (Solo Showdown): fields `mode`, `battle_type`, `duration`, `trophy_change`,
/// `rank`, `players`
/// - **Weekend events:** Depends on the event. Should always be there: `mode`, `duration`.
///   - Here, `trophy_change` is always equal to 0.
///   - **Boss fight:** `mode`, `duration`, `players`
///   - **Big Brawler:** `mode`, `duration`, `result`, `star_player`, `teams` (needs testing!)
///   - **Robo Rumble:** `mode`, `duration`, `players` (needs testing!)
///
/// [`Battle`]: struct.Battle.html
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleResultInfo {
    /// The event mode (e.g. "brawlBall", "soloShowdown"...). Should be the same as [`BattleEvent.mode`].
    ///
    /// [`BattleEvent.mode`]: ./struct.BattleEvent.html#structfield.mode
    #[serde(default)]
    pub mode: String,

    /// The type of battle (e.g. "ranked").
    ///
    /// If this is `None`, then this is likely a weekend event.
    #[serde(default)]
    #[serde(rename = "type")]
    pub battle_type: Option<String>,

    /// The duration of this battle, in seconds.
    #[serde(default)]
    pub duration: usize,

    /// The difference in trophies applied to the player after the battle. E.g. -4 (lost 4 trophies)
    ///
    /// This is always 0 for weekend events and practice.
    #[serde(default)]
    pub trophy_change: isize,

    /// If this was a solo mode match, then this is the player's final rank (1-10). Otherwise,
    /// `None`.
    #[serde(default)]
    pub rank: Option<u8>,

    /// If this was a match with teams, then this is the outcome for the player
    /// (Victory/Defeat/Draw), otherwise `None`.
    #[serde(default)]
    pub result: Option<BattleOutcome>,

    /// The data indicating who was the Star Player in the match. This is generally from the
    /// winning team, unless a draw occurred, in which case it can be from either team.
    /// If this was a solo mode or boss fight match, for instance, then there is no star player
    /// (None).
    #[serde(default)]
    pub star_player: Option<BattlePlayer>,

    /// If this was a match with teams, then this is a vector with all teams of players
    /// (as vectors) - this can be the teams in a teamed mode such as Bounty, or the pairs in
    /// Duo Showdown, for example.
    /// Otherwise, `None`.
    #[serde(default)]
    pub teams: Option<Vec<Vec<BattlePlayer>>>,

    /// If this was a solo match or a mode without teams, such as Showdown, then this is a vector
    /// with all the players in the match. Otherwise, `None`.
    #[serde(default)]
    pub players: Option<Vec<BattlePlayer>>,
}

impl Default for BattleResultInfo {
    /// Returns an instance of `BattleResultInfo` with initial values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::BattleResultInfo;
    ///
    /// assert_eq!(
    ///     BattleResultInfo::default(),
    ///     BattleResultInfo {
    ///         mode: String::from(""),
    ///         battle_type: Some(String::from("")),
    ///         duration: 0,
    ///         trophy_change: 0,
    ///         rank: None,
    ///         star_player: None,
    ///         result: None,
    ///         teams: None,
    ///         players: None,
    ///     }
    /// );
    /// ```
    fn default() -> BattleResultInfo {
        BattleResultInfo {
            mode: String::from(""),
            battle_type: Some(String::from("")),
            duration: 0,
            trophy_change: 0,
            rank: None,
            star_player: None,
            result: None,
            teams: None,
            players: None,
        }
    }
}

/// Represents a player in a [`BattleResultInfo`] object, with only partial data about it (note that
/// the `brawler` field is exclusive to this struct, representing the brawler the player was using
/// during the battle).
/// One can use [`Player::fetch_from`] to obtain a full [`Player`] instance from an existing
/// `BattlePlayer` instance.
///
/// [`BattleResultInfo`]: ./struct.BattleResult.html
/// [`Player`]: ../player/struct.Player.html
/// [`Player::fetch_from`]: ../player/struct.Player.html#method.fetch_from
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattlePlayer {
    /// The player's tag.
    #[serde(default)]
    pub tag: String,

    /// The player's name.
    #[serde(default)]
    pub name: String,

    /// The brawler the player was using during the battle.
    #[serde(default)]
    pub brawler: BattleBrawler,
}

impl Default for BattlePlayer {
    /// Returns an instance of `BattlePlayer` with initial values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::{BattlePlayer, BattleBrawler};
    ///
    /// assert_eq!(
    ///     BattlePlayer::default(),
    ///     BattlePlayer {
    ///         tag: String::from(""),
    ///         name: String::from(""),
    ///         brawler: BattleBrawler::default(),
    ///     }
    /// );
    /// ```
    fn default() -> BattlePlayer {
        BattlePlayer {
            tag: String::from(""),
            name: String::from(""),
            brawler: BattleBrawler::default(),
        }
    }
}

/// Represents the brawler a player was using in a [`BattlePlayer`] object.
///
/// [`BattlePlayer`]: ./struct.BattlePlayer.html
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleBrawler {
    /// The brawler's id (an arbitrary number).
    #[serde(default)]
    pub id: usize,

    /// The brawler's name (e.g. "PENNY", "ROSA", "BROCK"...)
    #[serde(default, deserialize_with = "deserialize_null_string_as_default")]
    pub name: String,

    /// The brawler's power level.
    #[serde(
        default = "one_default",
        deserialize_with = "deserialize_u8_allow_negative_default_one"
    )]
    pub power: u8,

    #[serde(
        default,
        deserialize_with = "deserialize_usize_allow_negative_default_zero"
    )]
    pub trophies: usize,
}

impl Default for BattleBrawler {
    /// Returns an instance of `BattleBrawler` with initial values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::BattleBrawler;
    ///
    /// assert_eq!(
    ///     BattleBrawler::default(),
    ///     BattleBrawler {
    ///         id: 0,
    ///         name: String::from(""),
    ///         power: 1,
    ///         trophies: 0,
    ///     }
    /// );
    /// ```
    fn default() -> BattleBrawler {
        BattleBrawler {
            id: 0,
            name: String::from(""),
            power: 1,
            trophies: 0,
        }
    }
}

///////////////////////////////////   tests   ///////////////////////////////////

#[cfg(test)]
mod tests {
    use super::BattleLog;
    use serde_json;
    use std::fs::read_to_string;

    mod battlelog_deser_expected {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/battlelog/battlelog_deser_expected.rs"
        ));
    }
    mod mode_id_present_expected {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/battlelog/mode_id_present_expected.rs"
        ));
    }
    mod mode_id_missing_expected {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/battlelog/mode_id_missing_expected.rs"
        ));
    }
    mod null_brawler_name_expected {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/battlelog/null_brawler_name_expected.rs"
        ));
    }
    mod null_event_map_expected {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/battlelog/null_event_map_expected.rs"
        ));
    }
    mod friendly_negative_stats_expected {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/battlelog/friendly_negative_stats_expected.rs"
        ));
    }

    /// Tests for battlelog deserialization from API-provided JSON.
    #[test]
    fn battlelog_deser() -> Result<(), Box<dyn ::std::error::Error>> {
        let battlelog_json_s = read_to_string("tests/fixtures/battlelog/battlelog_deser.json")?;
        let battle_log = serde_json::from_str::<BattleLog>(&battlelog_json_s)?;

        assert_eq!(battle_log, battlelog_deser_expected::expected());

        Ok(())
    }

    #[test]
    fn battlelog_mode_id_deser() -> Result<(), Box<dyn ::std::error::Error>> {
        let json = read_to_string("tests/fixtures/battlelog/mode_id_present.json")?;
        let log: BattleLog = serde_json::from_str(&json)?;

        assert_eq!(log, mode_id_present_expected::expected());

        Ok(())
    }

    #[test]
    fn battlelog_missing_mode_id() -> Result<(), Box<dyn ::std::error::Error>> {
        let json = read_to_string("tests/fixtures/battlelog/mode_id_missing.json")?;
        let log: BattleLog = serde_json::from_str(&json)?;

        assert_eq!(log, mode_id_missing_expected::expected());

        Ok(())
    }

    #[test]
    fn battlelog_null_brawler_name_defaults_to_unknown() -> Result<(), Box<dyn ::std::error::Error>>
    {
        let battlelog_json_s = read_to_string("tests/fixtures/battlelog/null_brawler_name.json")?;
        let battle_log = serde_json::from_str::<BattleLog>(&battlelog_json_s)?;

        assert_eq!(battle_log, null_brawler_name_expected::expected());

        Ok(())
    }

    #[test]
    fn battlelog_null_event_map_defaults_to_empty() -> Result<(), Box<dyn ::std::error::Error>> {
        let battlelog_json_s = read_to_string("tests/fixtures/battlelog/null_event_map.json")?;
        let battle_log = serde_json::from_str::<BattleLog>(&battlelog_json_s)?;

        assert_eq!(battle_log, null_event_map_expected::expected());

        Ok(())
    }

    #[test]
    fn battlelog_negative_brawler_stats_default() -> Result<(), Box<dyn ::std::error::Error>> {
        let battlelog_json_s = read_to_string("tests/fixtures/battlelog/friendly_negative_stats.json")?;
        let battle_log = serde_json::from_str::<BattleLog>(&battlelog_json_s)?;

        assert_eq!(battle_log, friendly_negative_stats_expected::expected());

        Ok(())
    }
}

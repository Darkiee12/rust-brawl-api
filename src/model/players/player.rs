//! Models for the `/players/:tag` API endpoint.
//! Included by the feature `"players"`; removing that feature will disable the usage of this module.

use serde::{self, Serialize, Deserialize};


#[cfg(feature = "async")]


#[cfg(feature = "async")]
use crate::util::a_fetch_route;

#[cfg(feature = "async")]

use crate::traits::{FetchFrom, PropFetchable, GetFetchProp};
use crate::error::{Result};

#[cfg(feature = "clubs")]
use super::super::clubs::ClubMember;

use crate::http::Client;
use crate::http::routes::Route;
use crate::util::{auto_hashtag, fetch_route};
use crate::serde::{deserialize_number_from_string, one_default, oxffffff_default};

use super::super::common::{StarPower, Gadget, Gear, HyperCharge, Skin, Buffies, Icon};

use super::battlelog::BattlePlayer;

#[cfg(feature = "rankings")]
use crate::PlayerRanking;

/// A struct representing a Brawl Stars player, with all of its data.
/// Use [`Player::fetch`] to fetch one based on tag. (Make sure the [`PropFetchable`] trait
/// is imported - in general, it is recommended to **at least** `use brawl_api::traits::*`, or,
/// even, `use brawl_api::prelude::*` to bring the models into scope as well.)
///
/// [`PropFetchable`]: traits/trait.PropFetchable.html
/// [`Player::fetch`]: ./struct.Player.html#method.fetch
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {

    /// The player's tag. **Note: this includes the initial '#'.**
    #[serde(default)]
    pub tag: String,

    /// The player's name.
    #[serde(default)]
    pub name: String,

    /// The player's name color, as an integer (Default is 0xffffff = 16777215).
    #[serde(default = "oxffffff_default")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub name_color: u64,

    /// The player's icon.
    #[serde(default)]
    pub icon: Option<Icon>,

    /// The player's current trophies.
    #[serde(default)]
    pub trophies: usize,

    /// The player's highest trophies amount.
    #[serde(default)]
    pub highest_trophies: usize,

    /// The player's total prestige level across all brawlers.
    #[serde(default)]
    pub total_prestige_level: usize,

    /// The player's experience level.
    #[serde(default = "one_default")]
    pub exp_level: usize,

    /// The player's experience points.
    #[serde(default)]
    pub exp_points: usize,

    /// Whether or not the Player was qualified from the Championship challenge.
    #[serde(default = "false_default")]
    pub is_qualified_from_championship_challenge: bool,

    /// Amount of 3v3 victories the Player has earned.
    #[serde(rename = "3vs3Victories")]
    pub tvt_victories: usize,

    /// The player's victories in solo showdown (how many times ranked #1).
    #[serde(default)]
    pub solo_victories: usize,

    /// The player's victories in duo showdown (how many times ranked #1).
    #[serde(default)]
    pub duo_victories: usize,

    /// The player's best Robo Rumble time, in seconds.
    #[serde(default)]
    pub best_robo_rumble_time: usize,

    /// The player's best time as a Big Brawler, in seconds.
    #[serde(default)]
    pub best_time_as_big_brawler: usize,

    /// The club the Player is in, or None if none.
    #[serde(default)]
    pub club: Option<PlayerClub>,

    /// The player's brawlers.
    #[serde(default)]
    pub brawlers: Vec<PlayerBrawlerStat>,
}
fn false_default() -> bool { false }

impl Default for Player {
    fn default() -> Player {
        Player {
            tag: String::from(""),
            name: String::from(""),
            name_color: 0xff_ff_ff,
            icon: None,
            trophies: 0,
            highest_trophies: 0,
            total_prestige_level: 0,
            exp_level: 1,
            exp_points: 0,
            is_qualified_from_championship_challenge: false,
            tvt_victories: 0,
            solo_victories: 0,
            duo_victories: 0,
            best_robo_rumble_time: 0,
            best_time_as_big_brawler: 0,
            club: None,
            brawlers: Vec::new(),
        }
    }
}

impl GetFetchProp for Player {
    type Property = str;

    fn get_fetch_prop(&self) -> &str { &*self.tag }

    fn get_route(tag: &str) -> Route { Route::Player(auto_hashtag(tag)) }
}

impl PropFetchable for Player {
    type Property = str;

    /// (Sync) Fetches a player from its tag.
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
    /// use brawl_api::{Client, Player, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let player = Player::fetch(&my_client, "#PLAYERTAGHERE")?;
    /// // now the data for the given player is available for use
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    fn fetch(client: &Client, tag: &str) -> Result<Player> {
        let route = Self::get_route(tag);
        fetch_route::<Player>(client, &route)
    }

    /// (Async) Fetches a player from its tag.
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
    /// use brawl_api::{Client, Player, traits::*};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let player = Player::a_fetch(&my_client, "#PLAYERTAGHERE").await?;
    /// // now the data for the given player is available for use
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error::Request`]: error/enum.Error.html#variant.Request
    /// [`Error::Status`]: error/enum.Error.html#variant.Status
    /// [`Error::Ratelimited`]: error/enum.Error.html#variant.Ratelimited
    /// [`Error::Json`]: error/enum.Error.html#variant.Json
    #[cfg(feature="async")]
    async fn a_fetch(client: &Client, tag: &str) -> Result<Player> {
        let route = Player::get_route(tag);
        a_fetch_route::<Player>(client, &route).await
    }
}

#[cfg(feature = "clubs")]
impl FetchFrom<ClubMember> for Player {
    /// (Sync) Fetches a `Player` instance, given a preexisting `ClubMember` instance.
    ///
    /// # Errors
    ///
    /// See [`Player::fetch`].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, Club, traits::*};
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let club = Club::fetch(&my_client, "#CLUB_TAG_HERE")?;
    /// let some_member = &club.members[0];
    /// let some_player = Player::fetch_from(&my_client, some_member)?;
    /// // now `some_member`'s full data, as a Player, is available for use.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Player::fetch`]: struct.Player.html#method.fetch
    fn fetch_from(client: &Client, member: &ClubMember) -> Result<Player> {
        Player::fetch(client, &member.tag)
    }

    /// (Async) Fetches a `Player` instance, given a preexisting `ClubMember` instance.
    ///
    /// # Errors
    ///
    /// See [`Player::fetch`].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{Client, Player, Club, traits::*};
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let club = Club::a_fetch(&my_client, "#CLUB_TAG_HERE").await?;
    /// let some_member = &club.members[0];
    /// let some_player = Player::a_fetch_from(&my_client, some_member).await?;
    /// // now `some_member`'s full data, as a Player, is available for use.
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Player::fetch`]: struct.Player.html#method.fetch
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, member: &ClubMember) -> Result<Player> {
        Player::a_fetch(client, &member.tag).await
    }
}

impl FetchFrom<BattlePlayer> for Player {
    /// (Async) Fetches a `Player` instance, given a preexisting `BattlePlayer` instance.
    ///
    /// # Errors
    ///
    /// See [`Player::fetch`].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{
    ///     Client, Player, BattleLog, Battle, BattleResultInfo, BattlePlayer,
    ///     traits::*
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let battlelog = BattleLog::fetch(&my_client, "#PLAYER_TAG_HERE")?;
    /// let most_recent_battle: Option<&Battle> = battlelog.get(0);
    ///
    /// if let Some(battle) = most_recent_battle {
    ///     if let Some(ref teams) = &battle.result.teams {
    ///         let some_b_player: &BattlePlayer = &teams[0][0];
    ///         let some_player = Player::fetch_from(&my_client, some_b_player)?;
    ///         // now `some_b_player`'s full data, as a Player, is available for use.
    ///     }
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Player::fetch`]: struct.Player.html#method.fetch
    fn fetch_from(client: &Client, b_player: &BattlePlayer) -> Result<Player> {
        Player::fetch(client, &b_player.tag)
    }

    /// (Async) Fetches a `Player` instance, given a preexisting `BattlePlayer` instance.
    ///
    /// # Errors
    ///
    /// See [`Player::fetch`].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brawl_api::{
    ///     Client, Player, BattleLog, Battle, BattleResultInfo, BattlePlayer,
    ///     traits::*
    /// };
    ///
    /// # async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    /// let my_client = Client::new("my auth token");
    /// let battlelog = BattleLog::a_fetch(&my_client, "#PLAYER_TAG_HERE").await?;
    /// let most_recent_battle: Option<&Battle> = battlelog.get(0);
    ///
    /// if let Some(battle) = most_recent_battle {
    ///     if let Some(ref teams) = &battle.result.teams {
    ///         let some_b_player: &BattlePlayer = &teams[0][0];
    ///         let some_player = Player::a_fetch_from(&my_client, some_b_player).await?;
    ///         // now `some_b_player`'s full data, as a Player, is available for use.
    ///     }
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Player::fetch`]: struct.Player.html#method.fetch
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, b_player: &BattlePlayer) -> Result<Player> {
        Player::a_fetch(client, &b_player.tag).await
    }
}

#[cfg(feature = "rankings")]
impl FetchFrom<PlayerRanking> for Player {

    /// (Sync) Fetches a `Player` using data from a [`PlayerRanking`] object.
    ///
    /// [`PlayerRanking`]: ../../rankings/players/struct.PlayerRanking.html
    fn fetch_from(client: &Client, p_ranking: &PlayerRanking) -> Result<Player> {
        Player::fetch(client, &p_ranking.tag)
    }

    /// (Async) Fetches a `Player` using data from a [`PlayerRanking`] object.
    ///
    /// [`PlayerRanking`]: ../../rankings/players/struct.PlayerRanking.html
    #[cfg(feature = "async")]
    async fn a_fetch_from(client: &Client, p_ranking: &PlayerRanking) -> Result<Player> {
        Player::a_fetch(client, &p_ranking.tag).await
    }
}


/// A struct representing a club obtained from [`Player.club`].
/// Note that it does not contain all of a club's information.
/// For that, use [`Club::fetch_from`] (fetches the full Club).
///
/// [`Player.club`]: ./struct.Player.html#structfield.club
/// [`Club::fetch_from`]: ../clubs/struct.Club.html#method.fetch_from
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerClub {

    /// The club's tag.
    #[serde(default)]
    pub tag: String,

    /// The club's name
    #[serde(default)]
    pub name: String
}

impl Default for PlayerClub {

    /// Returns an instance of `PlayerClub` with initial values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brawl_api::PlayerClub;
    ///
    /// assert_eq!(
    ///     PlayerClub::default(),
    ///     PlayerClub {
    ///         tag: String::from(""),
    ///         name: String::from(""),
    ///     }
    /// );
    /// ```
    fn default() -> PlayerClub {
        PlayerClub {
            tag: String::from(""),
            name: String::from("")
        }
    }
}

/// A struct containing information about a player's brawler (see [`Player.brawlers`]).
///
/// [`Player.brawlers`]: ./struct.Player.html#structfield.brawlers
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBrawlerStat {
    /// The brawler's id (an arbitrary number).
    #[serde(default)]
    pub id: usize,

    /// The brawler's name.
    #[serde(default)]
    pub name: String,

    /// The brawler's power (1-11).
    #[serde(default = "one_default")]
    pub power: u8,

    /// The brawler's rank.
    #[serde(default = "one_default")]
    pub rank: u16,

    /// The brawler's trophies.
    #[serde(default)]
    pub trophies: usize,

    /// The brawler's highest trophies amount.
    #[serde(default)]
    pub highest_trophies: usize,

    /// The brawler's prestige level.
    #[serde(default)]
    pub prestige_level: usize,

    /// The brawler's current win streak.
    #[serde(default)]
    pub current_win_streak: usize,

    /// The brawler's maximum win streak.
    #[serde(default)]
    pub max_win_streak: usize,

    /// The brawler's equipped skin, if any.
    #[serde(default)]
    pub skin: Option<Skin>,

    /// The brawler's star powers.
    #[serde(default)]
    pub star_powers: Vec<StarPower>,

    /// The brawler's gadgets.
    #[serde(default)]
    pub gadgets: Vec<Gadget>,

    /// The brawler's gears.
    #[serde(default)]
    pub gears: Vec<Gear>,

    /// The brawler's hyper charges.
    #[serde(default)]
    pub hyper_charges: Vec<HyperCharge>,

    /// Which ability upgrades the brawler has unlocked.
    #[serde(default)]
    pub buffies: Option<Buffies>,
}

impl Default for PlayerBrawlerStat {
    fn default() -> PlayerBrawlerStat {
        PlayerBrawlerStat {
            id: 0,
            name: String::from(""),
            power: 1,
            rank: 1,
            trophies: 0,
            highest_trophies: 0,
            prestige_level: 0,
            current_win_streak: 0,
            max_win_streak: 0,
            skin: None,
            star_powers: vec![],
            gadgets: vec![],
            gears: vec![],
            hyper_charges: vec![],
            buffies: None,
        }
    }
}

///////////////////////////////////   tests   ///////////////////////////////////

#[cfg(test)]
mod tests {
    use std::result::Result as StdResult;
    use super::{Player, PlayerClub, PlayerBrawlerStat, StarPower};
    use crate::model::common::{Icon, Gadget, Gear, HyperCharge, Skin, Buffies};
    use crate::error::Error as BrawlError;
    use serde_json;

    #[test]
    fn players_deser() -> StdResult<(), Box<dyn ::std::error::Error>> {
        let player_json_s = r##"{
  "tag": "#CCCCCC",
  "name": "User",
  "nameColor": "0xff1ba5f5",
  "trophies": 13370,
  "highestTrophies": 30000,
  "powerPlayPoints": 200,
  "highestPowerPlayPoints": 900,
  "expLevel": 100,
  "expPoints": 70000,
  "isQualifiedFromChampionshipChallenge": false,
  "3vs3Victories": 3333,
  "soloVictories": 999,
  "duoVictories": 333,
  "bestRoboRumbleTime": 350,
  "bestTimeAsBigBrawler": 250,
  "club": {
    "tag": "#888888",
    "name": "Club"
  },
  "brawlers": [
    {
      "id": 16000000,
      "name": "SHELLY",
      "power": 9,
      "rank": 20,
      "trophies": 500,
      "highestTrophies": 549,
      "starPowers": []
    },
    {
      "id": 16000001,
      "name": "COLT",
      "power": 10,
      "rank": 18,
      "trophies": 420,
      "highestTrophies": 440,
      "starPowers": [
        {
          "id": 23000138,
          "name": "Magnum Special"
        },
        {
          "id": 23000077,
          "name": "Slick Boots"
        }
      ]
    }
  ]
}"##;
        let player: Player = serde_json::from_str::<Player>(player_json_s)
            .map_err(BrawlError::Json)?;

        assert_eq!(
            player,
            Player {
                tag: String::from("#CCCCCC"),
                name: String::from("User"),
                name_color: 0xff1ba5f5,
                trophies: 13370,
                highest_trophies: 30000,
                exp_level: 100,
                exp_points: 70000,
                is_qualified_from_championship_challenge: false,
                tvt_victories: 3333,
                solo_victories: 999,
                duo_victories: 333,
                best_robo_rumble_time: 350,
                best_time_as_big_brawler: 250,
                club: Some(PlayerClub {
                    tag: String::from("#888888"),
                    name: String::from("Club")
                }),
                brawlers: vec![
                    PlayerBrawlerStat {
                        id: 16000000,
                        name: String::from("SHELLY"),
                        power: 9,
                        rank: 20,
                        trophies: 500,
                        highest_trophies: 549,
                        star_powers: vec![],
                        ..PlayerBrawlerStat::default()
                    },
                    PlayerBrawlerStat {
                        id: 16000001,
                        name: String::from("COLT"),
                        power: 10,
                        rank: 18,
                        trophies: 420,
                        highest_trophies: 440,
                        star_powers: vec![
                            StarPower {
                                id: 23000138,
                                name: String::from("Magnum Special")
                            },
                            StarPower {
                                id: 23000077,
                                name: String::from("Slick Boots")
                            }
                        ],
                        ..PlayerBrawlerStat::default()
                    },
                ],
                ..Player::default()
            }
        );
        Ok(())
    }

    #[test]
    fn player_new_fields_deser() -> StdResult<(), Box<dyn ::std::error::Error>> {
        let json = r##"{
  "tag": "#PLAYER123",
  "name": "ExamplePlayer",
  "nameColor": "0xfff9c908",
  "icon": { "id": 28000272 },
  "trophies": 67383,
  "highestTrophies": 67383,
  "totalPrestigeLevel": 18,
  "expLevel": 314,
  "expPoints": 502341,
  "isQualifiedFromChampionshipChallenge": false,
  "3vs3Victories": 28058,
  "soloVictories": 899,
  "duoVictories": 2089,
  "bestRoboRumbleTime": 20,
  "bestTimeAsBigBrawler": 0,
  "club": { "tag": "#CLUB123", "name": "Example Club" },
  "brawlers": [{
    "id": 16000000,
    "name": "SHELLY",
    "power": 11,
    "rank": 4,
    "trophies": 675,
    "highestTrophies": 807,
    "prestigeLevel": 0,
    "currentWinStreak": 1,
    "maxWinStreak": 1,
    "skin": { "id": 29000844, "name": "SQUAD BUSTER\nSHELLY" },
    "gadgets": [
      { "id": 23000255, "name": "FAST FORWARD" },
      { "id": 23000288, "name": "CLAY PIGEONS" }
    ],
    "gears": [
      { "id": 62000002, "name": "DAMAGE", "level": 3 },
      { "id": 62000000, "name": "SPEED", "level": 3 }
    ],
    "starPowers": [
      { "id": 23000076, "name": "SHELL SHOCK" },
      { "id": 23000135, "name": "BAND-AID" }
    ],
    "hyperCharges": [
      { "id": 23000613, "name": "DOUBLE BARREL" }
    ],
    "buffies": {
      "gadget": true,
      "starPower": true,
      "hyperCharge": true
    }
  }]
}"##;
        let player: Player = serde_json::from_str(json).map_err(BrawlError::Json)?;

        assert_eq!(player.tag, "#PLAYER123");
        assert_eq!(player.icon, Some(Icon { id: 28000272 }));
        assert_eq!(player.total_prestige_level, 18);
        assert_eq!(player.trophies, 67383);
        assert_eq!(player.exp_level, 314);

        let brawler = &player.brawlers[0];
        assert_eq!(brawler.power, 11);
        assert_eq!(brawler.prestige_level, 0);
        assert_eq!(brawler.current_win_streak, 1);
        assert_eq!(brawler.max_win_streak, 1);
        assert_eq!(brawler.skin, Some(Skin {
            id: 29000844,
            name: String::from("SQUAD BUSTER\nSHELLY"),
        }));
        assert_eq!(brawler.gadgets.len(), 2);
        assert_eq!(brawler.gadgets[0], Gadget { id: 23000255, name: String::from("FAST FORWARD") });
        assert_eq!(brawler.gears.len(), 2);
        assert_eq!(brawler.gears[0], Gear { id: 62000002, name: String::from("DAMAGE"), level: 3 });
        assert_eq!(brawler.hyper_charges.len(), 1);
        assert_eq!(brawler.hyper_charges[0], HyperCharge { id: 23000613, name: String::from("DOUBLE BARREL") });
        assert_eq!(brawler.buffies, Some(Buffies {
            gadget: true,
            star_power: true,
            hyper_charge: true,
        }));

        Ok(())
    }

    #[test]
    fn player_missing_optional_fields() -> StdResult<(), Box<dyn ::std::error::Error>> {
        let json = r##"{
  "tag": "#MINIMAL",
  "name": "MinPlayer",
  "nameColor": "0xffffffff",
  "trophies": 100,
  "highestTrophies": 100,
  "expLevel": 1,
  "expPoints": 0,
  "isQualifiedFromChampionshipChallenge": false,
  "3vs3Victories": 0,
  "soloVictories": 0,
  "duoVictories": 0,
  "bestRoboRumbleTime": 0,
  "bestTimeAsBigBrawler": 0,
  "brawlers": []
}"##;
        let player: Player = serde_json::from_str(json).map_err(BrawlError::Json)?;

        assert_eq!(player.tag, "#MINIMAL");
        assert_eq!(player.icon, None);
        assert_eq!(player.total_prestige_level, 0);
        assert_eq!(player.club, None);
        assert!(player.brawlers.is_empty());

        Ok(())
    }

    #[test]
    fn player_roundtrip_serialization() -> StdResult<(), Box<dyn ::std::error::Error>> {
        let player = Player {
            tag: String::from("#TEST"),
            name: String::from("Test"),
            icon: Some(Icon { id: 28000001 }),
            total_prestige_level: 5,
            ..Player::default()
        };

        let json = serde_json::to_string(&player)?;
        let deser: Player = serde_json::from_str(&json)?;
        assert_eq!(player, deser);

        Ok(())
    }
}


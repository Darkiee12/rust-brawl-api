//! Models shared for usage by more than one endpoint. Note that, if all the relevant endpoints'
//! features are disabled, then the respective models here are also disabled.
//!
//! Field names and shapes mirror the official Brawl Stars API responses:
//! https://developer.brawlstars.com/#/documentation

use serde::{self, Serialize, Deserialize};

/// A struct representing a player or brawler icon.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Icon {
    /// The icon's id.
    #[serde(default)]
    pub id: usize,
}

/// A struct representing a brawler's star power.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg(any(feature = "players", feature = "brawlers"))]
pub struct StarPower {
    /// The star power's id (an arbitrary number).
    #[serde(default)]
    pub id: usize,

    /// The star power name.
    #[serde(default)]
    pub name: String,
}

/// A struct representing a brawler's gadget.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg(any(feature = "players", feature = "brawlers"))]
pub struct Gadget {
    /// The gadget's id.
    #[serde(default)]
    pub id: usize,

    /// The gadget's name.
    #[serde(default)]
    pub name: String,
}

/// A struct representing a brawler's gear.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg(any(feature = "players", feature = "brawlers"))]
pub struct Gear {
    /// The gear's id.
    #[serde(default)]
    pub id: usize,

    /// The gear's name.
    #[serde(default)]
    pub name: String,

    /// The gear's level.
    #[serde(default)]
    pub level: u8,
}

/// A struct representing a brawler's hyper charge.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg(any(feature = "players", feature = "brawlers"))]
pub struct HyperCharge {
    /// The hyper charge's id.
    #[serde(default)]
    pub id: usize,

    /// The hyper charge's name.
    #[serde(default)]
    pub name: String,
}

/// A struct representing a brawler's equipped skin (from player brawler stats).
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg(feature = "players")]
pub struct Skin {
    /// The skin's id.
    #[serde(default)]
    pub id: usize,

    /// The skin's name.
    #[serde(default)]
    pub name: String,
}

/// A struct representing which ability upgrades a brawler has unlocked.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg(feature = "players")]
pub struct Buffies {
    /// Whether the brawler has a gadget unlocked.
    #[serde(default)]
    pub gadget: bool,

    /// Whether the brawler has a star power unlocked.
    #[serde(default)]
    pub star_power: bool,

    /// Whether the brawler has a hyper charge unlocked.
    #[serde(default)]
    pub hyper_charge: bool,
}

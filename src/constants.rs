//! Contains constant values used within the lib.

/// The initial URL path to the Brawl Stars API v1.
pub const API_URI: &str = "https://api.brawlstars.com/v1/";

/// The user agent to use indicating this lib was used to request.
pub const USER_AGENT: &str = concat!(
    "Rust (brawl-api crate, ", env!("CARGO_PKG_VERSION"),
    " - https://github.com/PgBiel/rust-brawl-api)"
);

/// The format used in [`TimeLike.parse`]. (Feature-gated with the `chrono` feature)
///
/// `"%Y%m%dT%H%M%S%.fZ"`
///
/// See [this table] for more info.
///
/// [`TimeLike.parse`]: ../time/struct.TimeLike.html#method.parse
/// [this table]: https://docs.rs/chrono/*/chrono/format/strftime/index.html
#[cfg(feature = "chrono")]
pub const TIMELIKE_FORMAT: &str = "%Y%m%dT%H%M%S%.fZ";

/// Maps human-readable brawler names to their API IDs.
/// Use by casting to `usize`: e.g. `Brawlers::Shelly as usize`.
///
/// This enum is updated to match the brawlers endpoint but may lag behind new releases.
/// For an always-current list, fetch the `/brawlers/` endpoint at runtime.
/// 
/// This is by no means a final enum and must be updated on every new Brawler release.
///
/// If a permanently up-to-date list is needed, one can fetch the `/brawlers/` endpoint using
/// the available models. If still using this enum, though, rest assured that we will do our best
/// to keep it updated - if it is not, why not contribute with a PR? ;)
#[non_exhaustive]
#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Brawlers {
    Shelly = 16000000,
    Colt = 16000001,
    Bull = 16000002,
    Brock = 16000003,
    Rico = 16000004,
    Spike = 16000005,
    Barley = 16000006,
    Jessie = 16000007,
    Nita = 16000008,
    Dynamike = 16000009,
    ElPrimo = 16000010,
    Mortis = 16000011,
    Crow = 16000012,
    Poco = 16000013,
    Bo = 16000014,
    Piper = 16000015,
    Pam = 16000016,
    Tara = 16000017,
    Darryl = 16000018,
    Penny = 16000019,
    Frank = 16000020,
    Gene = 16000021,
    Tick = 16000022,
    Leon = 16000023,
    Rosa = 16000024,
    Carl = 16000025,
    Bibi = 16000026,
    EightBit = 16000027,
    Sandy = 16000028,
    Bea = 16000029,
    Emz = 16000030,
    MrP = 16000031,
    Max = 16000032,
    Jacky = 16000034,
    Gale = 16000035,
    Nani = 16000036,
    Sprout = 16000037,
    Surge = 16000038,
    Colette = 16000039,
    Amber = 16000040,
    Lou = 16000041,
    Byron = 16000042,
    Edgar = 16000043,
    Ruffs = 16000044,
    Stu = 16000045,
    Belle = 16000046,
    Squeak = 16000047,
    Grom = 16000048,
    Buzz = 16000049,
    Griff = 16000050,
    Ash = 16000051,
    Meg = 16000052,
    Lola = 16000053,
    Fang = 16000054,
    Eve = 16000056,
    Janet = 16000057,
    Bonnie = 16000058,
    Otis = 16000059,
    Sam = 16000060,
    Gus = 16000061,
    Buster = 16000062,
    Chester = 16000063,
    Gray = 16000064,
    Mandy = 16000065,
    RT = 16000066,
    Willow = 16000067,
    Maisie = 16000068,
    Hank = 16000069,
    Cordelius = 16000070,
    Doug = 16000071,
    Pearl = 16000072,
    Chuck = 16000073,
    Charlie = 16000074,
    Mico = 16000075,
    Kit = 16000076,
    LarryAndLawrie = 16000077,
    Melodie = 16000078,
    Angelo = 16000079,
    Draco = 16000080,
    Lily = 16000081,
    Berry = 16000082,
    Clancy = 16000083,
    Moe = 16000084,
    Kenji = 16000085,
    Shade = 16000086,
    Juju = 16000087,
    Meeple = 16000089,
    Ollie = 16000090,
    Lumi = 16000091,
    Finx = 16000092,
    JaeYong = 16000093,
    Kaze = 16000094,
    Alli = 16000095,
    Trunk = 16000096,
    Mina = 16000097,
    Ziggy = 16000098,
    Pierce = 16000099,
    Gigi = 16000100,
    Glowbert = 16000101,
    Sirius = 16000102,
    Najia = 16000103,
}

/// Game modes as returned by the API's `modeId` field.
///
/// Not every mode is always active; some rotate in and out of events.
#[non_exhaustive]
#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq)]
pub enum GameMode {
    GemGrab = 0,
    Heist = 2,
    Bounty = 3,
    BrawlBall = 5,
    SoloShowdown = 6,
    DuoShowdown = 7,
    BigGame = 8,
    RoboRumble = 9,
    Takedown = 10,
    LoneStar = 11,
    PresentPlunder = 12,
    HotZone = 17,
    SuperCityRampage = 18,
    Knockout = 20,
    Basketbrawl = 22,
    Volleyball = 23,
    Wipeout = 25,
    Payload = 26,
    BrawlBall5v5 = 27,
    GemGrab5v5 = 28,
    Knockout5v5 = 29,
    Wipeout5v5 = 30,
    PaintBrawl = 36,
}

/// Gear types available for brawlers.
#[non_exhaustive]
#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq)]
pub enum GearType {
    Speed = 62000000,
    Health = 62000001,
    Damage = 62000002,
    Vision = 62000003,
    Shield = 62000004,
    ReloadSpeed = 62000005,
    SuperCharge = 62000006,
    ThiccHead = 62000007,
    TalkToTheHand = 62000008,
    EnduringToxins = 62000009,
    StickySpikes = 62000010,
    LingeringSmoke = 62000011,
    ExhaustingStorm = 62000012,
    StickyOil = 62000013,
    PetPower = 62000014,
    Quadruplets = 62000015,
    SuperTurret = 62000016,
    GadgetCooldown = 62000017,
    BatStorm = 62000018,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brawler_ids_correct() {
        assert_eq!(Brawlers::Shelly as usize, 16000000);
        assert_eq!(Brawlers::Colt as usize, 16000001);
        assert_eq!(Brawlers::EightBit as usize, 16000027);
        assert_eq!(Brawlers::MrP as usize, 16000031);
        assert_eq!(Brawlers::Max as usize, 16000032);
        assert_eq!(Brawlers::Jacky as usize, 16000034);
        assert_eq!(Brawlers::Najia as usize, 16000103);
    }

    #[test]
    fn game_mode_ids_correct() {
        assert_eq!(GameMode::GemGrab as usize, 0);
        assert_eq!(GameMode::Heist as usize, 2);
        assert_eq!(GameMode::Bounty as usize, 3);
        assert_eq!(GameMode::BrawlBall as usize, 5);
        assert_eq!(GameMode::SoloShowdown as usize, 6);
        assert_eq!(GameMode::HotZone as usize, 17);
        assert_eq!(GameMode::Knockout as usize, 20);
    }

    #[test]
    fn gear_type_ids_correct() {
        assert_eq!(GearType::Speed as usize, 62000000);
        assert_eq!(GearType::Health as usize, 62000001);
        assert_eq!(GearType::Damage as usize, 62000002);
        assert_eq!(GearType::GadgetCooldown as usize, 62000017);
    }

    #[test]
    fn brawler_enum_copy_clone() {
        let shelly = Brawlers::Shelly;
        let shelly2 = shelly;
        assert_eq!(shelly, shelly2);
    }
}

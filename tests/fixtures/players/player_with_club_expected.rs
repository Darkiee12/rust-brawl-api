use crate::model::common::{Buffies, Gadget, Icon, Skin};
use crate::model::players::player::{Player, PlayerBrawlerStat, PlayerClub};

pub fn expected() -> Player {
    Player {
        tag: String::from("#AAAAAAA"),
        name: String::from("TestPlayer1"),
        name_color: 0xff1ba5f5,
        icon: Some(Icon { id: 28000151 }),
        trophies: 16817,
        highest_trophies: 17071,
        total_prestige_level: 0,
        exp_level: 121,
        exp_points: 77399,
        is_qualified_from_championship_challenge: false,
        tvt_victories: 2735,
        solo_victories: 274,
        duo_victories: 446,
        best_robo_rumble_time: 5,
        best_time_as_big_brawler: 0,
        club: Some(PlayerClub {
            tag: String::from("#CCCCCCC"),
            name: String::from("TestClub1"),
        }),
        brawlers: vec![PlayerBrawlerStat {
            id: 16000000,
            name: String::from("SHELLY"),
            power: 9,
            rank: 2,
            trophies: 422,
            highest_trophies: 434,
            prestige_level: 0,
            current_win_streak: 0,
            max_win_streak: 0,
            skin: Some(Skin {
                id: 29000159,
                name: String::from("TEST_SKIN_1"),
            }),
            star_powers: vec![],
            gadgets: vec![Gadget {
                id: 23000288,
                name: String::from("TEST_GADGET_1"),
            }],
            gears: vec![],
            hyper_charges: vec![],
            buffies: Buffies {
                gadget: false,
                star_power: false,
                hyper_charge: false,
            },
        }],
    }
}

use crate::model::common::{Buffies, Gadget, Gear, HyperCharge, Icon, Skin, StarPower};
use crate::model::players::player::{Player, PlayerBrawlerStat, PlayerClub};

pub fn expected() -> Player {
    Player {
        tag: String::from("#BBBBBBB"),
        name: String::from("TestPlayer2"),
        name_color: 0xfff9c908,
        icon: Some(Icon { id: 28000272 }),
        trophies: 67383,
        highest_trophies: 67383,
        total_prestige_level: 18,
        exp_level: 314,
        exp_points: 502341,
        is_qualified_from_championship_challenge: false,
        tvt_victories: 28058,
        solo_victories: 899,
        duo_victories: 2089,
        best_robo_rumble_time: 20,
        best_time_as_big_brawler: 0,
        club: Some(PlayerClub::default()),
        brawlers: vec![PlayerBrawlerStat {
            id: 16000000,
            name: String::from("SHELLY"),
            power: 11,
            rank: 4,
            trophies: 675,
            highest_trophies: 807,
            prestige_level: 0,
            current_win_streak: 1,
            max_win_streak: 1,
            skin: Some(Skin {
                id: 29000844,
                name: String::from("TEST_SKIN_2"),
            }),
            star_powers: vec![
                StarPower {
                    id: 23000076,
                    name: String::from("TEST_STARPOWER_1"),
                },
                StarPower {
                    id: 23000135,
                    name: String::from("TEST_STARPOWER_2"),
                },
            ],
            gadgets: vec![
                Gadget {
                    id: 23000255,
                    name: String::from("TEST_GADGET_2"),
                },
                Gadget {
                    id: 23000288,
                    name: String::from("TEST_GADGET_3"),
                },
            ],
            gears: vec![
                Gear {
                    id: 62000002,
                    name: String::from("TEST_GEAR_1"),
                    level: 3,
                },
                Gear {
                    id: 62000000,
                    name: String::from("TEST_GEAR_2"),
                    level: 3,
                },
            ],
            hyper_charges: vec![HyperCharge {
                id: 23000613,
                name: String::from("TEST_HYPERCHARGE_1"),
            }],
            buffies: Buffies {
                gadget: true,
                star_power: true,
                hyper_charge: true,
            },
        }],
    }
}

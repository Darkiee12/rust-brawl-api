use crate::model::players::battlelog::{
    Battle, BattleBrawler, BattleEvent, BattleLog, BattleOutcome, BattlePlayer, BattleResultInfo,
};
use crate::time::TimeLike;

pub fn expected() -> BattleLog {
    BattleLog {
        tag: String::from(""),
        items: vec![Battle {
            battle_time: TimeLike(String::from("20250401T120000.000Z")),
            event: BattleEvent {
                id: 15000005,
                mode: String::from("bounty"),
                mode_id: None,
                map: String::from("Shooting Star"),
            },
            result: BattleResultInfo {
                mode: String::from("bounty"),
                battle_type: Some(String::from("ranked")),
                duration: 90,
                trophy_change: -5,
                rank: None,
                result: Some(BattleOutcome::Defeat),
                star_player: Some(BattlePlayer {
                    tag: String::from("#BNT0004"),
                    name: String::from("TestPlayerB4"),
                    brawler: BattleBrawler {
                        id: 16000001,
                        name: String::from("TEST_BRAWLER_B4"),
                        power: 11,
                        trophies: 530,
                    },
                }),
                teams: Some(vec![
                    vec![
                        BattlePlayer {
                            tag: String::from("#BNT0001"),
                            name: String::from("TestPlayerB1"),
                            brawler: BattleBrawler {
                                id: 16000019,
                                name: String::from("TEST_BRAWLER_B1"),
                                power: 10,
                                trophies: 520,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#BNT0002"),
                            name: String::from("TestPlayerB2"),
                            brawler: BattleBrawler {
                                id: 16000017,
                                name: String::from("TEST_BRAWLER_B2"),
                                power: 11,
                                trophies: 518,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#BNT0003"),
                            name: String::from("TestPlayerB3"),
                            brawler: BattleBrawler {
                                id: 16000023,
                                name: String::from("TEST_BRAWLER_B3"),
                                power: 11,
                                trophies: 515,
                            },
                        },
                    ],
                    vec![
                        BattlePlayer {
                            tag: String::from("#BNT0004"),
                            name: String::from("TestPlayerB4"),
                            brawler: BattleBrawler {
                                id: 16000001,
                                name: String::from("TEST_BRAWLER_B4"),
                                power: 11,
                                trophies: 530,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#BNT0005"),
                            name: String::from("TestPlayerB5"),
                            brawler: BattleBrawler {
                                id: 16000045,
                                name: String::from("TEST_BRAWLER_B5"),
                                power: 10,
                                trophies: 526,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#BNT0006"),
                            name: String::from("TestPlayerB6"),
                            brawler: BattleBrawler {
                                id: 16000020,
                                name: String::from("TEST_BRAWLER_B6"),
                                power: 10,
                                trophies: 522,
                            },
                        },
                    ],
                ]),
                players: None,
            },
        }],
    }
}

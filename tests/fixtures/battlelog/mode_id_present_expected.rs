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
                id: 15000007,
                mode: String::from("gemGrab"),
                mode_id: Some(0),
                map: String::from("Hard Rock Mine"),
            },
            result: BattleResultInfo {
                mode: String::from("gemGrab"),
                battle_type: Some(String::from("ranked")),
                duration: 120,
                trophy_change: 10,
                rank: None,
                result: Some(BattleOutcome::Victory),
                star_player: Some(BattlePlayer {
                    tag: String::from("#GEM0001"),
                    name: String::from("TestPlayerG1"),
                    brawler: BattleBrawler {
                        id: 16000019,
                        name: String::from("TEST_BRAWLER_G1"),
                        power: 11,
                        trophies: 520,
                    },
                }),
                teams: Some(vec![
                    vec![
                        BattlePlayer {
                            tag: String::from("#GEM0001"),
                            name: String::from("TestPlayerG1"),
                            brawler: BattleBrawler {
                                id: 16000019,
                                name: String::from("TEST_BRAWLER_G1"),
                                power: 11,
                                trophies: 520,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#GEM0002"),
                            name: String::from("TestPlayerG2"),
                            brawler: BattleBrawler {
                                id: 16000023,
                                name: String::from("TEST_BRAWLER_G2"),
                                power: 10,
                                trophies: 515,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#GEM0003"),
                            name: String::from("TestPlayerG3"),
                            brawler: BattleBrawler {
                                id: 16000005,
                                name: String::from("TEST_BRAWLER_G3"),
                                power: 11,
                                trophies: 510,
                            },
                        },
                    ],
                    vec![
                        BattlePlayer {
                            tag: String::from("#GEM0004"),
                            name: String::from("TestPlayerG4"),
                            brawler: BattleBrawler {
                                id: 16000012,
                                name: String::from("TEST_BRAWLER_G4"),
                                power: 11,
                                trophies: 500,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#GEM0005"),
                            name: String::from("TestPlayerG5"),
                            brawler: BattleBrawler {
                                id: 16000026,
                                name: String::from("TEST_BRAWLER_G5"),
                                power: 10,
                                trophies: 498,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#GEM0006"),
                            name: String::from("TestPlayerG6"),
                            brawler: BattleBrawler {
                                id: 16000014,
                                name: String::from("TEST_BRAWLER_G6"),
                                power: 11,
                                trophies: 495,
                            },
                        },
                    ],
                ]),
                players: None,
            },
        }],
    }
}

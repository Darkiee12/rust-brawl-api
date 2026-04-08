use crate::model::players::battlelog::{
    Battle, BattleBrawler, BattleEvent, BattleLog, BattleOutcome, BattlePlayer, BattleResultInfo,
};
use crate::time::TimeLike;

pub fn expected() -> BattleLog {
    BattleLog {
        tag: String::from(""),
        items: vec![Battle {
            battle_time: TimeLike(String::from("20241218T021311.000Z")),
            event: BattleEvent {
                id: 0,
                mode: String::from("bounty"),
                mode_id: Some(3),
                map: String::from(""),
            },
            result: BattleResultInfo {
                mode: String::from("bounty"),
                battle_type: Some(String::from("friendly")),
                duration: 120,
                trophy_change: 0,
                rank: None,
                result: Some(BattleOutcome::Victory),
                star_player: None,
                teams: Some(vec![
                    vec![
                        BattlePlayer {
                            tag: String::from("#CCC0001"),
                            name: String::from("TestPlayer7"),
                            brawler: BattleBrawler {
                                id: 16000019,
                                name: String::from("TEST_BRAWLER_7"),
                                power: 10,
                                trophies: 100,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#CCC0002"),
                            name: String::from("TestPlayer8"),
                            brawler: BattleBrawler {
                                id: 16000043,
                                name: String::from("TEST_BRAWLER_8"),
                                power: 10,
                                trophies: 101,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#CCC0003"),
                            name: String::from("TestPlayer9"),
                            brawler: BattleBrawler {
                                id: 16000023,
                                name: String::from("TEST_BRAWLER_9"),
                                power: 10,
                                trophies: 102,
                            },
                        },
                    ],
                    vec![
                        BattlePlayer {
                            tag: String::from("#DDD0001"),
                            name: String::from("TestPlayer10"),
                            brawler: BattleBrawler {
                                id: 16000001,
                                name: String::from("TEST_BRAWLER_10"),
                                power: 10,
                                trophies: 99,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#DDD0002"),
                            name: String::from("TestPlayer11"),
                            brawler: BattleBrawler {
                                id: 16000008,
                                name: String::from("TEST_BRAWLER_11"),
                                power: 10,
                                trophies: 98,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#DDD0003"),
                            name: String::from("TestPlayer12"),
                            brawler: BattleBrawler {
                                id: 16000045,
                                name: String::from("TEST_BRAWLER_12"),
                                power: 10,
                                trophies: 97,
                            },
                        },
                    ],
                ]),
                players: None,
            },
        }],
    }
}

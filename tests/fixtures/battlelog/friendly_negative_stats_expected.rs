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
                            tag: String::from("#BBBBBBB"),
                            name: String::from("TestPlayer2"),
                            brawler: BattleBrawler {
                                id: 16000019,
                                name: String::from("TEST_BRAWLER"),
                                power: 0,
                                trophies: 0,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#EEE0002"),
                            name: String::from("TestPlayer13"),
                            brawler: BattleBrawler {
                                id: 16000001,
                                name: String::from("TEST_BRAWLER_13"),
                                power: 0,
                                trophies: 0,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#EEE0003"),
                            name: String::from("TestPlayer14"),
                            brawler: BattleBrawler {
                                id: 16000004,
                                name: String::from("TEST_BRAWLER_14"),
                                power: 0,
                                trophies: 0,
                            },
                        },
                    ],
                    vec![
                        BattlePlayer {
                            tag: String::from("#FFF0001"),
                            name: String::from("TestPlayer15"),
                            brawler: BattleBrawler {
                                id: 16000011,
                                name: String::from("TEST_BRAWLER_15"),
                                power: 0,
                                trophies: 0,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#FFF0002"),
                            name: String::from("TestPlayer16"),
                            brawler: BattleBrawler {
                                id: 16000038,
                                name: String::from("TEST_BRAWLER_16"),
                                power: 0,
                                trophies: 0,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#FFF0003"),
                            name: String::from("TestPlayer17"),
                            brawler: BattleBrawler {
                                id: 16000024,
                                name: String::from("TEST_BRAWLER_17"),
                                power: 0,
                                trophies: 0,
                            },
                        },
                    ],
                ]),
                players: None,
            },
        }],
    }
}

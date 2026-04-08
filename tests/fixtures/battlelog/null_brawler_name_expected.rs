use crate::model::players::battlelog::{
    Battle, BattleBrawler, BattleEvent, BattleLog, BattleOutcome, BattlePlayer, BattleResultInfo,
};
use crate::time::TimeLike;

pub fn expected() -> BattleLog {
    BattleLog {
        tag: String::from(""),
        items: vec![Battle {
            battle_time: TimeLike(String::from("20250201T233210.000Z")),
            event: BattleEvent {
                id: 15000007,
                mode: String::from("gemGrab"),
                mode_id: Some(0),
                map: String::from("TestMap1"),
            },
            result: BattleResultInfo {
                mode: String::from("gemGrab"),
                battle_type: Some(String::from("championshipChallenge")),
                duration: 145,
                trophy_change: 0,
                rank: None,
                result: Some(BattleOutcome::Defeat),
                star_player: Some(BattlePlayer {
                    tag: String::from("#STAR001"),
                    name: String::from("TestPlayerStar"),
                    brawler: BattleBrawler {
                        id: 16000068,
                        name: String::from("TEST_STAR_BRAWLER"),
                        power: 11,
                        trophies: 200,
                    },
                }),
                teams: Some(vec![
                    vec![
                        BattlePlayer {
                            tag: String::from("#AAAAAAA"),
                            name: String::from("TestPlayer1"),
                            brawler: BattleBrawler {
                                id: 16000088,
                                name: String::from("UNKNOWN"),
                                power: 11,
                                trophies: 200,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#AAA0002"),
                            name: String::from("TestPlayer2"),
                            brawler: BattleBrawler {
                                id: 16000023,
                                name: String::from("TEST_BRAWLER_2"),
                                power: 11,
                                trophies: 201,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#AAA0003"),
                            name: String::from("TestPlayer3"),
                            brawler: BattleBrawler {
                                id: 16000005,
                                name: String::from("TEST_BRAWLER_3"),
                                power: 11,
                                trophies: 202,
                            },
                        },
                    ],
                    vec![
                        BattlePlayer {
                            tag: String::from("#BBB0001"),
                            name: String::from("TestPlayer4"),
                            brawler: BattleBrawler {
                                id: 16000026,
                                name: String::from("TEST_BRAWLER_4"),
                                power: 11,
                                trophies: 200,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#BBB0002"),
                            name: String::from("TestPlayer5"),
                            brawler: BattleBrawler {
                                id: 16000014,
                                name: String::from("TEST_BRAWLER_5"),
                                power: 11,
                                trophies: 203,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#BBB0003"),
                            name: String::from("TestPlayer6"),
                            brawler: BattleBrawler {
                                id: 16000019,
                                name: String::from("TEST_BRAWLER_6"),
                                power: 11,
                                trophies: 204,
                            },
                        },
                    ],
                ]),
                players: None,
            },
        }],
    }
}

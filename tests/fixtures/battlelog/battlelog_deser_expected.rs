use crate::model::players::battlelog::{
    Battle, BattleBrawler, BattleEvent, BattleLog, BattleOutcome, BattlePlayer, BattleResultInfo,
};
use crate::time::TimeLike;

pub fn expected() -> BattleLog {
    BattleLog {
        tag: String::from(""),
        items: vec![Battle {
            battle_time: TimeLike(String::from("20200131T003432.000Z")),
            event: BattleEvent {
                id: 15000163,
                mode: String::from("brawlBall"),
                mode_id: None,
                map: String::from("Coarse Course"),
            },
            result: BattleResultInfo {
                mode: String::from("brawlBall"),
                battle_type: Some(String::from("ranked")),
                duration: 96,
                trophy_change: 8,
                rank: None,
                result: Some(BattleOutcome::Victory),
                star_player: Some(BattlePlayer {
                    tag: String::from("#CCCCCCCC"),
                    name: String::from("User"),
                    brawler: BattleBrawler {
                        id: 16000008,
                        name: String::from("NITA"),
                        power: 10,
                        trophies: 500,
                    },
                }),
                teams: Some(vec![
                    vec![
                        BattlePlayer {
                            tag: String::from("#CCCCCCCC"),
                            name: String::from("User"),
                            brawler: BattleBrawler {
                                id: 16000008,
                                name: String::from("NITA"),
                                power: 10,
                                trophies: 500,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#RRRAAALLL"),
                            name: String::from("Other User"),
                            brawler: BattleBrawler {
                                id: 16000001,
                                name: String::from("COLT"),
                                power: 8,
                                trophies: 510,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#GGGGGGGGG"),
                            name: String::from("Another User"),
                            brawler: BattleBrawler {
                                id: 16000018,
                                name: String::from("DARRYL"),
                                power: 10,
                                trophies: 520,
                            },
                        },
                    ],
                    vec![
                        BattlePlayer {
                            tag: String::from("#777777777"),
                            name: String::from("User User User"),
                            brawler: BattleBrawler {
                                id: 16000032,
                                name: String::from("MAX"),
                                power: 10,
                                trophies: 500,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#SUVSUVSUV"),
                            name: String::from("User.User?!\u{f8ff}"),
                            brawler: BattleBrawler {
                                id: 16000024,
                                name: String::from("ROSA"),
                                power: 9,
                                trophies: 400,
                            },
                        },
                        BattlePlayer {
                            tag: String::from("#QCPJ09J"),
                            name: String::from(
                                "\u{043f}\u{043e}\u{043b}\u{044c}\u{0437}\u{043e}\u{0432}\u{0430}\u{0442}\u{0435}\u{043b}\u{044c}",
                            ),
                            brawler: BattleBrawler {
                                id: 16000028,
                                name: String::from("SANDY"),
                                power: 10,
                                trophies: 450,
                            },
                        },
                    ],
                ]),
                players: None,
            },
        }],
    }
}

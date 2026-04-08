use std::collections::BTreeMap;

use crate::model::events::{JsonLocalizedName, ScheduledEvent, ScheduledEventLocation, ScheduledEvents};

pub fn expected() -> ScheduledEvents {
    let mut map_values = BTreeMap::new();
    map_values.insert(String::from("en"), String::from("TestMap1"));

    ScheduledEvents {
        items: vec![ScheduledEvent {
            slot_id: 1,
            event: ScheduledEventLocation {
                mode: String::from("gemGrab"),
                mode_id: Some(0),
                modifiers: vec![String::from("none")],
                id: 15000001,
                map: JsonLocalizedName { values: map_values },
            },
            start_time: String::from("20250401T120000.000Z"),
            end_time: String::from("20250401T140000.000Z"),
        }],
    }
}

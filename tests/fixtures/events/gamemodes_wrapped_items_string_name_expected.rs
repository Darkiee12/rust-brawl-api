use std::collections::BTreeMap;

use crate::model::events::{EventType, EventTypeList, JsonLocalizedName};

pub fn expected() -> EventTypeList {
    let mut name_values = BTreeMap::new();
    name_values.insert(String::from("en"), String::from("Gem Grab"));

    EventTypeList {
        items: vec![EventType {
            id: 1,
            name: JsonLocalizedName { values: name_values },
        }],
    }
}

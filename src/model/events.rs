//! Models related to `/gamemodes` and `/events/rotation` endpoints.

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{self, Deserialize, Deserializer, Serialize};

use crate::error::Result;
use crate::http::routes::Route;
use crate::http::Client;
use crate::util::fetch_route;

#[cfg(feature = "async")]
use crate::util::a_fetch_route;

/// Represents a localized name object from the API.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonLocalizedName {
    pub values: BTreeMap<String, String>,
}

fn deserialize_localized_name<'de, D>(deserializer: D) -> ::std::result::Result<JsonLocalizedName, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum LocalizedNameInput {
        Localized(BTreeMap<String, String>),
        Plain(String),
    }

    match Option::<LocalizedNameInput>::deserialize(deserializer)? {
        Some(LocalizedNameInput::Localized(values)) => Ok(JsonLocalizedName { values }),
        Some(LocalizedNameInput::Plain(name)) => {
            let mut values = BTreeMap::new();
            values.insert(String::from("en"), name);
            Ok(JsonLocalizedName { values })
        }
        None => Ok(JsonLocalizedName::default()),
    }
}

/// Represents one game mode item from `/gamemodes`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EventType {
    /// The localized name payload.
    #[serde(default, deserialize_with = "deserialize_localized_name")]
    pub name: JsonLocalizedName,

    /// The game mode id.
    #[serde(default)]
    pub id: usize,
}

/// Represents the array response from `/gamemodes`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct EventTypeList {
    /// List of available game modes.
    pub items: Vec<EventType>,
}

impl<'de> Deserialize<'de> for EventTypeList {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum EventTypeListInput {
            Wrapped { items: Vec<EventType> },
            Bare(Vec<EventType>),
        }

        let input = EventTypeListInput::deserialize(deserializer)?;
        let items = match input {
            EventTypeListInput::Wrapped { items } => items,
            EventTypeListInput::Bare(items) => items,
        };

        Ok(EventTypeList { items })
    }
}

impl Deref for EventTypeList {
    type Target = Vec<EventType>;

    fn deref(&self) -> &Vec<EventType> {
        &self.items
    }
}

impl DerefMut for EventTypeList {
    fn deref_mut(&mut self) -> &mut Vec<EventType> {
        &mut self.items
    }
}

impl EventTypeList {
    /// Returns the route for `/gamemodes` with optional pagination parameters.
    pub fn get_route(limit: Option<u8>, before: Option<&str>, after: Option<&str>) -> Route {
        Route::GameModes {
            limit,
            before: before.map(|s| s.to_owned()),
            after: after.map(|s| s.to_owned()),
        }
    }

    /// (Sync) Fetches game modes from `/gamemodes`.
    pub fn fetch(
        client: &Client,
        limit: Option<u8>,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Result<EventTypeList> {
        let route = EventTypeList::get_route(limit, before, after);
        fetch_route::<EventTypeList>(client, &route)
    }

    /// (Async) Fetches game modes from `/gamemodes`.
    #[cfg(feature = "async")]
    pub async fn a_fetch(
        client: &Client,
        limit: Option<u8>,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Result<EventTypeList> {
        let route = EventTypeList::get_route(limit, before, after);
        a_fetch_route::<EventTypeList>(client, &route).await
    }
}

/// Represents one scheduled event from `/events/rotation`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledEvent {
    /// Slot id of the event rotation entry.
    #[serde(default)]
    pub slot_id: usize,

    /// Event location and mode details.
    #[serde(default)]
    pub event: ScheduledEventLocation,

    /// Start time string.
    #[serde(default)]
    pub start_time: String,

    /// End time string.
    #[serde(default)]
    pub end_time: String,
}

/// Event metadata for a scheduled event entry.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledEventLocation {
    /// Mode id string (for example: `gemGrab`).
    #[serde(default)]
    pub mode: String,

    /// Numeric mode id.
    #[serde(default)]
    pub mode_id: Option<usize>,

    /// Active modifiers for this slot.
    #[serde(default)]
    pub modifiers: Vec<String>,

    /// Event id.
    #[serde(default)]
    pub id: usize,

    /// Localized map name payload.
    #[serde(default, deserialize_with = "deserialize_localized_name")]
    pub map: JsonLocalizedName,
}

/// Represents the array response from `/events/rotation`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct ScheduledEvents {
    /// List of scheduled events.
    pub items: Vec<ScheduledEvent>,
}

impl<'de> Deserialize<'de> for ScheduledEvents {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ScheduledEventsInput {
            Wrapped { items: Vec<ScheduledEvent> },
            Bare(Vec<ScheduledEvent>),
        }

        let input = ScheduledEventsInput::deserialize(deserializer)?;
        let items = match input {
            ScheduledEventsInput::Wrapped { items } => items,
            ScheduledEventsInput::Bare(items) => items,
        };

        Ok(ScheduledEvents { items })
    }
}

impl Deref for ScheduledEvents {
    type Target = Vec<ScheduledEvent>;

    fn deref(&self) -> &Vec<ScheduledEvent> {
        &self.items
    }
}

impl DerefMut for ScheduledEvents {
    fn deref_mut(&mut self) -> &mut Vec<ScheduledEvent> {
        &mut self.items
    }
}

impl ScheduledEvents {
    /// Returns the route for `/events/rotation`.
    pub fn get_route() -> Route {
        Route::EventRotation
    }

    /// (Sync) Fetches scheduled events from `/events/rotation`.
    pub fn fetch(client: &Client) -> Result<ScheduledEvents> {
        let route = ScheduledEvents::get_route();
        fetch_route::<ScheduledEvents>(client, &route)
    }

    /// (Async) Fetches scheduled events from `/events/rotation`.
    #[cfg(feature = "async")]
    pub async fn a_fetch(client: &Client) -> Result<ScheduledEvents> {
        let route = ScheduledEvents::get_route();
        a_fetch_route::<ScheduledEvents>(client, &route).await
    }
}

#[cfg(test)]
mod tests {
    use super::{EventTypeList, ScheduledEvents};
    use crate::http::routes::Route;
    use std::fs::read_to_string;

    mod gamemodes_array_expected {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/events/gamemodes_array_expected.rs"
        ));
    }
    mod events_rotation_array_expected {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/events/events_rotation_array_expected.rs"
        ));
    }
    mod gamemodes_wrapped_items_string_name_expected {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/events/gamemodes_wrapped_items_string_name_expected.rs"
        ));
    }
    mod events_rotation_wrapped_items_string_map_expected {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/events/events_rotation_wrapped_items_string_map_expected.rs"
        ));
    }

    #[test]
    fn gamemodes_deser_array() -> Result<(), Box<dyn ::std::error::Error>> {
        let json = read_to_string("tests/fixtures/events/gamemodes_array.json")?;
        let list = serde_json::from_str::<EventTypeList>(&json)?;

        assert_eq!(list, gamemodes_array_expected::expected());

        Ok(())
    }

    #[test]
    fn scheduled_events_deser_array() -> Result<(), Box<dyn ::std::error::Error>> {
        let json = read_to_string("tests/fixtures/events/events_rotation_array.json")?;
        let events = serde_json::from_str::<ScheduledEvents>(&json)?;

        assert_eq!(events, events_rotation_array_expected::expected());

        Ok(())
    }

    #[test]
    fn gamemodes_deser_wrapped_string_name() -> Result<(), Box<dyn ::std::error::Error>> {
        let json = read_to_string("tests/fixtures/events/gamemodes_wrapped_items_string_name.json")?;
        let list = serde_json::from_str::<EventTypeList>(&json)?;

        assert_eq!(list, gamemodes_wrapped_items_string_name_expected::expected());

        Ok(())
    }

    #[test]
    fn scheduled_events_deser_wrapped_string_map() -> Result<(), Box<dyn ::std::error::Error>> {
        let json = read_to_string("tests/fixtures/events/events_rotation_wrapped_items_string_map.json")?;
        let events = serde_json::from_str::<ScheduledEvents>(&json)?;

        assert_eq!(events, events_rotation_wrapped_items_string_map_expected::expected());

        Ok(())
    }

    #[test]
    fn gamemodes_route_with_pagination() {
        let route = Route::GameModes {
            limit: Some(10),
            before: Some(String::from("abc")),
            after: Some(String::from("def")),
        };

        assert_eq!(
            route.to_url_str(),
            "https://api.brawlstars.com/v1/gamemodes?limit=10&before=abc&after=def"
        );
    }
}

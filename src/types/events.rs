use serde_derive::Deserialize;
use crate::types::{HassEntity, Context};
//use serde_json::Value;

//This constructed against StateChangedEvent, may not be compatible with other event types
//TODO try out other type of events
#[derive(Debug, Deserialize, PartialEq)]
pub struct HassEvent {
    pub event_type: String,
    pub data: EventData,
    pub origin: String,
    pub time_fired: String,
    pub context: Context,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct EventData {
    entity_id: String,
    new_state: Option<HassEntity>,
    old_state: Option<HassEntity>,
}

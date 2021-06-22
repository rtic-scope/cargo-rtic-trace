//! API used between RTIC Scope front- and backends.

use chrono::prelude::Local;
use itm_decode::ExceptionAction;
#[allow(unused_imports)]
use itm_decode::TracePacket;
use serde::{Deserialize, Serialize};

type Timestamp = chrono::DateTime<Local>;

/// A set of events that occurred at a certain timepoint after target
/// reset.
#[derive(Serialize, Deserialize)]
pub struct EventChunk {
    /// Collective timestamp for the chunk of [EventChunk::events].
    pub timestamp: Timestamp,

    pub events: Vec<EventType>,
}

/// Verbatim copy of [ExceptionAction], sans the enum name.
#[derive(Serialize, Deserialize)]
pub enum TaskAction {
    /// Task was entered.
    Entered,

    /// Task was exited.
    Exited,

    /// Task was returned to.
    Returned,
}

/// Derivative subset of [TracePacket], where RTIC task information has
/// been resolved.
#[derive(Serialize, Deserialize)]
pub enum EventType {
    /// [TracePacket::Overflow] equivalent.
    Overflow,

    /// An RTIC task performed an action.
    Task {
        /// What RTIC task did something?
        name: String,

        /// What did the RTIC task do?
        action: TaskAction,
    },
}

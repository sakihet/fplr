use crate::models::Event;

/// Find the current gameweek event
pub fn find_current_event(events: &[Event]) -> Option<&Event> {
    events.iter().find(|e| e.is_current)
}

/// Find the next gameweek event
pub fn find_next_event(events: &[Event]) -> Option<&Event> {
    events.iter().find(|e| e.is_next)
}

/// Get the effective event ID based on user input or current/next gameweek.
///
/// Returns the specified event ID if provided, otherwise:
/// - Returns next event ID if the current event is finished
/// - Returns current event ID if available and not yet finished
/// - Falls back to next event ID - 1 if no current event (between gameweeks)
/// - Returns None if the season hasn't started yet (next event is GW1)
/// - Falls back to the last event ID if the season is over (no next event scheduled)
pub fn get_effective_event_id(events: &[Event], specified: Option<u32>) -> Option<u32> {
    if let Some(id) = specified {
        return Some(id);
    }

    // If current event is finished, show the next gameweek instead
    if let Some(current) = find_current_event(events) {
        if current.finished
            && let Some(next) = find_next_event(events)
        {
            return Some(next.id as u32);
        }
        return Some(current.id as u32);
    }

    // Between gameweeks: no current event, but there's a previous gameweek to fall back to.
    // If the next event is GW1, the season hasn't started and no gameweek data exists yet —
    // this must be distinguished from "no next event" (season over), which falls through below.
    if let Some(next) = find_next_event(events) {
        return if next.id > 1 {
            Some((next.id - 1) as u32)
        } else {
            None
        };
    }

    // Fall back to last event (season ended, no next event scheduled)
    events.last().map(|e| e.id as u32)
}

/// Get the current gameweek ID, or None if season hasn't started
pub fn get_current_event_id(events: &[Event]) -> Option<u32> {
    find_current_event(events).map(|e| e.id as u32)
}

use crate::models::Event;

/// Find the current gameweek event
pub fn find_current_event(events: &[Event]) -> Option<&Event> {
    events.iter().find(|e| e.is_current)
}

/// Find the next gameweek event
pub fn find_next_event(events: &[Event]) -> Option<&Event> {
    events.iter().find(|e| e.is_next)
}

/// Get the effective event ID based on user input or current/next gameweek
///
/// Returns the specified event ID if provided, otherwise:
/// - Returns current event ID if available
/// - Falls back to next event ID - 1 if no current event
/// - Falls back to the last event ID if season is over
pub fn get_effective_event_id(events: &[Event], specified: Option<u32>) -> Option<u32> {
    if let Some(id) = specified {
        return Some(id);
    }

    // Try current event first
    if let Some(current) = find_current_event(events) {
        return Some(current.id as u32);
    }

    // Try next event - 1 (between gameweeks)
    if let Some(next) = find_next_event(events)
        && next.id > 1
    {
        return Some((next.id - 1) as u32);
    }

    // Fall back to last event (season ended)
    events.last().map(|e| e.id as u32)
}

/// Get the current gameweek ID, or None if season hasn't started
pub fn get_current_event_id(events: &[Event]) -> Option<u32> {
    find_current_event(events).map(|e| e.id as u32)
}

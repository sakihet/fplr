use crate::models::Event;

/// Find the current gameweek event
pub fn find_current_event(events: &[Event]) -> Option<&Event> {
    events.iter().find(|e| e.is_current)
}

/// Find the next gameweek event
pub fn find_next_event(events: &[Event]) -> Option<&Event> {
    events.iter().find(|e| e.is_next)
}

/// Find the previous gameweek event relative to the current one
pub fn find_prev_event(events: &[Event]) -> Option<&Event> {
    if let Some(current) = find_current_event(events) {
        if current.id > 1 {
            return events.iter().find(|e| e.id == current.id - 1);
        }
    } else if let Some(next) = find_next_event(events) {
        // Between gameweeks: "prev" is two before next (i.e. next - 2)
        if next.id > 2 {
            return events.iter().find(|e| e.id == next.id - 2);
        } else if next.id == 2 {
            return events.iter().find(|e| e.id == 1);
        }
    }
    None
}

/// Get the effective event ID based on user input or current/next gameweek.
///
/// Returns the specified event ID if provided, otherwise:
/// - Returns next event ID if the current event is finished
/// - Returns current event ID if available and not yet finished
/// - Falls back to next event ID - 1 if no current event
/// - Falls back to the last event ID if season is over
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

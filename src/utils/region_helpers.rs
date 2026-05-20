use crate::commands::region::REGIONS;

/// Find a region ID by a query string (ID, Name, ISO2, or ISO3)
pub fn find_region_id(query: &str) -> Option<u64> {
    let query_lower = query.to_lowercase();

    // 1. Try to parse as ID
    if let Ok(id) = query.parse::<u64>()
        && REGIONS.iter().any(|(rid, _, _, _)| *rid as u64 == id)
    {
        return Some(id);
    }

    // 2. Search by Name, ISO2, or ISO3
    REGIONS.iter().find_map(|(id, name, iso2, iso3)| {
        if name.to_lowercase().contains(&query_lower)
            || iso2.to_lowercase() == query_lower
            || iso3.to_lowercase() == query_lower
        {
            Some(*id as u64)
        } else {
            None
        }
    })
}

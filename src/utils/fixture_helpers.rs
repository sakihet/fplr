use crate::models::Fixture;

/// Progress of a set of fixtures within a gameweek
pub struct GameweekProgress {
    pub total: usize,
    pub settled: usize,
    pub in_play: usize,
    pub awaiting_bonus: usize,
}

/// A fixture's score is final once it is settled.
/// `finished` only flips after FPL confirms bonus points, so `finished_provisional`
/// already marks a settled match.
pub fn is_settled(fixture: &Fixture) -> bool {
    fixture.finished || fixture.finished_provisional
}

/// A fixture is in play once it has kicked off but its score is not settled yet.
pub fn is_in_play(fixture: &Fixture) -> bool {
    fixture.started == Some(true) && !is_settled(fixture)
}

/// A settled fixture still waiting for FPL to confirm bonus points
pub fn is_awaiting_bonus(fixture: &Fixture) -> bool {
    fixture.finished_provisional && !fixture.finished
}

/// Count fixtures by state in a single pass
pub fn gameweek_progress<'a>(fixtures: impl IntoIterator<Item = &'a Fixture>) -> GameweekProgress {
    let mut progress = GameweekProgress {
        total: 0,
        settled: 0,
        in_play: 0,
        awaiting_bonus: 0,
    };

    for fixture in fixtures {
        progress.total += 1;
        if is_settled(fixture) {
            progress.settled += 1;
        }
        if is_in_play(fixture) {
            progress.in_play += 1;
        }
        if is_awaiting_bonus(fixture) {
            progress.awaiting_bonus += 1;
        }
    }

    progress
}

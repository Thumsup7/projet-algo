use crate::polygon::{Point, segments_intersect};

/// A chord between vertex i and vertex j with precomputed length
#[derive(Debug, Clone, Copy)]
pub struct Chord {
    pub i: usize,
    pub j: usize,
    pub length: f64,
}

/// Build the sorted list of all possible chords for an n-vertex polygon
/// (non-adjacent pairs only), sorted by length ascending.
pub fn all_chords(vertices: &[Point]) -> Vec<Chord> {
    let n = vertices.len();
    let mut chords = Vec::new();
    for i in 0..n {
        for j in (i + 2)..n {
            // Exclude the edge (0, n-1) which is also adjacent
            if i == 0 && j == n - 1 {
                continue;
            }
            let dx = vertices[i].x - vertices[j].x;
            let dy = vertices[i].y - vertices[j].y;
            let length = (dx * dx + dy * dy).sqrt();
            chords.push(Chord { i, j, length });
        }
    }
    chords.sort_by(|a, b| a.length.partial_cmp(&b.length).unwrap());
    chords
}

/// B.1 — validecorde(i, j): returns true if the chord (i,j) has not already been
/// drawn and does not intersect any already-drawn chord.
/// `drawn` is the list of chords already selected.
pub fn valid_chord(
    ci: usize,
    cj: usize,
    drawn: &[Chord],
    vertices: &[Point],
) -> bool {
    let n = vertices.len();
    // Check not already drawn
    for c in drawn {
        if (c.i == ci && c.j == cj) || (c.i == cj && c.j == ci) {
            return false;
        }
    }
    // Check no intersection with already drawn chords
    // Two chords (a,b) and (c,d) intersect if they share no endpoint and cross
    let pi = &vertices[ci];
    let pj = &vertices[cj];
    for c in drawn {
        let ca = c.i;
        let cb = c.j;
        // Skip if they share an endpoint (adjacent chords don't cross)
        if ca == ci || ca == cj || cb == ci || cb == cj {
            continue;
        }
        if segments_intersect(pi, pj, &vertices[ca], &vertices[cb]) {
            return false;
        }
    }
    // For a STRICTLY CONVEX polygon, any chord between two non-adjacent vertices
    // lies entirely inside the polygon and cannot cross any polygon edge.
    // This check is therefore redundant for the convex case guaranteed by the problem.
    // It is kept to make the function correct for non-convex inputs as well.
    for e in 0..n {
        let next = (e + 1) % n;
        // Skip edges that share an endpoint with our chord
        if e == ci || e == cj || next == ci || next == cj {
            continue;
        }
        if segments_intersect(pi, pj, &vertices[e], &vertices[next]) {
            return false;
        }
    }
    true
}

/// State shared across the recursive search
pub struct BacktrackState<'a> {
    pub vertices: &'a [Point],
    pub chords: &'a [Chord],
    /// Number of chords needed for a full triangulation (= n - 3)
    pub needed: usize,
    /// Current set of selected chord indices (into `chords`)
    pub selected: Vec<usize>,
    pub best_cost: f64,
    pub best_solution: Vec<usize>,
    /// Number of recursive calls (for complexity analysis)
    pub call_count: u64,
}

impl<'a> BacktrackState<'a> {
    pub fn new(vertices: &'a [Point], chords: &'a [Chord]) -> Self {
        let needed = vertices.len() - 3;
        Self {
            vertices,
            chords,
            needed,
            selected: Vec::new(),
            best_cost: f64::INFINITY,
            best_solution: Vec::new(),
            call_count: 0,
        }
    }

    /// Current total length of selected chords
    fn current_cost(&self) -> f64 {
        self.selected.iter().map(|&idx| self.chords[idx].length).sum()
    }
}

/// B.3a — Main backtracking entry point.
/// Explores all subsets of chords, each considered at most once (iterate in order).
/// `start` is the index in `chords` from which to consider next chords.
pub fn backtrack(state: &mut BacktrackState, start: usize) {
    state.call_count += 1;

    // === Pruning B.3c ===

    // 1. Already have enough chords: for a convex polygon, exactly n-3 mutually
    //    non-crossing chords always form a complete triangulation (Euler's formula).
    if state.selected.len() == state.needed {
        let cost = state.current_cost();
        if cost < state.best_cost {
            state.best_cost = cost;
            state.best_solution = state.selected.clone();
        }
        return;
    }

    let remaining_chords = state.chords.len() - start;
    let still_needed = state.needed - state.selected.len();

    // 2. Not enough chords left to complete a triangulation
    if remaining_chords < still_needed {
        return;
    }

    // 3. Current cost already exceeds best known (branch-and-bound)
    let current = state.current_cost();
    if current >= state.best_cost {
        return;
    }

    // Build drawn_chords once before the loop (not inside it).
    // selected does not change within this loop, so rebuilding per iteration is wasteful.
    let drawn_chords: Vec<Chord> = state
        .selected
        .iter()
        .map(|&i| state.chords[i])
        .collect();

    // Explore: try including each remaining chord in turn, or implicitly skip it.
    for idx in start..state.chords.len() {
        let chord = state.chords[idx];

        // Stronger branch-and-bound: since chords are sorted by length ascending,
        // the cheapest possible completion uses the next `still_needed` chords.
        // If even that lower bound exceeds the best cost, we can stop the loop entirely.
        let lower_bound = state.current_cost()
            + (0..still_needed)
                .map(|k| state.chords.get(idx + k).map_or(f64::INFINITY, |c| c.length))
                .sum::<f64>();
        if lower_bound >= state.best_cost {
            break; // all remaining chords are at least as long, so no improvement possible
        }

        if valid_chord(chord.i, chord.j, &drawn_chords, state.vertices) {
            state.selected.push(idx);
            backtrack(state, idx + 1);
            state.selected.pop();
        }
    }
}

/// Public API: find the minimum triangulation via backtracking.
/// Also returns the recursive call count for complexity analysis.
pub fn min_triangulation_backtrack_verbose(vertices: &[Point]) -> (f64, Vec<Chord>, u64) {
    let chords = all_chords(vertices);
    let mut state = BacktrackState::new(vertices, &chords);
    backtrack(&mut state, 0);
    let solution = state
        .best_solution
        .iter()
        .map(|&i| chords[i])
        .collect();
    (state.best_cost, solution, state.call_count)
}

use crate::polygon::Point;

/// D — Greedy triangulation.
///
/// An "exterior chord" of a polygon is a chord that forms a triangle
/// with two consecutive edges of the remaining polygon.
/// Equivalently, for a polygon with vertices [v_0, v_1, ..., v_m-1],
/// the exterior chords are the diagonals (v_{i-1}, v_{i+1}) for each
/// non-adjacent vertex v_i (i.e., v_i is the "ear tip").
///
/// Strategy: repeatedly pick the shortest exterior chord,
/// remove the corresponding ear, repeat until fully triangulated.

#[derive(Debug, Clone)]
pub struct GreedyResult {
    pub cost: f64,
    /// Each chord as (original_index_a, original_index_b, length)
    pub chords: Vec<(usize, usize, f64)>,
}

pub fn min_triangulation_greedy(vertices: &[Point]) -> GreedyResult {
    let n = vertices.len();
    // Work with indices into the original vertices array
    let mut ring: Vec<usize> = (0..n).collect();
    let mut chords = Vec::new();
    let mut total_cost = 0.0;

    // We need to remove n-2 triangles (ears), each adding one chord
    // (for n>=4; for n=3 no chord needed)
    while ring.len() > 3 {
        // Find all exterior chords and their lengths
        // For each consecutive triple (ring[i-1], ring[i], ring[i+1]),
        // the exterior chord is (ring[i-1], ring[i+1]).
        let m = ring.len();
        let mut best_len = f64::INFINITY;
        let mut best_ear = 0usize; // index i in ring of the ear tip

        for i in 0..m {
            let prev = ring[(i + m - 1) % m];
            let next = ring[(i + 1) % m];
            let tip = ring[i];
            let _ = tip; // ear tip (not the chord endpoint)

            let dx = vertices[prev].x - vertices[next].x;
            let dy = vertices[prev].y - vertices[next].y;
            let len = (dx * dx + dy * dy).sqrt();

            if len < best_len {
                best_len = len;
                best_ear = i;
            }
        }

        // Add the best exterior chord (if m > 3, i.e., it's a real chord, not an edge)
        let prev = ring[(best_ear + m - 1) % m];
        let next = ring[(best_ear + 1) % m];
        total_cost += best_len;
        chords.push((prev, next, best_len));

        // Remove the ear tip from the ring
        ring.remove(best_ear);
    }

    // The last triangle needs no chord (its three sides are already edges/committed chords)

    GreedyResult {
        cost: total_cost,
        chords,
    }
}

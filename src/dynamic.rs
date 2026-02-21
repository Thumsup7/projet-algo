use crate::polygon::Point;

/// C — Dynamic programming triangulation.
///
/// T[i][t] = minimum CHORD cost of triangulating the sub-polygon
///   s_i, s_{i+1}, ..., s_{i+t-1}  (indices mod n)
///
/// Recurrence (t >= 3):
///   T[i][t] = min over k in 1..=t-2 of:
///       c(i, i+k, k) + c(i+k, i+t-1, k,t) + T[i][k+1] + T[i+k][t-k]
///
///   where c(a,b) = d(s_a, s_b) if the segment is a CHORD (not a polygon edge), else 0.
///   Specifically:
///     - (s_i, s_{i+k})    is a chord iff k >= 2  (k=1 means the edge s_i->s_{i+1})
///     - (s_{i+k}, s_{i+t-1}) is a chord iff k <= t-3 (k=t-2 means edge s_{i+t-2}->s_{i+t-1})
///   The base segment (s_i, s_{i+t-1}) is excluded: it was committed by the parent call.
///
/// Base cases: T[i][1] = T[i][2] = 0  (no triangle possible)

pub struct DpResult {
    pub cost: f64,
    /// Which vertex k was chosen at each (i, t) — for reconstructing the triangulation
    pub choice: Vec<Vec<usize>>,
    pub n: usize,
}

pub fn min_triangulation_dp(vertices: &[Point]) -> DpResult {
    let n = vertices.len();

    // Precompute distances
    let dist: Vec<Vec<f64>> = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    let dx = vertices[i].x - vertices[j].x;
                    let dy = vertices[i].y - vertices[j].y;
                    (dx * dx + dy * dy).sqrt()
                })
                .collect()
        })
        .collect();

    // dp[i][t] = min cost of T_{i,t}
    // We use 1-indexed t: t in 1..=n
    let mut dp = vec![vec![f64::INFINITY; n + 1]; n];
    let mut choice = vec![vec![0usize; n + 1]; n];

    // Base cases
    for i in 0..n {
        dp[i][1] = 0.0;
        if n >= 2 {
            dp[i][2] = 0.0;
        }
    }

    // Fill by increasing sub-problem size t
    for t in 3..=n {
        for i in 0..n {
            let mut best = f64::INFINITY;
            let mut best_k = 1;
            // k ranges from 1 to t-2
            // The triangle formed is (s_i, s_{i+k}, s_{i+t-1})
            // Sub-problems: T(i, k+1) and T(i+k, t-k)
            for k in 1..=(t - 2) {
                let ik = (i + k) % n;
                let it1 = (i + t - 1) % n;

                // Only count sides that are true chords (not polygon edges).
                // (s_i, s_{i+k}) is a polygon edge iff k == 1.
                // (s_{i+k}, s_{i+t-1}) is a polygon edge iff k == t-2.
                let chord_ik  = if k >= 2       { dist[i][ik]  } else { 0.0 };
                let chord_ikt1 = if k <= t - 3  { dist[ik][it1] } else { 0.0 };

                let cost = chord_ik + chord_ikt1 + dp[i][k + 1] + dp[ik][t - k];

                if cost < best {
                    best = cost;
                    best_k = k;
                }
            }
            dp[i][t] = best;
            choice[i][t] = best_k;
        }
    }

    DpResult {
        cost: dp[0][n],
        choice,
        n,
    }
}

/// Reconstruct the list of chords from the DP choice table.
/// Returns pairs (i, j) of vertex indices that are chords (not polygon edges).
pub fn reconstruct_chords(result: &DpResult) -> Vec<(usize, usize)> {
    let n = result.n;
    let mut chords = Vec::new();
    reconstruct(0, n, result, &mut chords);
    chords
}

fn reconstruct(i: usize, t: usize, result: &DpResult, chords: &mut Vec<(usize, usize)>) {
    if t < 3 {
        return;
    }
    let n = result.n;
    let k = result.choice[i][t];
    let ik = (i + k) % n;
    let it1 = (i + t - 1) % n;

    // The three sides of the chosen triangle are:
    //   (i, ik)  — chord if k != 1 (otherwise it's an edge s_i -> s_{i+1})
    //            Wait: k=1 means ik = i+1, which is an edge. k>=2 means it's a chord.
    //   (ik, it1) — chord if k != t-2 (otherwise it's an edge s_{i+t-2} -> s_{i+t-1})
    //   (i, it1)  — this is the "base" of the sub-problem, either an edge (t=n at top)
    //               or an already-committed chord from the parent call.

    if k >= 2 {
        chords.push((i, ik));
    }
    if k <= t - 3 {
        chords.push((ik, it1));
    }

    reconstruct(i, k + 1, result, chords);
    reconstruct(ik, t - k, result, chords);
}

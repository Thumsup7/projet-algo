mod backtracking;
mod dynamic;
mod greedy;
mod polygon;
mod tests;

use polygon::{heptagon, regular_polygon, Point};
use std::time::Instant;

fn print_section(title: &str) {
    println!("\n{}", "=".repeat(60));
    println!("  {}", title);
    println!("{}", "=".repeat(60));
}

fn run_all(vertices: &[Point], label: &str) {
    let n = vertices.len();
    println!("\n--- {} ({} vertices) ---", label, n);

    // Dynamic programming (always run)
    let t0 = Instant::now();
    let dp = dynamic::min_triangulation_dp(vertices);
    let elapsed_dp = t0.elapsed();
    let dp_chords = dynamic::reconstruct_chords(&dp);
    println!(
        "[DP]      cost = {:.4}  chords = {:?}  time = {:?}",
        dp.cost, dp_chords, elapsed_dp
    );

    // Greedy
    let t1 = Instant::now();
    let gr = greedy::min_triangulation_greedy(vertices);
    let elapsed_gr = t1.elapsed();
    println!(
        "[Greedy]  cost = {:.4}  chords = {:?}  time = {:?}",
        gr.cost,
        gr.chords
            .iter()
            .map(|&(a, b, _)| (a, b))
            .collect::<Vec<_>>(),
        elapsed_gr
    );

    // Backtracking (only for small n)
    if n <= 14 {
        let t2 = Instant::now();
        let (bt_cost, bt_chords, calls) =
            backtracking::min_triangulation_backtrack_verbose(vertices);
        let elapsed_bt = t2.elapsed();
        println!(
            "[BT]      cost = {:.4}  chords = {:?}  calls = {}  time = {:?}",
            bt_cost,
            bt_chords.iter().map(|c| (c.i, c.j)).collect::<Vec<_>>(),
            calls,
            elapsed_bt
        );
    } else {
        println!("[BT]      skipped (n > 14)");
    }
}

fn main() {
    // ----------------------------------------------------------------
    // Section A: count chords and number of chords in a triangulation
    // ----------------------------------------------------------------
    print_section("A. Preliminary questions");
    println!("Chords in n-gon = n*(n-3)/2");
    for n in [4usize, 5, 6, 7, 10] {
        let total_chords = n * (n - 3) / 2;
        let triangulation_chords = n - 3;
        println!(
            "  n={:2}: possible chords = {:3}, chords per triangulation = {}",
            n, total_chords, triangulation_chords
        );
    }

    // ----------------------------------------------------------------
    // Section B+C+D: run all algorithms on the heptagon from the PDF
    // ----------------------------------------------------------------
    print_section("B/C/D — Heptagon from the instructions");
    let hepta = heptagon();
    run_all(&hepta, "Heptagon (PDF example)");

    // ----------------------------------------------------------------
    // Verify the non-minimal triangulation given in the PDF
    // ----------------------------------------------------------------
    println!("\n[Check] Non-minimal triangulation from PDF (chords s0-s2, s0-s3, s0-s5, s3-s5):");
    let verts = heptagon();
    let pairs = [(0usize, 2usize), (0, 3), (0, 5), (3, 5)];
    let sum: f64 = pairs
        .iter()
        .map(|&(a, b)| {
            let dx = verts[a].x - verts[b].x;
            let dy = verts[a].y - verts[b].y;
            (dx * dx + dy * dy).sqrt()
        })
        .sum();
    println!("  weight = {:.4} (expected ~77.56)", sum);

    // ----------------------------------------------------------------
    // Section B.3d — Backtracking scaling: measure call counts vs n
    // ----------------------------------------------------------------
    print_section("B.3d — Backtracking scaling");
    println!("{:>4} {:>12} {:>12} {:>12}", "n", "calls", "bt_cost", "time_ms");
    for n in 4..=20usize {
        let verts = regular_polygon(n);
        let t = Instant::now();
        let (cost, _, calls) = backtracking::min_triangulation_backtrack_verbose(&verts);
        let elapsed = t.elapsed().as_millis();
        println!("{:>4} {:>12} {:>12.4} {:>12}", n, calls, cost, elapsed);
        if elapsed > 120_000 {
            println!("  (stopping: exceeded 2 minutes)");
            break;
        }
    }

    // ----------------------------------------------------------------
    // Larger polygons: DP and greedy only
    // ----------------------------------------------------------------
    print_section("C/D — Larger polygons (DP + Greedy)");
    for n in [10usize, 20, 50, 100] {
        let verts = regular_polygon(n);
        let dp = dynamic::min_triangulation_dp(&verts);
        let gr = greedy::min_triangulation_greedy(&verts);
        println!(
            "n={:4}: DP cost = {:10.4}  Greedy cost = {:10.4}  ratio = {:.4}",
            n,
            dp.cost,
            gr.cost,
            gr.cost / dp.cost
        );
    }
}

/// Integration and unit tests for all three triangulation algorithms.
///
/// Test strategy:
///   - Unit tests for polygon utilities (distance, intersection, chord enumeration)
///   - Cross-validation: DP and backtracking must agree on small polygons
///   - Greedy lower bound: greedy cost must be >= DP cost (DP is optimal)
///   - Known values: the heptagon from the PDF has documented costs
///   - Edge cases: n=3 (triangle, 0 chords), n=4 (quadrilateral, 1 chord)
///   - Reconstruction: chords returned by DP must be valid (non-crossing, correct count)

#[cfg(test)]
mod tests {
    use crate::backtracking::{all_chords, min_triangulation_backtrack_verbose, valid_chord};
    use crate::dynamic::{min_triangulation_dp, reconstruct_chords};
    use crate::greedy::min_triangulation_greedy;
    use crate::polygon::{heptagon, regular_polygon, segments_intersect, Point};

    const EPS: f64 = 1e-6;

    // -----------------------------------------------------------------------
    // A.1 — Chord count formula: n*(n-3)/2
    // -----------------------------------------------------------------------

    #[test]
    fn test_chord_count_formula() {
        for n in 4..=12usize {
            let verts = regular_polygon(n);
            let chords = all_chords(&verts);
            let expected = n * (n - 3) / 2;
            assert_eq!(
                chords.len(),
                expected,
                "n={}: expected {} chords, got {}",
                n,
                expected,
                chords.len()
            );
        }
    }

    // -----------------------------------------------------------------------
    // A.2 — Triangulation chord count: n-3 chords
    // -----------------------------------------------------------------------

    #[test]
    fn test_triangulation_has_n_minus_3_chords_dp() {
        for n in 4..=10usize {
            let verts = regular_polygon(n);
            let result = min_triangulation_dp(&verts);
            let chords = reconstruct_chords(&result);
            assert_eq!(
                chords.len(),
                n - 3,
                "n={}: DP returned {} chords, expected {}",
                n,
                chords.len(),
                n - 3
            );
        }
    }

    #[test]
    fn test_triangulation_has_n_minus_3_chords_bt() {
        for n in 4..=9usize {
            let verts = regular_polygon(n);
            let (_, chords, _) = min_triangulation_backtrack_verbose(&verts);
            assert_eq!(
                chords.len(),
                n - 3,
                "n={}: BT returned {} chords, expected {}",
                n,
                chords.len(),
                n - 3
            );
        }
    }

    #[test]
    fn test_triangulation_has_n_minus_3_chords_greedy() {
        for n in 4..=10usize {
            let verts = regular_polygon(n);
            let result = min_triangulation_greedy(&verts);
            assert_eq!(
                result.chords.len(),
                n - 3,
                "n={}: greedy returned {} chords, expected {}",
                n,
                result.chords.len(),
                n - 3
            );
        }
    }

    // -----------------------------------------------------------------------
    // Edge case: triangle (n=3) — no chords needed, cost = 0
    // -----------------------------------------------------------------------

    #[test]
    fn test_triangle_no_chords_dp() {
        let verts = regular_polygon(3);
        let result = min_triangulation_dp(&verts);
        assert_eq!(result.cost, 0.0, "triangle DP cost should be 0");
        let chords = reconstruct_chords(&result);
        assert_eq!(chords.len(), 0, "triangle should have 0 chords");
    }

    #[test]
    fn test_triangle_no_chords_greedy() {
        let verts = regular_polygon(3);
        let result = min_triangulation_greedy(&verts);
        assert_eq!(result.cost, 0.0, "triangle greedy cost should be 0");
        assert_eq!(result.chords.len(), 0, "triangle should have 0 chords");
    }

    // -----------------------------------------------------------------------
    // Edge case: quadrilateral (n=4) — exactly 1 chord
    // -----------------------------------------------------------------------

    #[test]
    fn test_quadrilateral_one_chord_dp() {
        let verts = regular_polygon(4);
        let result = min_triangulation_dp(&verts);
        let chords = reconstruct_chords(&result);
        assert_eq!(chords.len(), 1, "quadrilateral should have exactly 1 chord");
        assert!(result.cost > 0.0, "quadrilateral chord cost should be positive");
    }

    #[test]
    fn test_quadrilateral_one_chord_bt() {
        let verts = regular_polygon(4);
        let (cost, chords, _) = min_triangulation_backtrack_verbose(&verts);
        assert_eq!(chords.len(), 1, "quadrilateral should have exactly 1 chord");
        assert!(cost > 0.0, "quadrilateral chord cost should be positive");
    }

    // -----------------------------------------------------------------------
    // Cross-validation: DP and backtracking must agree
    // -----------------------------------------------------------------------

    #[test]
    fn test_dp_equals_backtrack_regular_polygons() {
        for n in 4..=10usize {
            let verts = regular_polygon(n);
            let dp_cost = min_triangulation_dp(&verts).cost;
            let (bt_cost, _, _) = min_triangulation_backtrack_verbose(&verts);
            assert!(
                (dp_cost - bt_cost).abs() < EPS,
                "n={}: DP={:.6} != BT={:.6}",
                n,
                dp_cost,
                bt_cost
            );
        }
    }

    #[test]
    fn test_dp_equals_backtrack_heptagon() {
        let verts = heptagon();
        let dp_cost = min_triangulation_dp(&verts).cost;
        let (bt_cost, _, _) = min_triangulation_backtrack_verbose(&verts);
        assert!(
            (dp_cost - bt_cost).abs() < EPS,
            "Heptagon: DP={:.6} != BT={:.6}",
            dp_cost,
            bt_cost
        );
    }

    // -----------------------------------------------------------------------
    // Optimality: greedy cost must be >= DP cost (DP is exact)
    // -----------------------------------------------------------------------

    #[test]
    fn test_greedy_ge_dp_regular_polygons() {
        for n in 4..=20usize {
            let verts = regular_polygon(n);
            let dp_cost = min_triangulation_dp(&verts).cost;
            let gr_cost = min_triangulation_greedy(&verts).cost;
            assert!(
                gr_cost >= dp_cost - EPS,
                "n={}: greedy({:.6}) < dp({:.6}), greedy cannot be better than optimal",
                n,
                gr_cost,
                dp_cost
            );
        }
    }

    // -----------------------------------------------------------------------
    // Known value: heptagon from the PDF
    // -----------------------------------------------------------------------

    #[test]
    fn test_heptagon_pdf_non_minimal_cost() {
        // The PDF gives a non-minimal triangulation with cost ~77.56
        let verts = heptagon();
        let pairs = [(0usize, 2usize), (0, 3), (0, 5), (3, 5)];
        let cost: f64 = pairs.iter().map(|&(a, b)| {
            let dx = verts[a].x - verts[b].x;
            let dy = verts[a].y - verts[b].y;
            (dx * dx + dy * dy).sqrt()
        }).sum();
        assert!(
            (cost - 77.5635).abs() < 1e-3,
            "PDF non-minimal cost should be ~77.56, got {:.4}",
            cost
        );
    }

    #[test]
    fn test_heptagon_optimal_cost() {
        // Both DP and BT must find the optimal cost ~75.4307
        let verts = heptagon();
        let dp_cost = min_triangulation_dp(&verts).cost;
        let (bt_cost, _, _) = min_triangulation_backtrack_verbose(&verts);
        assert!(
            (dp_cost - 75.4307).abs() < 1e-3,
            "Heptagon DP optimal cost should be ~75.43, got {:.4}",
            dp_cost
        );
        assert!(
            (bt_cost - 75.4307).abs() < 1e-3,
            "Heptagon BT optimal cost should be ~75.43, got {:.4}",
            bt_cost
        );
    }

    #[test]
    fn test_heptagon_optimal_better_than_pdf() {
        let verts = heptagon();
        let dp_cost = min_triangulation_dp(&verts).cost;
        assert!(
            dp_cost < 77.5635,
            "Optimal ({:.4}) must be strictly less than the PDF example (77.56)",
            dp_cost
        );
    }

    // -----------------------------------------------------------------------
    // Greedy is not exact: heptagon counter-example (D.2)
    // -----------------------------------------------------------------------

    #[test]
    fn test_greedy_not_optimal_heptagon() {
        let verts = heptagon();
        let dp_cost = min_triangulation_dp(&verts).cost;
        let gr_cost = min_triangulation_greedy(&verts).cost;
        // The greedy gives ~75.83, optimal is ~75.43
        assert!(
            gr_cost > dp_cost + EPS,
            "Heptagon: greedy should be strictly worse than DP (gr={:.4} <= dp={:.4})",
            gr_cost,
            dp_cost
        );
    }

    // -----------------------------------------------------------------------
    // segments_intersect — unit tests
    // -----------------------------------------------------------------------

    fn pt(x: f64, y: f64) -> Point {
        Point::new(x, y)
    }

    #[test]
    fn test_segments_intersect_crossing() {
        // Two diagonals of a square cross
        let p1 = pt(0.0, 0.0);
        let p2 = pt(1.0, 1.0);
        let p3 = pt(1.0, 0.0);
        let p4 = pt(0.0, 1.0);
        assert!(
            segments_intersect(&p1, &p2, &p3, &p4),
            "Diagonals of a square should intersect"
        );
    }

    #[test]
    fn test_segments_intersect_not_crossing() {
        // Two parallel horizontal segments
        let p1 = pt(0.0, 0.0);
        let p2 = pt(1.0, 0.0);
        let p3 = pt(0.0, 1.0);
        let p4 = pt(1.0, 1.0);
        assert!(
            !segments_intersect(&p1, &p2, &p3, &p4),
            "Parallel segments should not intersect"
        );
    }

    #[test]
    fn test_segments_intersect_shared_endpoint() {
        // Two segments sharing an endpoint: not a proper crossing
        let p1 = pt(0.0, 0.0);
        let p2 = pt(1.0, 0.0);
        let p3 = pt(1.0, 0.0);
        let p4 = pt(1.0, 1.0);
        assert!(
            !segments_intersect(&p1, &p2, &p3, &p4),
            "Segments sharing an endpoint should not count as intersecting"
        );
    }

    // -----------------------------------------------------------------------
    // valid_chord — unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_valid_chord_no_drawn() {
        let verts = regular_polygon(6);
        // Any non-adjacent pair is valid when nothing is drawn
        assert!(
            valid_chord(0, 2, &[], &verts),
            "Chord (0,2) should be valid when no chords are drawn"
        );
        assert!(
            valid_chord(0, 3, &[], &verts),
            "Chord (0,3) should be valid when no chords are drawn"
        );
    }

    #[test]
    fn test_valid_chord_crossing_rejected() {
        use crate::backtracking::Chord;
        let verts = regular_polygon(6);
        // (0,3) and (1,4) cross in a regular hexagon
        let drawn = vec![Chord { i: 0, j: 3, length: 0.0 }];
        assert!(
            !valid_chord(1, 4, &drawn, &verts),
            "Chord (1,4) should be invalid because it crosses (0,3)"
        );
    }

    #[test]
    fn test_valid_chord_sharing_endpoint_allowed() {
        use crate::backtracking::Chord;
        let verts = regular_polygon(6);
        // (0,2) and (0,4) share vertex 0, so they don't cross
        let drawn = vec![Chord { i: 0, j: 2, length: 0.0 }];
        assert!(
            valid_chord(0, 4, &drawn, &verts),
            "Chord (0,4) should be valid even with (0,2) drawn (they share vertex 0)"
        );
    }

    // -----------------------------------------------------------------------
    // DP chord reconstruction: returned chords must be non-crossing
    // -----------------------------------------------------------------------

    #[test]
    fn test_dp_chords_are_non_crossing() {
        for n in 4..=10usize {
            let verts = regular_polygon(n);
            let result = min_triangulation_dp(&verts);
            let chords = reconstruct_chords(&result);

            // Check every pair of chords
            for i in 0..chords.len() {
                for j in (i + 1)..chords.len() {
                    let (a, b) = chords[i];
                    let (c, d) = chords[j];
                    // Skip pairs sharing an endpoint
                    if a == c || a == d || b == c || b == d {
                        continue;
                    }
                    assert!(
                        !segments_intersect(
                            &verts[a], &verts[b],
                            &verts[c], &verts[d]
                        ),
                        "n={}: DP chords ({},{}) and ({},{}) should not cross",
                        n, a, b, c, d
                    );
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Greedy returned chords: non-crossing and correct count
    // -----------------------------------------------------------------------

    #[test]
    fn test_greedy_chords_are_non_crossing() {
        for n in 4..=10usize {
            let verts = regular_polygon(n);
            let result = min_triangulation_greedy(&verts);
            let chords: Vec<(usize, usize)> = result.chords.iter().map(|&(a, b, _)| (a, b)).collect();

            for i in 0..chords.len() {
                for j in (i + 1)..chords.len() {
                    let (a, b) = chords[i];
                    let (c, d) = chords[j];
                    if a == c || a == d || b == c || b == d {
                        continue;
                    }
                    assert!(
                        !segments_intersect(
                            &verts[a], &verts[b],
                            &verts[c], &verts[d]
                        ),
                        "n={}: Greedy chords ({},{}) and ({},{}) should not cross",
                        n, a, b, c, d
                    );
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // DP cost matches sum of reconstructed chord lengths
    // -----------------------------------------------------------------------

    #[test]
    fn test_dp_cost_matches_reconstructed_chord_lengths() {
        for n in 4..=10usize {
            let verts = regular_polygon(n);
            let result = min_triangulation_dp(&verts);
            let chords = reconstruct_chords(&result);

            let chord_sum: f64 = chords.iter().map(|&(a, b)| {
                let dx = verts[a].x - verts[b].x;
                let dy = verts[a].y - verts[b].y;
                (dx * dx + dy * dy).sqrt()
            }).sum();

            assert!(
                (chord_sum - result.cost).abs() < EPS,
                "n={}: sum of reconstructed chord lengths ({:.6}) != DP cost ({:.6})",
                n, chord_sum, result.cost
            );
        }
    }
}

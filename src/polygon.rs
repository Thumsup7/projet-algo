/// Represents a 2D point (polygon vertex)
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Returns true if segment (p1,p2) and segment (p3,p4) properly intersect
/// (not counting shared endpoints, which happen for adjacent chords)
pub fn segments_intersect(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
    let d1 = cross(p3, p4, p1);
    let d2 = cross(p3, p4, p2);
    let d3 = cross(p1, p2, p3);
    let d4 = cross(p1, p2, p4);

    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
    {
        return true;
    }
    false
}

fn cross(o: &Point, a: &Point, b: &Point) -> f64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

/// The heptagon from the instructions, used as a test case
pub fn heptagon() -> Vec<Point> {
    vec![
        Point::new(0.0, 10.0),  // s0
        Point::new(0.0, 20.0),  // s1
        Point::new(8.0, 26.0),  // s2
        Point::new(15.0, 26.0), // s3
        Point::new(27.0, 21.0), // s4
        Point::new(22.0, 12.0), // s5
        Point::new(10.0, 0.0),  // s6
    ]
}

/// Generate a regular convex polygon with n vertices (for testing)
pub fn regular_polygon(n: usize) -> Vec<Point> {
    use std::f64::consts::PI;
    (0..n)
        .map(|i| {
            let angle = 2.0 * PI * i as f64 / n as f64;
            Point::new(angle.cos() * 100.0, angle.sin() * 100.0)
        })
        .collect()
}

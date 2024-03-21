use std::{iter, fmt};
use unfold::unfold_count;

pub fn run() {
    const N: usize = 2 * usize::pow(10, 6);
    let mut points: Vec<Point> = gen_points(N).collect();
    points.sort_by(|a, b| a.x.cmp(&b.x));

    let preview: Vec<&Point> = points.iter().take(5).collect::<Vec<_>>();
    println!("{:#?}", preview);

    println!("{:?}", min_dist(&points));

    // let mut min_dist: f64 = f64::MAX;
    // let mut min_points: (usize, usize) = (0, 0);    
}

pub fn run_old() { // took 12 hours to get halfway lol
    const N: usize = 2 * usize::pow(10, 6);
    let points: Vec<Point> = gen_points(N).collect();
    let preview: Vec<&Point> = points.iter().take(5).collect::<Vec<_>>();
    println!("{:#?}", preview);

    let mut min_dist: f64 = f64::MAX;
    let mut min_points: (usize, usize) = (0, 0);
    // println!("{}", min_dist);

    const N_PERCENT: usize = N / 100;

    for i in 0..N {
        for j in i+1..N {
            let pt_dist = dist(points[i], points[j]);
            if pt_dist < min_dist {
                min_dist = pt_dist;
                min_points = (i, j);
            }
        }
        if i % (N_PERCENT / 100) == 0 {println!("{:.2}%", (i as f64) / ((N_PERCENT as f64)))}
    }

    println!("{:?}: {}", min_points, min_dist);
    
}

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the debug representation here
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}



fn gen_points(k: usize) -> impl Iterator<Item = Point> {
    let s0 = 290797;
    let s1 = next_pt(s0);
    unfold_count(
        |pt| {
            let b0 = next_pt(pt.y);
            let b1 = next_pt(b0);
            Point::new(b0, b1)
        },
        Point::new(s0, s1),
        k
    )
}

fn next_pt(s: isize) -> isize {
    s*s % 50515093
}

fn dist(p1: Point, p2: Point) -> f64 {
    (((p1.x-p2.x).pow(2) + (p1.y-p2.y).pow(2)) as f64).sqrt()
}

fn min_dist(points: &[Point]) -> ((Point, Point), f64) {
    let n = points.len();
    // println!("{:#?}", points);
    if n <= 3 {
        if n <= 1 {
            panic!("less than two points found");
        }
        else if n == 2 {
            let a = points[0]; let b = points[1];
            return ((a, b), dist(a, b));
        }
        else { // n = 3
            let a = points[0]; let b = points[1]; let c = points[2];
            let ab = dist(a,b); let ac = dist(a,c); let bc = dist(b,c);
            return match (ab < ac, ab < bc, ac < bc) {
                (true, true, _) => ((a, b), ab),
                (_, _, true) => ((a, c), ac),
                _ => ((b, c), bc),
            };
        }
    }
    else {
        let midpoint = n / 2;
        let mid_x = points[midpoint].x;
        let l_points = &points[..midpoint];
        let r_points = &points[midpoint..];

        let (l_pair, l_min) = min_dist(l_points);
        let (r_pair, r_min) = min_dist(r_points);

        let (mut min_points, mut delta) = if l_min < r_min {(l_pair, l_min)} else {(r_pair, r_min)};

        let l_filt: Vec<_> = l_points.iter().filter(|pt| pt.x > mid_x - (delta.ceil()  as isize)).collect(); // .sort_by(|a, b| a.y.cmp(&b.y));
        let r_filt: Vec<_> = r_points.iter().filter(|pt| pt.x < mid_x + (delta.floor() as isize)).collect(); // .sort_by(|a, b| a.y.cmp(&b.y));
        
        for l in l_filt.iter() {
            let delta_copy = delta;
            for r in r_filt.iter().filter(|pt| (pt.y-l.y).abs() < (delta_copy.ceil() as isize)) {
                let d = dist(**l, **r);
                if d < delta {
                    min_points = (**l, **r);
                    delta = d;
                }
                
            }
        }
        
        return (min_points, delta);

    }

}
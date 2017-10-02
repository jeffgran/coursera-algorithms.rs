#![feature(test)]

extern crate rand;
extern crate test;

use rand::distributions::{Range, IndependentSample};
use std::f64;
use std::convert::AsRef;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl AsRef<Point> for Point {
    fn as_ref(&self) -> &Point {
        &self
    }
}



fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        1000
    };
    let points: Vec<Point> = generate(n);
    let point_refs: Vec<&Point> = points.iter().map(AsRef::as_ref).collect();

    let (a, b, z) = closest_pair_n2(&point_refs);
    let (a2, b2, z2) = closest_pair_nlogn(&point_refs);

    println!("-------------------------------------------------------");
    println!("Slow algorithm:");
    println!("Closest pair of points was: {:?}, {:?}", a, b);
    println!("Distance was: {}", z);
    println!("-------------------------------------------------------");
    println!("Fast algorithm:");
    println!("Closest pair of points was: {:?}, {:?}", a2, b2);
    println!("Distance was: {}", z2);
    println!("-------------------------------------------------------");

    if z != z2 {
        println!("WRONG! Input was: {:?}", points);
    }
}

fn generate(n: usize) -> Vec<Point> {
    let range = Range::new(-10000, 10000);
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(Point {
            x: range.ind_sample(&mut rng),
            y: range.ind_sample(&mut rng),
        });
    }
    v
}


fn closest_pair_n2<'a>(points: &[&'a Point]) -> (&'a Point, &'a Point, f64) {
    let mut closest_pair: (&Point, &Point, f64) = (&points[0], &points[1], f64::MAX);
    for (i, pi) in points.iter().enumerate() {
        for (j, pj) in points.iter().enumerate() {
            if i == j {
                // next
            } else {
                let dij = delta(pi, pj);
                if dij < closest_pair.2 {
                    closest_pair = (&pi, &pj, dij);
                }
            }
        }
    }
    closest_pair
}

fn closest_pair_nlogn<'a>(points: &Vec<&'a Point>) -> (&'a Point, &'a Point, f64) {
    let mut sorted_by_x = points.clone();
    sorted_by_x.sort_by(|a, b| a.x.cmp(&b.x));
    let mut sorted_by_y = points.clone();
    sorted_by_y.sort_by(|a, b| a.y.cmp(&b.y));

    closest_pair_nlogn_recurse(&sorted_by_x, &sorted_by_y)
}

fn closest_pair_nlogn_recurse<'a>(
    px: &[&'a Point],
    py: &[&'a Point],
) -> (&'a Point, &'a Point, f64) {
    if px.len() <= 3 {
        return closest_pair_n2(px);
    }

    let mid = px.len() / 2;
    let (q_x, r_x) = px.split_at(mid);
    let biggest_qx = q_x[mid - 1].x;
    let mut q_y = Vec::new();
    let mut r_y = Vec::new();
    for p in py {
        if p.x <= biggest_qx {
            q_y.push(*p);
        } else {
            r_y.push(*p)
        }
    }


    let (p1, q1, d1) = closest_pair_nlogn_recurse(&q_x, &q_y[..]);
    let (p2, q2, d2) = closest_pair_nlogn_recurse(&r_x, &r_y[..]);
    // println!("----------------------");
    // println!("q_x: {:?}", q_x);
    // println!("q_y: {:?}", q_y);
    // println!("result: ({:?}, {:?}, {})", p1, q1, d1);
    // println!("----------------------");
    // println!("r_x: {:?}", r_x);
    // println!("r_y: {:?}", r_y);
    // println!("result: ({:?}, {:?}, {})", p2, q2, d2);
    // println!("----------------------");
    let min = d1.min(d2);
    let (newp, newq, newd) = match closest_split_pair(px, py, min) {
        Some((p3, q3, d3)) => {
            // println!("----------------------");
            // println!("(p3, q3, d3) = ({:?}, {:?}, {})", p3, q3, d3);
            // println!("----------------------");
            (p3, q3, d3)
        }
        None => {
            if min == d1 {
                (p1, q1, d1)
            } else {
                (p2, q2, d2)
            }
        }
    };

    // println!("(p1, q1, d1) = ({:?}, {:?}, {})", p1, q1, d1);
    // println!("(p2, q2, d2) = ({:?}, {:?}, {})", p2, q2, d2);
    // println!("(newp, newq, newd) = ({:?}, {:?}, {})", newp, newq, newd);
    // println!("--------------------------------------------");

    (&newp, &newq, newd)
}

fn closest_split_pair<'a>(
    px: &[&'a Point],
    py: &[&'a Point],
    d: f64,
) -> Option<(&'a Point, &'a Point, f64)> {
    let midxi = px.len() / 2;
    let barx = px[midxi].x as f64;

    let minx = barx - d;
    let maxx = barx + d;
    let mut s_y = Vec::new();
    for p in py.iter() {
        if p.x as f64 >= minx && p.x as f64 <= maxx {
            s_y.push(p);
        }
    }
    // println!("    -----------------------------");
    // println!("    s_y is {:?}", s_y);
    // println!("    px.len() = {}", px.len());
    // println!("    barx = {}", barx);
    // println!("    minx = {}", minx);
    // println!("    maxx = {}", maxx);
    // println!("    -----------------------------");

    let mut best_delta = d;
    let mut best_pair: Option<(&Point, &Point)> = None;

    for i in 0..(s_y.len()) {
        for j in 1..7 {
            let p = s_y[i];
            match s_y.get(i + j) {
                Some(q) => {
                    let dd = delta(p, q);
                    if dd < best_delta {
                        best_delta = dd;
                        best_pair = Some((p, q));
                    }
                }
                None => {}
            }
        }

    }


    match best_pair {
        Some((p, q)) => Some((p, q, best_delta)),
        None => None,
    }
}


fn delta(a: &Point, b: &Point) -> f64 {
    let delta_x_2 = (b.x - a.x).pow(2);
    let delta_y_2 = (b.y - a.y).pow(2);
    ((delta_x_2 + delta_y_2) as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_delta() {
        let a = Point { x: 1, y: 1 };
        let b = Point { x: 3, y: 3 };
        assert_eq!(delta(&a, &b), (8u32 as f64).sqrt());
    }

    #[test]
    fn test_case_1() {
        let points = vec![
            Point { x: 773, y: -921 },
            Point { x: -458, y: -90 },
            Point { x: -968, y: -42 },
            Point { x: 154, y: 185 },
            Point { x: 173, y: 210 },
            Point { x: 664, y: 93 },
            Point { x: -307, y: 967 },
            Point { x: -808, y: -367 },
            Point { x: 421, y: 249 },
            Point { x: 406, y: 813 },
            Point { x: 291, y: 13 },
            Point { x: -155, y: 815 },
            Point { x: -191, y: -47 },
            Point { x: 431, y: -905 },
            Point { x: -443, y: -19 },
            Point { x: -211, y: -109 },
            Point { x: -238, y: 386 },
            Point { x: 605, y: -59 },
            Point { x: 597, y: -628 },
            Point { x: -82, y: 885 },
            Point { x: 392, y: 774 },
            Point { x: -97, y: 565 },
            Point { x: 427, y: 336 },
            Point { x: -214, y: 308 },
            Point { x: 859, y: -338 },
            Point { x: 562, y: -768 },
            Point { x: -962, y: 295 },
            Point { x: -651, y: 292 },
            Point { x: 995, y: -728 },
            Point { x: 219, y: 914 }, // p
            Point { x: 775, y: -392 },
            Point { x: -404, y: -529 },
            Point { x: 141, y: 863 },
            Point { x: 221, y: 926 }, // q
            Point { x: 993, y: -463 },
            Point { x: 91, y: 214 },
            Point { x: -627, y: -83 },
            Point { x: 747, y: -981 },
            Point { x: -223, y: -224 },
            Point { x: 736, y: 285 },
            Point { x: 619, y: -570 },
            Point { x: -808, y: -344 },
            Point { x: 390, y: -140 },
        ];
        let point_refs: Vec<&Point> = points.iter().map(AsRef::as_ref).collect();
        let (a, b, z) = closest_pair_n2(&point_refs);
        let (a2, b2, z2) = closest_pair_nlogn(&point_refs);

        println!("-------------------------------------------------------");
        println!("Slow algorithm:");
        println!("Closest pair of points was: {:?}, {:?}", a, b);
        println!("Distance was: {}", z);
        println!("-------------------------------------------------------");
        println!("Fast algorithm:");
        println!("Closest pair of points was: {:?}, {:?}", a2, b2);
        println!("Distance was: {}", z2);
        println!("-------------------------------------------------------");

        assert_eq!(z, z2);
    }

    #[test]
    fn test_case_2() {
        let points = vec![
            Point { x: 781, y: -417 },
            Point { x: -392, y: 814 },
            Point { x: -459, y: -869 },
            Point { x: -718, y: -884 },
            Point { x: 237, y: 489 },
            Point { x: -334, y: 630 },
            Point { x: 39, y: 701 },
            Point { x: 755, y: 331 },
            Point { x: 911, y: 769 },
            Point { x: -504, y: 203 },
            Point { x: 275, y: 20 },
            Point { x: -487, y: 14 },
            Point { x: 789, y: 104 },
            Point { x: 104, y: 405 },
            Point { x: 861, y: 23 },
            Point { x: 921, y: -488 },
            Point { x: -30, y: 654 },
            Point { x: -624, y: 736 },
            Point { x: 333, y: -929 },
            Point { x: -839, y: -500 },
            Point { x: 120, y: -672 },
            Point { x: -742, y: -709 },
            Point { x: -323, y: 934 },
            Point { x: -199, y: 728 },
            Point { x: 469, y: -552 },
            Point { x: -217, y: -432 },
            Point { x: -344, y: -476 },
            Point { x: -692, y: -359 },
            Point { x: 182, y: -287 },
            Point { x: 625, y: 1 },
            Point { x: -395, y: 508 },
            Point { x: -699, y: 119 },
            Point { x: 490, y: 197 },
            Point { x: 67, y: 115 },
            Point { x: -304, y: -58 },
            Point { x: -109, y: 863 },
            Point { x: 100, y: 709 },
            Point { x: -587, y: -394 },
            Point { x: 321, y: -350 },
            Point { x: 650, y: -196 },
            Point { x: -440, y: -703 },
            Point { x: 873, y: -725 },
            Point { x: 799, y: 581 },
            Point { x: 592, y: -717 },
            Point { x: 460, y: 610 },
            Point { x: 474, y: -236 },
            Point { x: -147, y: 688 },
            Point { x: 622, y: -326 },
            Point { x: 493, y: -113 },
            Point { x: -346, y: -345 },
            Point { x: -376, y: -498 },
            Point { x: -103, y: -77 },
            Point { x: 535, y: 870 },
            Point { x: -40, y: -835 },
            Point { x: -30, y: -80 },
            Point { x: 94, y: -357 },
            Point { x: 864, y: 215 },
            Point { x: -147, y: -445 },
            Point { x: -255, y: -462 },
            Point { x: 984, y: 429 },
            Point { x: -516, y: -832 },
            Point { x: 87, y: -682 },
            Point { x: 912, y: -966 },
            Point { x: -672, y: 956 },
            Point { x: -328, y: 115 },
            Point { x: 384, y: 27 },
            Point { x: -55, y: -454 },
            Point { x: -103, y: 572 },
            Point { x: 581, y: -818 },
            Point { x: -208, y: 162 },
            Point { x: 511, y: 583 },
            Point { x: 605, y: 412 },
            Point { x: 515, y: -523 },
            Point { x: -675, y: 661 },
            Point { x: -212, y: -505 },
            Point { x: -448, y: -241 },
            Point { x: -749, y: 709 },
            Point { x: -130, y: -481 },
            Point { x: -596, y: 980 },
            Point { x: 502, y: -41 },
            Point { x: 759, y: -758 },
            Point { x: -293, y: -747 },
            Point { x: -722, y: -216 },
            Point { x: -89, y: -289 },
            Point { x: -894, y: 624 },
            Point { x: -293, y: -892 },
            Point { x: -592, y: -901 },
            Point { x: -833, y: 451 },
            Point { x: -395, y: 363 },
            Point { x: -138, y: -542 },
            Point { x: -868, y: -937 },
            Point { x: 114, y: -672 },
            Point { x: 420, y: 386 },
            Point { x: 735, y: -909 },
            Point { x: 366, y: -499 },
            Point { x: 657, y: -487 },
            Point { x: -44, y: -971 },
            Point { x: -687, y: -62 },
            Point { x: -289, y: -385 },
            Point { x: 42, y: -408 },
        ];
        let point_refs: Vec<&Point> = points.iter().map(AsRef::as_ref).collect();
        let (a, b, z) = closest_pair_n2(&point_refs);
        let (a2, b2, z2) = closest_pair_nlogn(&point_refs);

        println!("-------------------------------------------------------");
        println!("Slow algorithm:");
        println!("Closest pair of points was: {:?}, {:?}", a, b);
        println!("Distance was: {}", z);
        println!("-------------------------------------------------------");
        println!("Fast algorithm:");
        println!("Closest pair of points was: {:?}, {:?}", a2, b2);
        println!("Distance was: {}", z2);
        println!("-------------------------------------------------------");

        assert_eq!(z, z2);
    }

    #[bench]
    fn bench_n2(b: &mut Bencher) {
        let points: Vec<Point> = generate(10000);
        let point_refs: Vec<&Point> = points.iter().map(AsRef::as_ref).collect();
        b.iter(|| {
            closest_pair_n2(&point_refs);
        });
    }

    #[bench]
    fn bench_nlogn(b: &mut Bencher) {
        let points: Vec<Point> = generate(10000);
        let point_refs: Vec<&Point> = points.iter().map(AsRef::as_ref).collect();
        b.iter(|| {
            closest_pair_nlogn(&point_refs);
        });
    }
}

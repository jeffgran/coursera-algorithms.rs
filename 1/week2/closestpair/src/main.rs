use std::f64;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    //let points = generate()
    let points = vec![Point{x:1,y:2}, Point{x:99,y:0}, Point{x:0, y:99}, Point{x:2,y:3}];
    let closest_pair = closest_pair_n3(&points);
    let (a, b, z) = closest_pair;
    println!("Closest pair of points was: {:?}, {:?}", a, b);
    println!("Distance was: {}", z);
}

fn generate() {

}


fn closest_pair_n2(points: &Vec<Point>) -> (&Point, &Point, f64) {
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


fn delta(a: &Point, b: &Point) -> f64 {
    let delta_x_2 = (b.x - a.x).pow(2);
    let delta_y_2 = (b.y - a.y).pow(2);
    ((delta_x_2 + delta_y_2) as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta() {
        let a = Point{x: 1, y: 1};
        let b = Point{x: 3, y: 3};
        assert_eq!(delta(&a, &b), (8u32 as f64).sqrt());
    }
}

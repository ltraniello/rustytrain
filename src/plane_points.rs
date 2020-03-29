
use std::vec::Vec;
use std::collections::hash_map::{HashMap, Entry};
use std::hash::{Hash, Hasher};
use std::fmt;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

pub fn fill_points_between(tl_x: i32, tl_y: i32, br_x: i32, br_y: i32, points: &mut Vec::<Point>) {
    let mut x = tl_x;
    while x <= br_x {
        let mut y = tl_y;
        while y <= br_y {
            points.push(Point {x:x, y:y});
            y += 1;
        }
        x += 1;
    }
}

pub fn count_orthog(points: &Vec::<Point>) -> i32 {
    let mut count = 0;
    for (_i, a) in points.iter().enumerate() {
        for (_j, b) in points.iter().enumerate() {
            for (_k, c) in points.iter().enumerate() {
                for (_l, d) in points.iter().enumerate() {
                    if (a.x == d.x) & (b.x == c.x) & (a.y == b.y) & (d.y == c.y) {
                        count += 1;
                    }
                }
            }
        }
    }
    count / 4
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) & (self.y == other.y)
    }
}

impl std::cmp::Eq for Point {
}

pub fn count_orthog_fast_1(points: &Vec::<Point>) -> i32 {
    
    let mut count = 0;
    let mut vertical_pairs_by_length = HashMap::new();
    for (i, a) in points.iter().enumerate() {
        for (j, b) in points.iter().enumerate() {
            if (j > i) & (a.x == b.x) {
                let len;
                let pair_index;
                if b.y-a.y >= 0 {
                    len = b.y-a.y;
                    pair_index = (i,j);
                } else {
                    len = a.y-b.y;
                    pair_index = (j,i);
                }
                match vertical_pairs_by_length.entry(len) {
                    Entry::Vacant(e) => { e.insert(vec![pair_index]); },
                    Entry::Occupied(mut e) => { e.get_mut().push(pair_index); }
                }
            }
        }
    }

    for (_len, pair) in vertical_pairs_by_length {
        for (i, ab_idx) in pair.iter().enumerate() {
            for (j, cd_idx) in pair.iter().enumerate() {
                if j <= i {
                    continue;
                }
                if points[ab_idx.0].y == points[cd_idx.0].y {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn count_orthog_fast_2(points: &Vec::<Point>) -> i32 {
    
    let mut count = 0;
    let mut vertical_pair = HashMap::new();

    for a in points.iter() {
        for b in points.iter() {
            if (a.x == b.x) & (a.y < b.y) {
                match vertical_pair.entry((a.y,b.y)) {
                    Entry::Vacant(e) => { e.insert(1); },
                    Entry::Occupied(mut e) => { count+=*e.get_mut(); *e.get_mut()+=1; }
                }
            }
        }
    }
    count
}

pub fn count_fast_1(points: &Vec::<Point>) -> i32 {
    
    let mut count = 0;
    let mut diag_pairs_by_slope = HashMap::new();

    for (i, a) in points.iter().enumerate() {
        for (j, b) in points.iter().enumerate() {
            if (a.x <= b.x) & (a.y < b.y) {
                let pair_index = (i, j);
                match diag_pairs_by_slope.entry((b.x-a.x, b.y-a.y)) {
                    Entry::Vacant(e) => { e.insert(vec![pair_index]); },
                    Entry::Occupied(mut e) => {
                        for indexes in e.get() {
                            let c = &points[indexes.0];

                            let scalar = (b.x-a.x)*(c.x-a.x)+(b.y-a.y)*(c.y-a.y);
                            if scalar == 0 {
                                count+=1;
                            }
                        }
                        e.get_mut().push(pair_index);
                    }
                }
            }
        }
    }
    count
}


#[cfg(test)] // Only compiles when running tests
mod plane_points_test {

    use super::*;

    #[test]
    fn empty() {
        let mut points = Vec::new();
        points.push(Point {x:0, y:0});
        assert_eq!(0, count_orthog_fast_1(&points));
        assert_eq!(0, count_orthog_fast_2(&points));
        assert_eq!(0, count_fast_1(&points));
        points.push(Point {x:0, y:1});
        assert_eq!(0, count_orthog_fast_1(&points));
        assert_eq!(0, count_orthog_fast_2(&points));
        assert_eq!(0, count_fast_1(&points));
        points.push(Point {x:1, y:0});
        assert_eq!(0, count_orthog_fast_1(&points));
        assert_eq!(0, count_orthog_fast_2(&points));
        assert_eq!(0, count_fast_1(&points));
    }

    #[test]
    fn one() {
        let mut points = Vec::new();
        points.push(Point {x:0, y:0});
        points.push(Point {x:0, y:1});
        points.push(Point {x:1, y:0});
        points.push(Point {x:1, y:1});
        assert_eq!(1, count_orthog_fast_1(&points));
        assert_eq!(1, count_orthog_fast_2(&points));
        assert_eq!(1, count_fast_1(&points));
    }

    #[test]
    fn two_diag() {
        let mut points = Vec::new();
        points.push(Point {x:0, y:0});
        points.push(Point {x:1, y:1});
        points.push(Point {x:-1, y:1});
        points.push(Point {x:0, y:2});
        points.push(Point {x:2, y:2});
        points.push(Point {x:-2, y:2});
        points.push(Point {x:0, y:4});
        assert_eq!(0, count_orthog_fast_1(&points));
        assert_eq!(0, count_orthog_fast_2(&points));
        assert_eq!(2, count_fast_1(&points));
    }

    #[test]
    fn three() {
        let mut points = Vec::new();
        points.push(Point {x:0, y:0});
        points.push(Point {x:0, y:1});
        points.push(Point {x:1, y:0});
        points.push(Point {x:1, y:1});
        points.push(Point {x:2, y:0});
        points.push(Point {x:2, y:1});
        assert_eq!(3, count_orthog_fast_1(&points));
        assert_eq!(3, count_orthog_fast_2(&points));
        assert_eq!(3, count_fast_1(&points));
    }

    fn rect_count_in_grid_n_m(n: i32, m: i32) -> i32 {
        (m * n * (n + 1) * (m + 1)) / 4
    }

    #[test]
    fn grid_n_m() {
        let mut points = Vec::new();
        fill_points_between(0, 0, 3, 3, &mut points);
        assert_eq!( rect_count_in_grid_n_m(3, 3), count_orthog_fast_1(&points));
        assert_eq!( rect_count_in_grid_n_m(3, 3), count_orthog_fast_2(&points));
        
        points = Vec::new();
        fill_points_between(-10, -10, 10, 10, &mut points);
        assert_eq!(rect_count_in_grid_n_m(20, 20), count_orthog_fast_1(&points));
        assert_eq!(rect_count_in_grid_n_m(20, 20), count_orthog_fast_2(&points));
    }
}
#![feature(test)]

use count_rects;

extern crate test;

use count_rects::Point;
use std::vec::Vec;

#[test]
fn empty() {
    let mut points = Vec::new();
    points.push(Point {x:0, y:0});
    assert_eq!(0, count_rects::count_orthog_fast_1(&points));
    assert_eq!(0, count_rects::count_orthog_fast_2(&points));
    assert_eq!(0, count_rects::count_fast_1(&points));
    points.push(Point {x:0, y:1});
    assert_eq!(0, count_rects::count_orthog_fast_1(&points));
    assert_eq!(0, count_rects::count_orthog_fast_2(&points));
    assert_eq!(0, count_rects::count_fast_1(&points));
    points.push(Point {x:1, y:0});
    assert_eq!(0, count_rects::count_orthog_fast_1(&points));
    assert_eq!(0, count_rects::count_orthog_fast_2(&points));
    assert_eq!(0, count_rects::count_fast_1(&points));
}

#[test]
fn one() {
    let mut points = Vec::new();
    points.push(Point {x:0, y:0});
    points.push(Point {x:0, y:1});
    points.push(Point {x:1, y:0});
    points.push(Point {x:1, y:1});
    assert_eq!(1, count_rects::count_orthog_fast_1(&points));
    assert_eq!(1, count_rects::count_orthog_fast_2(&points));
    assert_eq!(1, count_rects::count_fast_1(&points));
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
    assert_eq!(0, count_rects::count_orthog_fast_1(&points));
    assert_eq!(0, count_rects::count_orthog_fast_2(&points));
    assert_eq!(2, count_rects::count_fast_1(&points));
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
    assert_eq!(3, count_rects::count_orthog_fast_1(&points));
    assert_eq!(3, count_rects::count_orthog_fast_2(&points));
    assert_eq!(3, count_rects::count_fast_1(&points));
}

fn rect_count_in_grid_n_m(n: i32, m: i32) -> i32 {
    (m * n * (n + 1) * (m + 1)) / 4
}

#[test]
fn grid_n_m() {
    let mut points = Vec::new();
    count_rects::fill_points_between(0, 0, 3, 3, &mut points);
    assert_eq!( rect_count_in_grid_n_m(3, 3), count_rects::count_orthog_fast_1(&points));
    assert_eq!( rect_count_in_grid_n_m(3, 3), count_rects::count_orthog_fast_2(&points));
    
    points = Vec::new();
    count_rects::fill_points_between(-10, -10, 10, 10, &mut points);
    assert_eq!(rect_count_in_grid_n_m(20, 20), count_rects::count_orthog_fast_1(&points));
    assert_eq!(rect_count_in_grid_n_m(20, 20), count_rects::count_orthog_fast_2(&points));
}

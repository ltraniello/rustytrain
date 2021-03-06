use std::vec::Vec;
use std::time::{Instant};


type FnCountRect = fn(&Vec::<rustytrain::plane_points::Point>) -> i32;

struct DecoratedFunc {
    f: FnCountRect,
    fname: String,
    maxSize: i32,
}

fn main() {
    let array_n: [i32; 7] = [4,14,30,50,90,200,400];

    let mut fns = Vec::new();
    fns.push(DecoratedFunc{f: rustytrain::plane_points::count_orthog_fast_1, fname: String::from("count_orthog_fast_1"), maxSize: 1000});
    fns.push(DecoratedFunc{f: rustytrain::plane_points::count_orthog_fast_2, fname: String::from("count_orthog_fast_2"), maxSize: 1000000});
    fns.push(DecoratedFunc{f: rustytrain::plane_points::count_fast_1,        fname: String::from("count_fast_1       "), maxSize: 1000});

    for n in &array_n {
        let mut points = Vec::new();
        rustytrain::plane_points::fill_points_between(0, 0, *n, *n, &mut points);

        let points_len = points.len() as i32;

        println!("{} points", points_len);

        for df in &fns {
            if points_len < df.maxSize {
                let now = Instant::now();
                let rects = (df.f)(&points);
                println!("\t\t{}\t{}\t{} ms", df.fname, rects, now.elapsed().as_millis());
            } else {
                println!("\t\t{}\t no data, too large", df.fname);
            }
        }
    }
}
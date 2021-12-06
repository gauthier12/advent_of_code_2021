#[macro_use]
extern crate scan_fmt;
extern crate ndarray;
use ndarray::Array2;
use std::cmp;
use std::env;
use std::fs;

struct Point {
    x: i32,
    y: i32,
}
struct Line {
    p1: Point,
    p2: Point,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    const GRID_SIZE: usize = 1000;
    let mut line_list: Vec<Line> = Vec::new();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let file_content = contents.lines();
    for s in file_content {
        if let Ok((x1, y1, x2, y2)) = scan_fmt!(
            s,                 // input string
            "{},{} -> {},{} ", // format
            i32, i32,
            i32,
            i32
        ) {
            let tmp_line: Line = Line {
                p1: Point { x: x1, y: y1 },
                p2: Point { x: x2, y: y2 },
            };
            line_list.push(tmp_line);
        } else {
            panic!("Parsing error")
        }
    }
    let mut map = Array2::<i32>::zeros((GRID_SIZE, GRID_SIZE));

    for line in line_list {
        let x_interval = line.p2.x - line.p1.x;
        let y_interval = line.p2.y - line.p1.y;
        if !(x_interval == 0 || y_interval == 0 || x_interval.abs() == y_interval.abs()) {
            panic!("Problem in the line");
        }
        let x_sign = if x_interval.abs() == 0 {
            0
        } else {
            x_interval / x_interval.abs()
        };
        let y_sign = if y_interval.abs() == 0 {
            0
        } else {
            y_interval / y_interval.abs()
        };
        let alpha_int = 1+cmp::max(x_interval.abs(),y_interval.abs());
        for alpha in 0..alpha_int {
            let xcur = (line.p1.x + x_sign * alpha) as usize;
            let ycur = (line.p1.y + y_sign * alpha) as usize;
            map[[xcur as usize, ycur as usize]] += 1;
        }
    }

    let mut nb_term2 = 0;
    for i_term in map.iter() {
        if *i_term > 1 {
            nb_term2 += 1;
        }
    }
    println!("Number of cases with more than 2 lines : {}", nb_term2);
}

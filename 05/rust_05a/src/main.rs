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
        if line.p1.x == line.p2.x {
            println!("line is horizontal");
            println!(
                "x1 = {} | y1={} | x2 = {} | y2 = {}",
                line.p1.x, line.p1.y, line.p2.x, line.p2.y
            );
            let ymin = cmp::min(line.p1.y, line.p2.y);
            let ymax = cmp::max(line.p1.y, line.p2.y);
            let xcur = line.p1.x;
            for ycur in ymin..(ymax+1) {
                map[[xcur as usize, ycur as usize]] += 1;
            }
        }
        if line.p1.y == line.p2.y {
            println!("line is vertical");
            println!(
                "x1 = {} | y1={} | x2 = {} | y2 = {}",
                line.p1.x, line.p1.y, line.p2.x, line.p2.y
            );
            let xmin = cmp::min(line.p1.x, line.p2.x);
            let xmax = cmp::max(line.p1.x, line.p2.x);
            let ycur = line.p1.y;
            for xcur in xmin..(xmax+1) {
                    map[[xcur as usize, ycur as usize]] += 1;
            }
        }
    }
    /*
    for xcur in 0..GRID_SIZE
    {
        for ycur in 0..GRID_SIZE
        {
            print!(" {} ", map[[ycur, xcur]]);
        }
        println!("");
    }*/
    let mut nb_term2 = 0;
    for i_term in map.iter()
    {
        if *i_term > 1
        {
            nb_term2+=1;
        }
    }
    println!("Number of case with more than 2 lines : {}", nb_term2);
}

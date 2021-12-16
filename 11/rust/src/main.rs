extern crate ndarray;
use ndarray::Array2;

use std::env;
use std::fs;

const ROW_NUM: usize = 10;
const LINE_NUM: usize = 10;

fn reset_octopus(map: &mut Array2<u32>) -> () {
    for val in map.into_iter() {
        if *val >= 10 {
            *val = 0;
        }
    }
}

fn activate_octopus(map: &mut Array2<u32>, location: (usize, usize)) -> u32 {
    let mut n_flash = 0;
    map[location] += 1;
    if map[location] == 10 {
        n_flash += 1;
        let x = location.0;
        let y = location.1;
        for x_pos in std::cmp::max(1, x) - 1..std::cmp::min(LINE_NUM, x + 2) {
            for y_pos in std::cmp::max(1, y) - 1..std::cmp::min(ROW_NUM, y + 2) {
                let i_loc = (x_pos, y_pos);
                if i_loc != location {
                    n_flash += activate_octopus(map, i_loc);
                }
            }
        }
    }
    return n_flash;
}

fn main() {
    const RADIX: u32 = 10;
    let mut map = Array2::<u32>::zeros((LINE_NUM, ROW_NUM));
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for (ln, l) in contents.lines().enumerate() {
        for (cn, c) in l.chars().enumerate() {
            if let Some(cur_val) = c.to_digit(RADIX) {
                map[[ln, cn]] = cur_val;
            }
        }
    }
    let mut n_flash = 0;
    for i_step in 1..=10000 {
        for x_pos in 0..LINE_NUM {
            for y_pos in 0..ROW_NUM {
                n_flash += activate_octopus(&mut map, (x_pos, y_pos));
            }
        }
        reset_octopus(&mut map);
        println!("Step {:03} : {:04} flashes", i_step, n_flash);
        if map.iter().all(|cval| *cval == 0) {
            break;
        }
    }
}

//extern crate colored;
//use colored::*;
/*fn print_octopus(map: &Array2<u32>) -> () {
    let mut strmap = Array2::<colored::ColoredString>::default((LINE_NUM, ROW_NUM));
    for (loc, val) in map.indexed_iter() {
        if *val == 0 {
            strmap[loc] = val.to_string().yellow().bold();
        } else {
            strmap[loc] = val.to_string().normal();
        }
    }
    println!("{}", strmap);
}*/
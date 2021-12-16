extern crate ndarray;
use ndarray::s;
use ndarray::Array2;
use std::env;
use std::fs;

const ROW_NUM: usize = 100;
const LINE_NUM: usize = 100;

fn search_bassin_neighbour(map: &Array2<u32>,bassin: &mut Array2<u32>,location: (usize, usize),min_num: usize) -> Option<u32> {
    if map[location] == 9 || bassin[location] != 0
    {
        None
    }
    else
    {
        let mut bassin_size = 1;
        bassin[location] = min_num as u32;
        for n_x in std::cmp::max(1, location.0) - 1..std::cmp::min(LINE_NUM, location.0 + 2)
        {
            let n_loc = (n_x, location.1);
            if map[n_loc] > map[location]
            {
                if let Some(n_size) = search_bassin_neighbour(map, bassin, n_loc, min_num)
                {
                    bassin_size += n_size;
                }
            }
        }
        for n_y in std::cmp::max(1, location.1) - 1..std::cmp::min(ROW_NUM, location.1 + 2)
        {
            let n_loc = (location.0, n_y);
            if map[n_loc] > map[location]
            {
                if let Some(n_size) = search_bassin_neighbour(map, bassin, n_loc, min_num)
                {
                    bassin_size += n_size;
                }
            }
        }
        Some(bassin_size)
    }
}

fn main() {
    const RADIX: u32 = 10;
    let mut map = Array2::<u32>::zeros((LINE_NUM, ROW_NUM));
    let mut bassin = Array2::<u32>::zeros((LINE_NUM, ROW_NUM));
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for (ln, l) in contents.lines().enumerate()
    {
        for (cn, c) in l.chars().enumerate()
        {
            if let Some(cur_val) = c.to_digit(RADIX)
            {
                map[[ln, cn]] = cur_val;
            }
        }
    }
    let mut local_min: Vec<(usize, usize)> = Vec::new();
    let mut risk_level = 0;
    for ((x, y), value) in map.indexed_iter()
    {
        let bh = map.slice(s![std::cmp::max(1, x) - 1..std::cmp::min(LINE_NUM, x + 2),y]);
        let bv = map.slice(s![x,std::cmp::max(1, y) - 1..std::cmp::min(ROW_NUM, y + 2)]);
        let num_min_val = bh.iter().filter(|cval| *cval <= value).count() + bv.iter().filter(|cval| *cval <= value).count() - 2;
        if num_min_val == 0
        {
            local_min.push((x, y));
            risk_level += 1 + value;
        }
    }
    println!("Number of local minimum : {}", local_min.len());
    println!("Risk level : {}", risk_level);
    let mut size_lize: Vec<u32> = Vec::new();
    for (i_min, l_min) in local_min.iter().enumerate()
    {
        if let Some(size) = search_bassin_neighbour(&map, &mut bassin, *l_min, i_min)
        {
            size_lize.push(size);
        }
    }
    size_lize.sort_unstable();
    let mut multiplication = 1;
    for i_max in &size_lize[size_lize.len() - 3..size_lize.len()]
    {
        multiplication *= i_max;
    }
    println!("Size multiplication : {}", multiplication);
}

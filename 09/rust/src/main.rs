
extern crate ndarray;
use ndarray::Array2;
use ndarray::{s};
use std::env;
use std::fs;

fn main() {
    const RADIX: u32 = 10;
    const ROW_NUM : usize = 11;
    const LINE_NUM: usize = 5;
    let mut map = Array2::<u32>::zeros((LINE_NUM, ROW_NUM));
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    for (ln,l) in contents.lines().enumerate()
    {
        for (cn,c) in l.chars().enumerate()
        {
            if let Some(cur_val) = c.to_digit(RADIX)
            {
                map[[ln, cn]] =cur_val;
            }
        }
    }
    println!("{}", map);
    /*for ((x, y), value) in map.indexed_iter() {
        println!("[{} {}] : {}", x,y,value);
        for bx in std::cmp::max(1,x)-1..std::cmp::min(LINE_NUM,x+2)
        {
            for by in std::cmp::max(1,y)-1..std::cmp::min(ROW_NUM,y+2)
            {
                println!("slice1 {} {} : {}", bx, by,map[[bx,by]]);

            }
        }
    }*/
    for ((x, y), value) in map.indexed_iter() {
        println!("===========================");
        println!("[{} {}] : {}", x,y,value);
        let b = map.slice(s![std::cmp::max(1,x)-1..std::cmp::min(LINE_NUM,x+2), std::cmp::max(1,y)-1..std::cmp::min(ROW_NUM,y+2)]);
        let min_val = b.iter().filter(|cval| *cval <= value).count();
        println!("{}",b);
        println!("count  {}", min_val);

    }
    println!("{}", map);
}

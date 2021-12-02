use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let mut prec_val = 99999;
    let mut n_inc = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let new_val = ip.parse::<i32>().unwrap();
                if new_val > prec_val
                {
                    println!("Plus grand");
                    n_inc += 1;
                }
                else
                {
                    println!("Plus petit");
                }
                prec_val = new_val;
            }
        }
    }
    println!("Nombre d'augmentation : {}",n_inc);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

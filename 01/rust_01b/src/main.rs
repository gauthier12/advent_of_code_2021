use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let mut prec_val: [i32; 3] = [99999; 3];
    let mut n_inc = 0;
    // File hosts must exist in current path before this produces output
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let tab_content = contents.lines();
    for s in tab_content {
        let new_val = s.parse::<i32>().unwrap();
        if new_val  >  prec_val[2]
        {
            n_inc += 1;
        }
        prec_val[2] = prec_val[1];
        prec_val[1] = prec_val[0];
        prec_val[0] = new_val;
    }
    println!("Nombre d'augmentation : {}",n_inc);
}
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let mut pos_hor= 0;
    let mut pos_ver= 0;
    let mut aim= 0;
    // File hosts must exist in current path before this produces output
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let tab_content = contents.lines();
    for s in tab_content {
        let splitted_line : Vec<_> = s.split_whitespace().collect();
        let command = splitted_line[0];
        let val = splitted_line[1].parse::<i32>().unwrap();
        match command{
            "up" =>
            {
                aim -=  val;
            },
            "down" =>
            {
                aim +=  val;
            },
            "forward" =>
            {
                pos_hor +=  val;
                pos_ver += aim * val;
            },
            _=>println!("wrong command"),
        };
    }
    println!("position depth {} hor {}", pos_ver, pos_hor);
    println!("solution {}", pos_ver * pos_hor);
}
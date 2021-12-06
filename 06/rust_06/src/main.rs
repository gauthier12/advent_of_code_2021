use std::env;
use std::fs;

fn main() {
    const NUM_DAY:u16 = 256;
    let mut population: [u128; 10]= [0; 10];
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let tab_content = contents.lines();
    for s in tab_content {
        let splitted_line = s.split_terminator(',');
        for ival in splitted_line
        {
            let read_age = ival.parse::<usize>().unwrap();
            population[read_age]+=1;
        }
    }
    for _i_day in 0..NUM_DAY
    {
        population[9] = population[0];
        for i_age in 0..9
        {
            population[i_age] = population[i_age + 1];
        }
        population[6] += population[9];
    }
    let mut pop_size :u128 = 0;
    for i_age in 0..9
    {
        pop_size += population[i_age];
    }
    println!("population size : {}", pop_size);
}
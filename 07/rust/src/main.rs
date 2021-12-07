use std::env;
use std::fs;
use std::cmp;

fn main() {
    let mut position_list: Vec<i32> = Vec::new();
    let mut consomption_cost: Vec<i32> = Vec::new();
    let mut max_position = 0;
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let tab_content = contents.lines();
    for s in tab_content {
        let splitted_line = s.split_terminator(',');
        for ival in splitted_line
        {
            let read_position = ival.parse::<i32>().unwrap();
            position_list.push(read_position);
            max_position = cmp::max(max_position,read_position);
        }
    }

    consomption_cost.resize((max_position as usize)+1,0);
    consomption_cost[0]=0;
    for i_dis in 1..(max_position+1) as usize
    {
        //pb_a
        //consomption_cost[i_dis] = i_dis as i32;
        //pb_b
        consomption_cost[i_dis] = consomption_cost[i_dis-1]+i_dis as i32;
    }
    let mut min_pos  =  i32::MAX;
    let mut  min_fuel =  i32::MAX;
    for i_test in 0..(max_position+1)
    {
        let mut fuel = 0;
        for i_pos in position_list.iter()
        {
            let dis = (*i_pos - i_test).abs();
            fuel += consomption_cost[dis as usize];
        }
        if fuel<min_fuel 
        {
            min_pos = i_test;
            min_fuel = fuel;
        }
    }
    println!("optimal position : {}",min_pos);
    println!("consumption      : {}", min_fuel);
}
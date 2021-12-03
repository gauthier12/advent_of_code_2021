use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    const LINE_SIZE : usize =12;
    let mut num1: [i32; LINE_SIZE] = [0; LINE_SIZE];
    let mut tot:i32 = 0; 

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let tab_content = contents.chars();
    let mut ichar =0 ;
    for s in tab_content {
        match s{
            '0' =>
            {
                ichar +=1;
            },
            '1' =>
            {
                num1[ichar]+=1;
                ichar +=1;
            },
            tmp if tmp.is_ascii_whitespace() =>
            {
                ichar=0;
                tot+=1;
            },
            _=>println!("wrong command"),
        };
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for item in num1 {
        gamma*=2;
        epsilon*=2;
        if item > tot / 2 
        {
            gamma+=1
        }
        else
        {
            epsilon+=1
        }
    }
    let lines: Vec<_>  = contents.lines().collect();
    let mut line_list: Vec<usize> = (0..tot as usize).collect();
    line_list.swap_remove(5);
    println!("power consumption {}", gamma * epsilon);
    for ele in line_list
    {
        println!("{}",lines[ele]);
    }

}
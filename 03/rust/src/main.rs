use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    const LINE_SIZE: usize = 12;
    let mut num1: [i32; LINE_SIZE] = [0; LINE_SIZE];
    let mut tot: i32 = 0;

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let tab_content = contents.chars();
    let mut ichar = 0;
    for s in tab_content {
        match s {
            '0' => {
                ichar += 1;
            }
            '1' => {
                num1[ichar] += 1;
                ichar += 1;
            }
            tmp if tmp.is_ascii_whitespace() => {
                ichar = 0;
                tot += 1;
            }
            _ => panic!("wrong command"),
        };
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for item in num1 {
        gamma *= 2;
        epsilon *= 2;
        if item > tot / 2 {
            gamma += 1
        } else {
            epsilon += 1
        }
    }
    println!("power consumption : {}", gamma * epsilon);

    let lines: Vec<_> = contents.lines().collect();

    let mut oxygen_list: Vec<usize> = (0..tot as usize).collect();
    for i_digit in 0..LINE_SIZE {
        let mut numl0 = 0;
        let mut numl1 = 0;
        for ele in oxygen_list.iter() {
            if lines[*ele].chars().nth(i_digit).unwrap() == '0' {
                numl0 += 1
            } else {
                numl1 += 1
            }
        }
        let common = if numl1 < numl0 { '0' } else { '1' };
        //println!("Digit {} : most common {}+{}={}  ==> {}", i_digit,numl0, numl1,numl0+numl1, common);
        let mut new_oxygen_list: Vec<usize> = Vec::new();
        for ele in oxygen_list.iter() {
            if lines[*ele].chars().nth(i_digit).unwrap() == common {
                new_oxygen_list.push(*ele)
            }
        }
        oxygen_list.clear();
        oxygen_list.resize(new_oxygen_list.len(), 0);
        oxygen_list.copy_from_slice(&new_oxygen_list);
        if oxygen_list.len() == 1 {
            break;
        }
    }
    assert_eq!(oxygen_list.len(),1,"Problem : multiple choice in oxygen list");
    let mut oxygen_rating = 0;
    for i_char in lines[oxygen_list[0]].chars() {
        oxygen_rating *= 2;
        if i_char == '1' {
            oxygen_rating += 1
        }
    }

    let mut co2_list: Vec<usize> = (0..tot as usize).collect();
    for i_digit in 0..LINE_SIZE {
        let mut numl0 = 0;
        let mut numl1 = 0;
        for ele in co2_list.iter() {
            if lines[*ele].chars().nth(i_digit).unwrap() == '0' {
                numl0 += 1
            } else {
                numl1 += 1
            }
        }
        let common = if numl1 < numl0 { '0' } else { '1' };
        //println!("Digit {} : most common {}+{}={}  ==> {}", i_digit,numl0, numl1,numl0+numl1, common);
        let mut new_co2_list: Vec<usize> = Vec::new();
        for ele in co2_list.iter() {
            if lines[*ele].chars().nth(i_digit).unwrap() != common {
                new_co2_list.push(*ele)
            }
        }
        co2_list.clear();
        co2_list.resize(new_co2_list.len(), 0);
        co2_list.copy_from_slice(&new_co2_list);
        if co2_list.len() == 1 {
            break;
        }
    }
    assert_eq!(co2_list.len(), 1, "Problem : multiple choice in co2 list");
    let mut co2_rating = 0;
    for i_char in lines[co2_list[0]].chars() {
        co2_rating *= 2;
        if i_char == '1' {
            co2_rating += 1
        }
    }

    println!("life support rating : {}", co2_rating * oxygen_rating);
}

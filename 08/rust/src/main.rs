use std::env;
use std::fs;
fn common(word_a: &String, word_b: &String) -> i32 {
    let mut tot = 0;
    for letter_a in (*word_a).chars() {
        for letter_b in (*word_b).chars() {
            if letter_a == letter_b {
                tot += 1;
            }
        }
    }
    return tot;
}
fn main() {
    type WordList = Vec<String>;
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut total_value = 0;
    let mut number_1478 = 0;
    for s in contents.lines() {
        let mut number_list: WordList = Vec::new();
        if let Some(part) = s.split_terminator('|').nth(0) {
            // construct the number dictionnary
            let splitted_part = part.split_whitespace();
            for iword in splitted_part {
                number_list.push(String::from(iword));
            }
        }
        //search the 1
        for iword in 0..10 {
            if number_list[iword].len() == 2 {
                number_list.swap(1, iword);
                break;
            }
        }
        //search the 4
        for iword in [0, 2, 3, 4, 5, 6, 7, 8, 9] {
            if number_list[iword].len() == 4 {
                number_list.swap(4, iword);
                break;
            }
        }
        //search the 7
        for iword in [0, 2, 3, 5, 6, 7, 8, 9] {
            if number_list[iword].len() == 3 {
                number_list.swap(7, iword);
                break;
            }
        }
        //search the 8
        for iword in [0, 2, 3, 5, 6, 8, 9] {
            if number_list[iword].len() == 7 {
                number_list.swap(8, iword);
                break;
            }
        }
        //search the 6
        for iword in [0, 2, 3, 5, 6, 9] {
            if number_list[iword].len() == 6 {
                if common(&number_list[iword], &number_list[1]) == 1 {
                    number_list.swap(6, iword);
                    break;
                }
            }
        }
        //search the 9
        for iword in [0, 2, 3, 5, 9] {
            if number_list[iword].len() == 6 {
                if common(&number_list[iword], &number_list[4]) == 4 {
                    number_list.swap(9, iword);
                    break;
                }
            }
        }
        //search the 0
        for iword in [0, 2, 3, 5] {
            if number_list[iword].len() == 6 {
                number_list.swap(0, iword);
                break;
            }
        }
        //search the 3
        for iword in [2, 3, 5] {
            if common(&number_list[iword], &number_list[1]) == 2 {
                number_list.swap(3, iword);
                break;
            }
        }
        //search the 5
        for iword in [2, 5] {
            if common(&number_list[iword], &number_list[4]) == 3 {
                number_list.swap(5, iword);
                break;
            }
        }
        //Verify the 2
        assert_eq!(
            common(&number_list[2], &number_list[4]),
            2,
            "Error in the last match"
        );

        // construct the digit
        if let Some(part) = s.split_terminator('|').nth(1) {
            let mut digit_value = 0;
            let splitted_part = part.split_whitespace();
            let mut multiplier = 1000;
            for iword in splitted_part {
                match iword.len() {
                    2 => {
                        number_1478 += 1;
                        digit_value += multiplier;
                    }
                    3 => {
                        number_1478 += 1;
                        digit_value += 7 * multiplier;
                    }
                    4 => {
                        number_1478 += 1;
                        digit_value += 4 * multiplier;
                    }
                    5 => {
                        for itest in [2, 3, 5] {
                            if common(&number_list[itest], &String::from(iword)) == 5 {
                                digit_value += (itest as i32) * multiplier;
                            }
                        }
                    }
                    6 => {
                        for itest in [0, 6, 9] {
                            if common(&number_list[itest], &String::from(iword)) == 6 {
                                digit_value += (itest as i32) * multiplier;
                            }
                        }
                    }
                    7 => {
                        number_1478 += 1;
                        digit_value += 8 * multiplier;
                    }
                    _ => {}
                }
                multiplier /= 10;
            }
            println!("{} is {} ", part, digit_value);
            total_value += digit_value;
        }
    }
    println!("Number of 1 4 7 8 : {}", number_1478);
    println!("Total value : {}", total_value);
}

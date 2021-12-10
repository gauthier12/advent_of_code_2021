use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut syntax_score = 0;
    let mut line_autocomplete_score : Vec<u64> = Vec::new();
    'lineloop:for s in contents.lines() {
        let mut pile : Vec<char> = Vec::new();
        println!("===============================================");
        println!("line : {}", s);
        for c in s.chars()
        {
            match c
            {
                '('=>
                {
                    pile.push('(');
                },
                ')'=>
                {
                    if let Some(top) = pile.pop()
                    {
                        if top!='(' 
                        {
                            println!("   error popping {} for {}", top , c);
                            syntax_score += 3;
                            continue 'lineloop;
                        }
                    }
                },
                '<'=>
                {
                    pile.push('<');
                },
                '>'=>
                {
                    if let Some(top) = pile.pop()
                    {
                        if top!='<' 
                        {
                            println!("   error popping {} for {}", top , c);
                            syntax_score += 25137;
                            continue 'lineloop;
                        }
                    }
                },
                '['=>
                {
                    pile.push('[');
                },
                ']'=>
                {
                    if let Some(top) = pile.pop()
                    {
                        if top!='[' 
                        {
                            println!("   error popping {} for {}", top , c);
                            syntax_score += 57;
                            continue 'lineloop;
                        }
                    }
                },
                '{'=>
                {
                    pile.push('{');
                },
                '}'=>
                {
                    if let Some(top) = pile.pop()
                    {
                        if top!='{' 
                        {
                            println!("   error popping {} for {}", top , c);
                            syntax_score += 1197;
                            continue 'lineloop;
                        }
                    }
                },
                _ =>
                {
                    panic!("Wrong character");
                }
            }

        print!("   {} : ",c);
        for c2 in pile.iter()
        {
            print!("{}", c2);
        }
        println!("");
        }
        println!("   remaining pile size : {}", pile.len());
        let mut line_a_score = 0;
        while let Some(c) = pile.pop()
        {
            line_a_score *=5;
            match c
            {
                '('=>
                {
                    line_a_score +=1;
                },
                '<'=>
                {
                    line_a_score +=4;
                },
                '['=>
                {
                    line_a_score +=2;
                },
                '{'=>
                {
                    line_a_score +=3;
                },
                _ =>
                {
                    panic!("Wrong character");
                }
            }
        }
        println!("   line autocomplete score : {}", line_a_score);
        line_autocomplete_score.push(line_a_score);
    }
    println!("===============================================");
    println!("Syntax score : {}", syntax_score);
    line_autocomplete_score.sort();
    println!("Autocomplete score : {}", line_autocomplete_score[(line_autocomplete_score.len()-1)/2]);
}

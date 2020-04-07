use std::io;

use lox::tokenizer::{Token, tokenize};

// fn run_prompt() -> io::Result<()> {
//     use std::io::{stdin, stdout, Write};
// 
//     loop {
//         let mut s = String::new();
//         print!("> ");
//         stdout().flush()?;
//         stdin().read_line(&mut s).expect("HFE");
//         let s = s;
//         if !run(&s) {
//             break;
//         }
//         print!("{}", s);
//     }
//     Ok(())
// }

fn run(s: &str) -> bool {
    let tokens = tokenize(&s);
    for token in &tokens {
        print!(" {}", token);
        match *token {
            Token::Semicolon | Token::Comment(_) => println!(),
            _ => (),
        }
    }
    println!();
    for token in &tokens {
        print!(" {:?}", token);
        match *token {
            Token::Semicolon | Token::Comment(_) => println!(),
            _ => (),
        }
    }
    println!();
    !tokens.is_empty()
}

fn main() -> io::Result<()> {
    // run_prompt()
    //let s = "
    //    var pi = 3.14;
    //    // this is a comment
    //    var d = 3.0;
    //    var x = pi * d;
    //    var c = d <= pi;
    //    var e = d != pi;
    //".to_string();

    let path = "/home/sparlock/workspace/lox_example/examples/fib.lox";
    let s = std::fs::read_to_string(path).unwrap();

    //let s = "69.0".to_string();
    let ok = run(&s);
    assert!(ok);
    Ok(())
}

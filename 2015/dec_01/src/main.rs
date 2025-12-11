use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Start program with a String Argument, ex: $cargo run --\"()()\"");
        process::exit(0);
    }
    let mut instruction: String = args[1].clone().chars().rev().collect();
    let mut floor_level: i16 = 0;
    let mut step = 0;

    while let Some(c) = instruction.pop() {
        match c {
            '(' => floor_level += 1,
            ')' => floor_level -= 1,
            _ => (),
        }
        step += 1;

        if floor_level < 0 {
            break;
        }
    }

    println!("santa got to the basement at step: {}", step);
}

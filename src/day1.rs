use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

pub(crate) fn sum_2020() {
    let input: Vec<i32> = fetch_input();
    let mut values: HashSet<i32> = HashSet::new();
    for num in &input {
        if values.contains(num) {
            println!("{}", num * (2020 - *num))
        }
        values.insert(2020 - *num);
    }
}

fn fetch_input() -> Vec<i32> {
    let mut input: Vec<i32> = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(s) => {
                if &s == "go" {
                    break;
                }
                match s.parse::<i32>() {
                    Ok(num) => input.push(num),
                    Err(e) => println!("{}", e.to_string()),
                }
            }
            Err(e) => println!("{}", e.to_string()),
        }
    }
    input
}

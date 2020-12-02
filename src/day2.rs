use std::io;
use std::io::BufRead;

pub(crate) fn valid_passwords() {
    let input = fetch_input();
    let mut output1: i32 = 0;
    let mut output2: i32 = 0;
    for i in &input {
        if is_correct_day1(i) {
            output1 += 1;
        }
        if is_correct_day2(i) {
            output2 += 1;
        }
    }
    println!(
        "[day1] correct passwords: {}\n[day2] correct passwords: {}",
        output1, output2
    )
}

fn fetch_input() -> Vec<String> {
    let mut input: Vec<String> = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(s) => {
                if &s == "go" {
                    break;
                }
                input.push(s)
            }
            Err(e) => println!("{}", e.to_string()),
        }
    }
    input
}

fn is_correct_day1(pass: &str) -> bool {
    let parts = pass.split(' ').collect::<Vec<&str>>();
    let (i, j): (i32, i32) = fetch_numbers(parts[0]);
    let key: char = fetch_key(parts[1]);
    let password: &str = parts[2];
    let mut check: i32 = 0;
    for c in password.chars() {
        if c == key {
            check += 1;
        }
    }
    check <= j && check >= i
}

fn is_correct_day2(pass: &str) -> bool {
    let parts = pass.split(' ').collect::<Vec<&str>>();
    let (i, j): (i32, i32) = fetch_numbers(parts[0]);
    let key: char = fetch_key(parts[1]);
    let password: &str = parts[2];
    let mut answer: bool = false;
    for (idx, c) in password.chars().enumerate() {
        if idx + 1 == i as usize {
            answer ^= c == key;
        }
        if idx + 1 == j as usize {
            answer ^= c == key;
        }
    }
    answer
}

fn fetch_numbers(nums: &str) -> (i32, i32) {
    let numbers: Vec<&str> = nums.split('-').collect();
    (numbers[0].parse().unwrap(), numbers[1].parse().unwrap())
}

fn fetch_key(k: &str) -> char {
    k.chars().next().unwrap()
}

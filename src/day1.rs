use std::collections::HashSet;
use std::io;
use std::io::BufRead;

pub(crate) fn sum_2020() {
    let input = fetch_input();
    let d: Vec<i32> = find_doubles(2020, input.clone());
    println!("product of double {}", d[0] * d[1]);
    let diffs: Vec<i32> = input.iter().map(|x| 2020 - *x).collect();
    for dif in &diffs {
        let doub = find_doubles(*dif, input.clone());
        if !doub.is_empty() {
            println!("product of triplet {}", doub[0] * doub[1] * (2020 - *dif));
            break;
        }
    }
}

fn find_doubles(n: i32, input: Vec<i32>) -> Vec<i32> {
    let mut doubles: Vec<i32> = vec![];
    let mut values: HashSet<i32> = HashSet::new();
    for num in &input {
        if values.contains(num) {
            doubles.push(*num);
            doubles.push(n - *num);
        }
        values.insert(n - *num);
    }
    doubles
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

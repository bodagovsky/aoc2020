use crate::day2;

pub(crate) fn way_down_we_go() {
    let input: Vec<String> = day2::fetch_input();
    let trees = count_trees(3, 1, input.clone());
    let trees1 = count_trees(1, 1, input.clone());
    let trees2 = count_trees(5, 1, input.clone());
    let trees3 = count_trees(7, 1, input.clone());
    let trees4 = count_trees(1, 2, input.clone());
    println!("[day1] trees: {}", trees);
    println!(
        "[day2] trees: {}",
        trees * trees1 * trees2 * trees3 * trees4
    );
}

fn count_trees(mut right: usize, mut down: usize, forest: Vec<String>) -> i64 {
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut trees = 0;
    while row < forest.len() {
        if forest[row].chars().nth(col).unwrap() == '#' {
            trees += 1;
        }
        row += down;
        col += right;
        while col >= forest[0].len() {
            col -= forest[0].len()
        }
    }
    trees
}

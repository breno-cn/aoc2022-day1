use std::fs;

fn parse_input(filepath: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(filepath)
        .unwrap()
        .split("\n\n")
        .map(|row| row.split("\n").collect::<Vec<&str>>())
        .map(|row| {
            row.iter()
                .map(|cal| cal.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn solve(calories: &Vec<Vec<i32>>) -> i32 {
    calories.iter()
        .map(|row| row.iter().sum())
        .max()
        .unwrap()
}

fn solve_part2(calories: &Vec<Vec<i32>>) -> i32 {
    let mut calories = calories.iter()
        .map(|row| row.iter().sum())
        .collect::<Vec<i32>>();
    
    calories.sort_by(|a, b| b.cmp(a));

    calories.iter().take(3).sum()
}

fn main() {
    let input = parse_input("input.txt");

    let result = solve(&input);
    println!("{:}", result);

    let result = solve_part2(&input);
    println!("{:}", result)
}

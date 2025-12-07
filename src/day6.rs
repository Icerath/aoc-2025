const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

enum Operator {
    Add,
    Mul,
}

fn part1(input: &str) -> u64 {
    let (numbers, operators) = input.rsplit_once("\n").unwrap();
    let numbers: Vec<u64> = numbers
        .lines()
        .flat_map(|line| line.split_whitespace().map(|n| n.parse::<u64>().unwrap()))
        .collect();

    let operators: Vec<Operator> = operators
        .split_whitespace()
        .map(|s| if s == "*" { Operator::Mul } else { Operator::Add })
        .collect();

    let num_rows = numbers.len() / operators.len();
    let row_len = operators.len();

    let mut sum = 0;
    for (col, operator) in operators.iter().enumerate() {
        let iter = (col..row_len * num_rows).step_by(row_len).map(|i| numbers[i]);
        sum += match operator {
            Operator::Add => iter.sum::<u64>(),
            Operator::Mul => iter.product::<u64>(),
        }
    }
    sum
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE), 4277556);
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day6.txt")), 5060053676136);
}

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

fn part2(input: &str) -> u64 {
    let (numbers, operators) = input.rsplit_once("\n").unwrap();

    let operators =
        operators.split_whitespace().map(|s| if s == "*" { Operator::Mul } else { Operator::Add });

    let lines: Vec<_> = numbers.lines().collect();
    assert_eq!(lines.iter().map(|s| s.len()).min(), lines.iter().map(|s| s.len()).max());

    let mut current_pos = 0;
    let mut cols: Vec<Vec<_>> = vec![];
    while lines[0].len() > current_pos {
        let largest = (lines.iter().map(|s| &s[current_pos..]))
            .map(|s| s.find(' ').unwrap_or(s.len()))
            .max()
            .unwrap();
        cols.push(lines.iter().map(|s| s[current_pos..current_pos + largest].as_bytes()).collect());
        current_pos += largest + 1;
    }

    let mut sum = 0;
    for (col, operator) in cols.iter().zip(operators) {
        let iter = (0..col[0].len()).map(|digit| {
            col.iter()
                .flat_map(|s| (s[digit] != b' ').then(|| (s[digit] - b'0') as u64))
                .fold(0, |acc, x| acc * 10 + x)
        });
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

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE), 3263827);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day6.txt")), 9695042567249);
}

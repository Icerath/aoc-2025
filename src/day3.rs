const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

fn part1(input: &str) -> u32 {
    input.lines().map(highest_in_bank).sum()
}

fn highest_in_bank(bank: &str) -> u32 {
    let mut max = 0;
    for start in 0..(bank.len() - 1) {
        let a = (bank.as_bytes()[start] - b'0') as u32 * 10;
        let b = bank[(start + 1)..].bytes().map(|b| b - b'0').max().unwrap() as u32;
        max = (a + b).max(max)
    }
    max
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE), 357);
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day3.txt")), 17403);
}

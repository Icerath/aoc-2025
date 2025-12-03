use std::collections::HashMap;

use rustc_hash::FxHashMap;

const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

fn part1(input: &str) -> u64 {
    input.lines().map(highest_in_bank(2)).sum()
}

fn part2(input: &str) -> u64 {
    input.lines().map(highest_in_bank(12)).sum()
}

fn highest_in_bank(n: u32) -> impl Fn(&str) -> u64 {
    move |bank| highest_n_in_bank(bank, n, &mut HashMap::default())
}

fn highest_n_in_bank<'a>(
    bank: &'a str,
    n: u32,
    cache: &mut FxHashMap<(&'a [u8], u32), u64>,
) -> u64 {
    if let Some(cached) = cache.get(&(bank.as_bytes(), n)) {
        return *cached;
    }
    if n == 0 {
        return 0;
    }
    let mut max = 0;
    for start in 0..(bank.len() - (n - 1) as usize) {
        let a = (bank.as_bytes()[start] - b'0') as u64 * 10u64.pow(n - 1);
        let b = highest_n_in_bank(&bank[start + 1..], n - 1, cache);
        max = (a + b).max(max)
    }
    cache.insert((bank.as_bytes(), n), max);
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

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE), 3121910778619);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day3.txt")), 173416889848394);
}

use std::ops::Range;

const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

fn part1(input: &str) -> u64 {
    let (fresh, ingredients) = input.split_once("\n\n").unwrap();

    let fresh: Vec<_> = fresh
        .lines()
        .map(|line| {
            let (lower, upper) = line.split_once('-').unwrap();
            lower.parse::<u64>().unwrap()..=upper.parse::<u64>().unwrap()
        })
        .collect();

    let ingredients = ingredients.lines().map(|ingredient| ingredient.parse::<u64>().unwrap());
    ingredients.filter(|id| fresh.iter().any(|range| range.contains(id))).count() as _
}

fn part2(input: &str) -> u64 {
    let (fresh, _) = input.split_once("\n\n").unwrap();

    let mut temp: Vec<Range<u64>> = vec![];
    let mut prev: Vec<Range<u64>> = vec![];
    for line in fresh.lines() {
        let (lower, upper) = line.split_once('-').unwrap();
        let range = lower.parse::<u64>().unwrap()..(upper.parse::<u64>().unwrap() + 1);
        temp.push(range);
    }
    loop {
        let len = temp.len();
        for range in temp.drain(..) {
            match prev.iter_mut().find(|prev| {
                (range.start < prev.end && range.end >= prev.start)
                    || (prev.start < range.end && prev.end >= range.start)
            }) {
                Some(prev) => {
                    prev.start = range.start.min(prev.start);
                    prev.end = range.end.max(prev.end);
                }
                None => prev.push(range.clone()),
            }
        }
        if prev.len() == len {
            break;
        }
        std::mem::swap(&mut temp, &mut prev);
    }
    prev.iter().map(|range| (range.start..range.end).count() as u64).sum()
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE), 3);
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day5.txt")), 674);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE), 14);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day5.txt")), 352509891817881);
}

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

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE), 3);
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day5.txt")), 674);
}

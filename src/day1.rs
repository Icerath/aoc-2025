const EXAMPLE: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

fn part1(input: &str) -> u32 {
    let mut current = 50;
    let mut sum = 0;
    for line in input.lines() {
        let (direction, distance) = line.split_at(1);
        let direction = if direction == "L" { -1 } else { 1 };
        let distance: i32 = distance.parse().unwrap();

        current += direction * distance;
        current = current.rem_euclid(100);
        sum += (current == 0) as u32;
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut current: i32 = 50;
    let mut sum = 0;
    for line in input.lines() {
        let (direction, distance) = line.split_at(1);
        let direction = if direction == "L" { -1 } else { 1 };
        let distance: i32 = distance.parse().unwrap();

        for _ in 0..distance {
            current += direction;
            current = current.rem_euclid(100);
            if current == 0 {
                sum += 1;
            }
        }
    }
    sum
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE), 3);
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day1.txt")), 1177);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE), 6);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day1.txt")), 6768);
}

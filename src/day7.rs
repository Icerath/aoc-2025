use std::collections::VecDeque;

const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

fn part1(input: &str) -> u32 {
    let mut input = input.to_string().into_bytes();
    let row_len = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let index = input.iter().position(|&b| b == b'S').unwrap();
    let mut queue = VecDeque::new();
    queue.push_back(index);
    let mut sum = 0;
    while let Some(current) = queue.pop_front() {
        let next = current + row_len;
        if next >= input.len() || input[next] == b'|' {
        } else if input[next] == b'^' {
            // there don't seem to be splitters at the edges
            queue.push_back(next + 1);
            queue.push_back(next - 1);
            input[next + 1] = b'|';
            input[next - 1] = b'|';
            sum += 1;
        } else {
            queue.push_back(next);
            input[next] = b'|';
        }
    }
    sum
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE), 21);
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day7.txt")), 1638);
}

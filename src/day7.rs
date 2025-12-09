use rustc_hash::FxHashMap;
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

fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let row_len = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let index = input.iter().position(|&b| b == b'S').unwrap();
    part2_inner(input, index, row_len, &mut FxHashMap::default()) + 1
}

fn part2_inner(
    grid: &[u8],
    current: usize,
    row_len: usize,
    cache: &mut FxHashMap<usize, u64>,
) -> u64 {
    if let Some(cached) = cache.get(&current) {
        return *cached;
    }
    let next = current + row_len;

    let mut sum = 0;
    if next >= grid.len() {
    } else if grid[next] == b'^' {
        sum += 1;
        sum += part2_inner(grid, next + 1, row_len, cache);
        sum += part2_inner(grid, next - 1, row_len, cache);
    } else {
        sum += part2_inner(grid, next, row_len, cache);
    }
    cache.insert(current, sum);
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

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE), 40);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day7.txt")), 7759107121385);
}

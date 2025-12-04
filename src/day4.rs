const EXAMPLE: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

const OFFSETS: [(isize, isize); 8] =
    [(0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 0), (1, 1)];

fn part1(input: &str) -> u32 {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();

    let width = grid[0].len();
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..width {
            if grid[y][x] == b'@' {
                let adj_papers = OFFSETS
                    .into_iter()
                    .filter(|&(ox, oy)| index_offset(&grid, [x, y], [ox, oy]) == Some(b'@'))
                    .count();
                sum += (adj_papers < 4) as u32;
            }
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut input = input.to_string().into_bytes();
    let mut grid: Vec<&mut [u8]> = input.split_mut(|b| *b == b'\n').collect();

    let width = grid[0].len();
    let mut total = 0;
    loop {
        let mut sum = 0;
        for y in 0..grid.len() {
            for x in 0..width {
                if grid[y][x] == b'@' {
                    let adj_papers = OFFSETS
                        .into_iter()
                        .filter(|&(ox, oy)| index_offset_mut(&grid, [x, y], [ox, oy]) == Some(b'@'))
                        .count();
                    if adj_papers < 4 {
                        grid[y][x] = b'.';
                        sum += 1;
                    }
                }
            }
        }
        if sum == 0 {
            break total;
        }
        total += sum;
    }
}

fn index_offset(grid: &[&[u8]], [x, y]: [usize; 2], [ox, oy]: [isize; 2]) -> Option<u8> {
    grid.get(y.wrapping_add_signed(oy)).and_then(|row| row.get(x.wrapping_add_signed(ox))).copied()
}

fn index_offset_mut(grid: &[&mut [u8]], [x, y]: [usize; 2], [ox, oy]: [isize; 2]) -> Option<u8> {
    grid.get(y.wrapping_add_signed(oy)).and_then(|row| row.get(x.wrapping_add_signed(ox))).copied()
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE), 13);
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day4.txt")), 1367);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE), 43);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day4.txt")), 9144);
}

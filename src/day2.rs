const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for range in input.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let range = start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap();
        sum += range.filter(|&int| is_invalid1(int)).sum::<u64>()
    }
    sum
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for range in input.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let range = start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap();
        sum += range.filter(|&int| is_invalid2(int)).sum::<u64>()
    }
    sum
}

fn is_invalid1(id: u64) -> bool {
    let id = id.to_string();
    let split_size = id.len() / 2;
    id[..split_size] == id[split_size..]
}

fn is_invalid2(id: u64) -> bool {
    let id = id.to_string();

    for splits in 2..=id.len() {
        if id.len() % splits != 0 {
            continue;
        }
        let split_size = id.len() / splits;

        let mut iter = (0..splits).map(|i| &id[i * split_size..(i + 1) * split_size]);
        let first = iter.next().unwrap();
        if iter.all(|item| item == first) {
            return true;
        }
    }
    false
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE), 1227775554);
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../input/day2.txt")), 40055209690);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE), 4174379265);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../input/day2.txt")), 50857215650);
}

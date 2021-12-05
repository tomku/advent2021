extern crate nom;

use std::cmp::{max, min};
use std::collections::HashMap;

type Point = (i32, i32);
type Line = (Point, Point);

pub fn record_lines(lines: &Vec<Line>) -> HashMap<Point, i32> {
    let mut map = HashMap::new();
    for &((x1, y1), (x2, y2)) in lines {
        if x1 == x2 {
            for y in min(y1, y2)..max(y1, y2) + 1 {
                *map.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            for x in min(x1, x2)..max(x1, x2) + 1 {
                *map.entry((x, y1)).or_insert(0) += 1;
            }
        } else {
            let dx = (x1 < x2) as i32 * 2 - 1;
            let dy = (y1 < y2) as i32 * 2 - 1;

            let mut x = x1;
            let mut y = y1;

            loop {
                *map.entry((x, y)).or_insert(0) += 1;
                if x == x2 || y == y2 { break; }
                x += dx;
                y += dy;
            }
        }
    }
    map
}

fn without_diagonals(lines: &Vec<Line>) -> Vec<Line> {
    lines.into_iter()
        .filter(|((x1, y1), (x2, y2))| (x1 == x2) || (y1 == y2))
        .copied()
        .collect()
}

fn overlaps(map: &HashMap<(i32, i32), i32>) -> usize {
    map.values().filter(|&n| *n > 1).count()
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{i32 as num, newline};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;

    use super::*;

    fn coord(input: &str) -> IResult<&str, (i32, i32)> {
        separated_pair(num, tag(","), num)(input)
    }

    fn line(input: &str) -> IResult<&str, (Point, Point)> {
        separated_pair(coord, tag(" -> "), coord)(input)
    }

    pub(crate) fn lines(input: &str) -> IResult<&str, Vec<Line>> {
        separated_list1(newline, line)(input)
    }
}

#[cfg(test)]
mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

    #[test]
    fn test_parse() {
        let (_, lines) = parse::lines(TEST_INPUT).unwrap();
        assert_eq!(*lines.last().unwrap(), ((5, 5), (8, 2)));
        assert_eq!(*lines.first().unwrap(), ((0, 9), (5, 9)));
    }

    #[test]
    fn test_overlapping_cardinals() {
        let (_, lines) = parse::lines(TEST_INPUT).unwrap();
        let cardinals = without_diagonals(&lines);
        let map = record_lines(&cardinals);

        assert_eq!(overlaps(&map), 5);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("05");
        let (_, lines) = parse::lines(&input).unwrap();
        let cardinals: Vec<Line> = without_diagonals(&lines);
        let map = record_lines(&cardinals);

        assert_eq!(overlaps(&map), 3990);
    }

    #[test]
    fn test_overlapping() {
        let (_, lines) = parse::lines(TEST_INPUT).unwrap();
        let map = record_lines(&lines);

        assert_eq!(overlaps(&map), 12);
    }

    #[test]
    fn part2() {
        let input = puzzle_input("05");
        let (_, lines) = parse::lines(&input).unwrap();
        let map = record_lines(&lines);

        assert_eq!(overlaps(&map), 21305);
    }
}
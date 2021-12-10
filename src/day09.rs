use std::collections::{HashMap, HashSet};

fn low_points(lava_map: &HashMap<(i32, i32), u32>, (cols, rows): (usize, usize)) -> Vec<(i32, i32)> {
    let mut lows = Vec::new();
    for y in 0..rows as i32 {
        for x in 0..cols as i32 {
            let current = *lava_map.get(&(x, y)).unwrap();
            let above = *lava_map.get(&(x, y - 1)).unwrap_or(&9);
            if current >= above { continue; }
            let left = *lava_map.get(&(x - 1, y)).unwrap_or(&9);
            if current >= left { continue; }
            let right = *lava_map.get(&(x + 1, y)).unwrap_or(&9);
            if current >= right { continue; }
            let below = *lava_map.get(&(x, y + 1)).unwrap_or(&9);
            if current >= below { continue; }
            lows.push((x, y));
        }
    }
    lows
}

fn risk_total(lava_map: &HashMap<(i32, i32), u32>, dims: (usize, usize)) -> u32 {
    let lowest = low_points(&lava_map, dims);
    lowest.iter().map(|pos| {
        lava_map.get(pos).unwrap() + 1
    }).sum()
}

fn reachable_from(lava_map: &HashMap<(i32, i32), u32>, start: (i32, i32)) -> u32 {
    let mut stack = vec![start];
    let mut seen = HashSet::new();
    let mut total = 0;

    loop {
        if let Some(current) = stack.pop() {
            let (x, y) = current;
            if seen.contains(&current) { continue; }
            seen.insert(current);

            let h = *lava_map.get(&current).unwrap_or(&9);
            if h >= 9 { continue; }

            total += 1;
            stack.push((x + 1, y));
            stack.push((x - 1, y));
            stack.push((x, y - 1));
            stack.push((x, y + 1));
        } else {
            break total;
        }
    }
}

fn top3_basins(lava_map: &HashMap<(i32, i32), u32>, dims: (usize, usize)) -> u32 {
    let lowest = low_points(&lava_map, dims);
    let mut basins: Vec<u32> = lowest.iter().map(|&pos|
        reachable_from(&lava_map, pos)
    ).collect();
    basins.sort_by_key(|&n| std::cmp::Reverse(n));
    basins.iter().take(3).product()
}

mod parse {
    use nom::character::complete::{newline, one_of};
    use nom::combinator::map;
    use nom::IResult;
    use nom::multi::{many1, separated_list1};

    use super::*;

    fn row(input: &str) -> IResult<&str, Vec<u32>> {
        many1(map(one_of("0123456789"), |c: char| c.to_digit(10).unwrap()))(input)
    }

    pub(crate) fn lava_map(input: &str) -> IResult<&str, ((usize, usize), HashMap<(i32, i32), u32>)> {
        map(separated_list1(newline, row),
            |vecs| {
                let (cols, rows) = (vecs[0].len(), vecs.len());
                let mut map = HashMap::with_capacity(rows * cols);
                for (y, row) in vecs.iter().enumerate() {
                    for (x, item) in row.iter().enumerate() {
                        map.insert((x as i32, y as i32), *item);
                    }
                }
                ((cols, rows), map)
            })(input)
    }
}

mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";

    #[test]
    fn test_parse() {
        let (_, ((_, _), lava_map)) = parse::lava_map(TEST_INPUT).unwrap();
        assert_eq!(*lava_map.get(&(0, 0)).unwrap(), 2);
        assert_eq!(*lava_map.get(&(0, 4)).unwrap(), 9);
        assert_eq!(*lava_map.get(&(9, 0)).unwrap(), 0);
        assert_eq!(*lava_map.get(&(9, 4)).unwrap(), 8);
    }

    #[test]
    fn test_risk_total() {
        let (_, (dims, lava_map)) = parse::lava_map(TEST_INPUT).unwrap();
        assert_eq!(risk_total(&lava_map, dims), 15)
    }

    #[test]
    fn part1() {
        let input = puzzle_input("09");
        let (_, (dims, lava_map)) = parse::lava_map(&input).unwrap();
        assert_eq!(risk_total(&lava_map, dims), 566)
    }

    #[test]
    fn test_reachable_from() {
        let (_, (_, lava_map)) = parse::lava_map(TEST_INPUT).unwrap();
        assert_eq!(reachable_from(&lava_map, (1, 0)), 3);
        assert_eq!(reachable_from(&lava_map, (9, 0)), 9);
        assert_eq!(reachable_from(&lava_map, (2, 2)), 14);
        assert_eq!(reachable_from(&lava_map, (6, 4)), 9);
    }

    #[test]
    fn test_top3_basins() {
        let (_, (dims, lava_map)) = parse::lava_map(TEST_INPUT).unwrap();
        assert_eq!(top3_basins(&lava_map, dims), 1134)
    }

    #[test]
    fn part2() {
        let input = puzzle_input("09");
        let (_, (dims, lava_map)) = parse::lava_map(&input).unwrap();
        assert_eq!(top3_basins(&lava_map, dims), 891684)
    }
}
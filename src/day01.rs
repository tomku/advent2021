fn count_increases(depths: Vec<i32>) -> i32 {
    depths
        .windows(2)
        .fold(0, |acc, w| if w[1] > w[0] { acc + 1 } else { acc })
}

fn windowed_sum(nums: Vec<i32>, size: usize) -> Vec<i32> {
    nums.windows(size)
        .map(|nums| nums.iter().sum())
        .collect()
}

mod parse {
    use nom::character::complete::{digit1, newline};
    use nom::combinator::map_res;
    use nom::IResult;
    use nom::multi::separated_list1;

    pub fn digits_as_i32(input: &str) -> IResult<&str, i32> {
        map_res(digit1, |num| str::parse(num))(input)
    }

    pub(crate) fn depths(input: &str) -> IResult<&str, Vec<i32>> {
        separated_list1(newline, digits_as_i32)(input)
    }
}

mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_parse() {
        let (_, depth_list) = parse::depths(TEST_INPUT).unwrap();
        assert_eq!(depth_list, [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
    }

    #[test]
    fn test_increases() {
        let (_, depth_list) = parse::depths(TEST_INPUT).unwrap();
        assert_eq!(count_increases(depth_list), 7)
    }

    #[test]
    fn test_increases_windowed() {
        let (_, depth_list) = parse::depths(TEST_INPUT).unwrap();
        assert_eq!(count_increases(windowed_sum(depth_list, 3)), 5)
    }

    #[test]
    fn part1() {
        let input = puzzle_input("01");
        let (_, depth_list) = parse::depths(&input).unwrap();
        let increases = count_increases(depth_list);

        println!("Number of increases: {}", increases);
        assert_eq!(1752, increases);
    }

    #[test]
    fn part2() {
        let input = puzzle_input("01");
        let (_, depth_list) = parse::depths(&input).unwrap();
        let increases = count_increases(windowed_sum(depth_list, 3));

        println!("Number of increases (windowed): {}", increases);
        assert_eq!(1781, increases);
    }
}
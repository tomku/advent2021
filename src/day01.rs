extern crate nom;

#[cfg(test)]
mod day01 {
    use crate::util::puzzle_input;

    mod parse {
        use nom::{IResult, Parser};
        use nom::character::complete::{digit1, newline};
        use nom::multi::separated_list1;

        pub(crate) fn depths(input: &str) -> IResult<&str, Vec<i32>> {
            separated_list1(newline, digit1.map(|num| str::parse(num).unwrap()))(input)
        }
    }

    const TEST_INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_parse() {
        let (_, depth_list) = parse::depths(TEST_INPUT).unwrap();
        assert_eq!(depth_list, [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
    }

    fn count_increases(depths: Vec<i32>) -> i32 {
        depths
            .iter()
            .zip(depths.iter().skip(1))
            .fold(0, |acc, (prev, curr)| if curr > prev { acc + 1 } else { acc })
    }

    #[test]
    fn test_increases() {
        let (_, depth_list) = parse::depths(TEST_INPUT).unwrap();
        assert_eq!(count_increases(depth_list), 7)
    }

    fn windowed_sum(nums: Vec<i32>) -> Vec<i32> {
        nums
            .iter()
            .zip(nums.iter().skip(1))
            .zip(nums.iter().skip(2))
            .map(|((one, two), three)| one + two + three)
            .collect()
    }

    #[test]
    fn test_increases_windowed() {
        let (_, depth_list) = parse::depths(TEST_INPUT).unwrap();
        assert_eq!(count_increases(windowed_sum(depth_list)), 5)
    }

    #[test]
    fn part1_answer() {
        let input = puzzle_input("01");
        let (_, depth_list) = parse::depths(&input).unwrap();
        let increases = count_increases(depth_list);

        println!("Number of increases: {}", increases);
        assert_eq!(1752, increases);
    }

    #[test]
    fn part2_answer() {
        let input = puzzle_input("01");
        let (_, depth_list) = parse::depths(&input).unwrap();
        let increases = count_increases(windowed_sum(depth_list));

        println!("Number of increases (windowed): {}", increases);
        assert_eq!(1781, increases);
    }
}
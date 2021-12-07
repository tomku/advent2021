fn fish_freqs(fish_list: Vec<u64>) -> Vec<u64> {
    let mut freqs: Vec<u64> = [0].repeat(9);
    for fish in fish_list {
        freqs[fish as usize] += 1;
    }
    freqs
}

fn grow(freqs: &mut Vec<u64>) {
    freqs.rotate_left(1);
    freqs[6] += freqs[8];
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::u64 as num;
    use nom::IResult;
    use nom::multi::separated_list1;

    pub(crate) fn lanternfish_list(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list1(tag(","), num)(input)
    }
}

mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_parse() {
        let (_, fish) = parse::lanternfish_list(TEST_INPUT).unwrap();
        assert_eq!(fish, &[3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_growth() {
        let (_, fish) = parse::lanternfish_list(TEST_INPUT).unwrap();
        let mut freqs = fish_freqs(fish);
        for _ in 0..80 {
            grow(&mut freqs);
        }
        assert_eq!(freqs.iter().sum::<u64>(), 5934);
    }

    #[test]
    fn test_growth_big() {
        let (_, fish) = parse::lanternfish_list(TEST_INPUT).unwrap();
        let mut freqs = fish_freqs(fish);
        for _ in 0..256 {
            grow(&mut freqs);
        }
        assert_eq!(freqs.iter().sum::<u64>(), 26984457539);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("06");
        let (_, fish) = parse::lanternfish_list(&input).unwrap();
        let mut freqs = fish_freqs(fish);
        for _ in 0..80 {
            grow(&mut freqs);
        }
        assert_eq!(freqs.iter().sum::<u64>(), 387413);
    }

    #[test]
    fn part2() {
        let input = puzzle_input("06");
        let (_, fish) = parse::lanternfish_list(&input).unwrap();
        let mut freqs = fish_freqs(fish);
        for _ in 0..256 {
            grow(&mut freqs);
        }
        assert_eq!(freqs.iter().sum::<u64>(), 1738377086345);
    }
}
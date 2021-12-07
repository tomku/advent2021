mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::i64 as num;
    use nom::IResult;
    use nom::multi::separated_list1;

    pub(crate) fn crab_list(input: &str) -> IResult<&str, Vec<i64>> {
        separated_list1(tag(","), num)(input)
    }
}

mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    fn distances_linear(crabs: &Vec<i64>) -> Vec<i64> {
        let max = *crabs.iter().max().unwrap() as usize;
        let mut dists = Vec::with_capacity(max);
        for pos in 0..max {
            let mut total = 0;
            for c in crabs {
                total += (pos as i64 - c).abs()
            }
            dists.push(total);
        }
        dists
    }

    fn distances_nonlinear(crabs: &Vec<i64>) -> Vec<i64> {
        let max = *crabs.iter().max().unwrap() as usize;
        let mut dists = Vec::with_capacity(max);

        for pos in 0..max {
            let mut total = 0;
            for c in crabs {
                let n = (pos as i64 - c).abs();

                total += (n * (n + 1)) / 2
            }
            dists.push(total);
        }
        dists
    }

    #[test]
    fn test_parse() {
        let (_, fish) = parse::crab_list(TEST_INPUT).unwrap();
        assert_eq!(fish, &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn test_dists() {
        let (_, fish) = parse::crab_list(TEST_INPUT).unwrap();
        let d = *distances_linear(&fish).iter().min().unwrap();
        assert_eq!(d, 37);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("07");
        let (_, fish) = parse::crab_list(&input).unwrap();
        let d = *distances_linear(&fish).iter().min().unwrap();
        assert_eq!(d, 333755);
    }

    #[test]
    fn test_dist_nonlinear() {
        let (_, fish) = parse::crab_list(TEST_INPUT).unwrap();
        let d = *distances_nonlinear(&fish).iter().min().unwrap();
        assert_eq!(d, 168);
    }

    #[test]
    fn part2() {
        let input = puzzle_input("07");
        let (_, fish) = parse::crab_list(&input).unwrap();
        let d = *distances_nonlinear(&fish).iter().min().unwrap();
        assert_eq!(d, 94017638);
    }
}
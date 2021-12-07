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
        let mut current_dist: i64 = crabs.iter().sum();
        let mut i: usize = 0;

        for pos in 0..max {
            while i < crabs.len() && &crabs[i] <= &(pos as i64) { i += 1; }

            current_dist += 2 * i as i64 - crabs.len() as i64;
            dists.push(current_dist);
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
        let (_, mut fish) = parse::crab_list(TEST_INPUT).unwrap();
        fish.sort();
        let d = *distances_linear(&fish).iter().min().unwrap();
        assert_eq!(d, 37);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("07");
        let (_, mut fish) = parse::crab_list(&input).unwrap();
        fish.sort();
        let d = *distances_linear(&fish).iter().min().unwrap();
        assert_eq!(d, 333755);
    }

    #[test]
    fn test_dist_nonlinear() {
        let (_, mut fish) = parse::crab_list(TEST_INPUT).unwrap();
        fish.sort();
        let d = *distances_nonlinear(&fish).iter().min().unwrap();
        assert_eq!(d, 168);
    }

    #[test]
    fn part2() {
        let input = puzzle_input("07");
        let (_, mut fish) = parse::crab_list(&input).unwrap();
        fish.sort();
        let d = *distances_nonlinear(&fish).iter().min().unwrap();
        assert_eq!(d, 94017638);
    }
}
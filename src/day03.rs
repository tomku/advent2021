fn gamma_epsilon(input: &Vec<u32>) -> (u32, u32) {
    let digits = input.iter().max().unwrap().next_power_of_two().trailing_zeros();
    let bits = input.len() as u32;
    let mut gamma = 0;

    for i in 0..digits {
        let ones: u32 = input
            .iter()
            .map(|row| (u32::pow(2, i) & *row != 0) as u32)
            .sum();

        let d = (ones >= (bits / 2)) as u32;
        gamma = gamma | d << i;
    }
    let mask = gamma.next_power_of_two() - 1;
    (gamma, gamma ^ mask)
}

fn nth_bit_set(num: u32, b: u32) -> bool { (1 << b) & num != 0 }

fn rating(mut vecs: Vec<u32>, most_common: bool) -> u32 {
    let digits = vecs.iter().max().unwrap().next_power_of_two().trailing_zeros();
    for i in 0..digits {
        let b = digits - i - 1;

        let ones = vecs
            .iter()
            .filter(|&&x| nth_bit_set(x, b))
            .count();

        let zeroes = vecs.len() - ones;
        if ones >= zeroes {
            vecs = vecs.iter().filter(|&&x| most_common ^ !nth_bit_set(x, b)).copied().collect()
        } else if ones < zeroes {
            vecs = vecs.iter().filter(|&&x| most_common ^ nth_bit_set(x, b)).copied().collect()
        }

        if vecs.len() < 2 {
            break;
        }
    }
    vecs[0]
}

mod parse {
    use nom::character::complete::newline;
    use nom::character::complete::not_line_ending;
    use nom::combinator::map_res;
    use nom::IResult;
    use nom::multi::separated_list1;

    fn binary_number_as_u32(input: &str) -> IResult<&str, u32> {
        map_res(not_line_ending, |digits|
            u32::from_str_radix(digits, 2))(input)
    }

    pub(crate) fn vector_of_binary_u32s(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(
            newline,
            binary_number_as_u32,
        )(input)
    }
}


#[cfg(test)]
mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn test_parse() {
        let (_, input) = parse::vector_of_binary_u32s(&TEST_INPUT).unwrap();

        assert_eq!(TEST_INPUT,
                   input.iter()
                       .map(|row| format!("{:05b}", row))
                       .collect::<Vec<String>>()
                       .join("\n"));
    }

    #[test]
    fn test_gamma_epsilon() {
        let (_, input) = parse::vector_of_binary_u32s(&TEST_INPUT).unwrap();
        let (gamma, epsilon) = gamma_epsilon(&input);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("03");
        let (_, vecs) = parse::vector_of_binary_u32s(&input).unwrap();
        let (gamma, epsilon) = gamma_epsilon(&vecs);
        assert_eq!(1092896, gamma * epsilon);
    }

    #[test]
    fn test_oxygen_co2() {
        let input = TEST_INPUT;
        let (_, vecs) = parse::vector_of_binary_u32s(&input).unwrap();

        let oxygen = rating(vecs.clone(), true);
        let co2 = rating(vecs, false);
        assert_eq!(23, oxygen);
        assert_eq!(10, co2);
    }

    #[test]
    fn part2() {
        let input = puzzle_input("03");
        let (_, vecs) = parse::vector_of_binary_u32s(&input).unwrap();

        let oxygen = rating(vecs.clone(), true);
        let co2 = rating(vecs, false);
        assert_eq!(1357, co2);
        assert_eq!(3443, oxygen);
    }
}


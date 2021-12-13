use std::collections::HashMap;

pub struct Entry {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

impl Entry {
    fn count_unique_lengths(&self) -> usize {
        self.outputs.iter().filter(|p| match p.len() {
            2 => true,
            4 => true,
            3 => true,
            7 => true,
            _ => false
        }).count()
    }
    fn deduce(&self) -> HashMap<String, &str> {
        let one = self.patterns.iter().find(|p| p.len() == 2).unwrap();

        let one_first = one.chars().nth(0).unwrap();
        let one_second = one.chars().nth(1).unwrap();

        let seven = self.patterns.iter().find(|p| p.len() == 3).unwrap();
        let four = self.patterns.iter().find(|p| p.len() == 4).unwrap();
        let eight = self.patterns.iter().find(|p| p.len() == 7).unwrap();

        let six = self.patterns.iter().filter(|&p| p.len() == 6)
            .find(|&p| { !(p.contains(one_first) && p.contains(one_second)) }).unwrap();

        let upright = eight.chars().find(|p| !six.contains(*p)).unwrap();
        let three = self.patterns.iter().filter(|&p| p.len() == 5)
            .find(|&p| (p.contains(one_first) && p.contains(one_second))).unwrap();

        let two = self.patterns.iter().filter(|&p| p.len() == 5)
            .find(|&p| { p != three && p.contains(upright) }).unwrap();

        let five = self.patterns.iter().filter(|&p| p.len() == 5)
            .find(|&p| { p != three && !p.contains(upright) }).unwrap();

        let lowleft = two.chars().find(|&p| p != upright && !five.contains(p)).unwrap();
        let nine = eight.replace(lowleft, "");

        let zero = self.patterns.iter().find(|&p| p.len() == 6 && p != six && *p != nine).unwrap();

        HashMap::from([
            (zero.clone(), "0"),
            (one.clone(), "1"),
            (two.clone(), "2"),
            (three.clone(), "3"),
            (four.clone(), "4"),
            (five.clone(), "5"),
            (six.clone(), "6"),
            (seven.clone(), "7"),
            (eight.clone(), "8"),
            (nine, "9")
        ])
    }

    fn decode(&self, key: &HashMap<String, &str>) -> u32 {
        let digits: Vec<&str> = self.outputs.iter().map(|c| key.get(c).unwrap().to_owned()).collect();
        str::parse::<u32>(&digits.join("")).unwrap()
    }
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, newline};
    use nom::combinator::{map, recognize};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;

    use crate::day08::Entry;

    fn pattern(input: &str) -> IResult<&str, String> {
        map(recognize(alpha1), |str: &str| {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort();
            chars.into_iter().collect()
        })(input)
    }

    fn patterns(input: &str) -> IResult<&str, Vec<String>> {
        separated_list1(tag(" "), pattern)(input)
    }

    fn entry(input: &str) -> IResult<&str, Entry> {
        map(separated_pair(
            patterns, tag(" | "), patterns),
            |(pat, out)|
                Entry { patterns: pat, outputs: out })(input)
    }

    pub(crate) fn all_entries(input: &str) -> IResult<&str, Vec<Entry>> {
        separated_list1(newline, entry)(input)
    }
}

mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_parse() {
        let (_, entries) = parse::all_entries(TEST_INPUT).unwrap();
        assert_eq!(entries[9].patterns[0], "abcfg");
        assert_eq!(entries[9].outputs[3], "abceg");
    }

    #[test]
    fn test_unique_digit_count() {
        let (_, entries) = parse::all_entries(TEST_INPUT).unwrap();
        let uniques: usize = entries.iter().map(|e| e.count_unique_lengths()).sum();
        assert_eq!(uniques, 26);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("08");
        let (_, entries) = parse::all_entries(&input).unwrap();

        let uniques: usize = entries.iter().map(|e| e.count_unique_lengths()).sum();
        assert_eq!(uniques, 392);
    }

    #[test]
    fn test_deduce() {
        let (_, entries) = parse::all_entries(TEST_INPUT).unwrap();
        let total: u32 = entries.iter().map(|e| {
            let key = e.deduce();
            e.decode(&key)
        }).sum();
        assert_eq!(total, 61229)
    }

    #[test]
    fn part2() {
        let input = puzzle_input("08");
        let (_, entries) = parse::all_entries(&input).unwrap();
        let total: u32 = entries.iter().map(|e| {
            let key = e.deduce();
            e.decode(&key)
        }).sum();
        assert_eq!(total, 1004688)
    }
}
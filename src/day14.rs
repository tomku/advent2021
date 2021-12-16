use std::collections::HashMap;

fn steps(polymer: &str, count: u32, rules: &HashMap<(char, char), char>) -> HashMap<char, u64> {
    let mut pairs = HashMap::new();

    for i in 0..polymer.len() - 1 {
        let ch1 = polymer.chars().nth(i).unwrap();
        let ch2 = polymer.chars().nth(i + 1).unwrap();
        *pairs.entry((ch1, ch2)).or_insert(0) += 1;
    }

    for _ in 0..count {
        let mut new_queue = HashMap::new();
        for ((ch1, ch2), n) in pairs {
            if let Some(&new_ch) = rules.get(&(ch1, ch2)) {
                *new_queue.entry((ch1, new_ch)).or_insert(0) += n;
                *new_queue.entry((new_ch, ch2)).or_insert(0) += n;
            }
        }
        pairs = new_queue;
    }

    let mut freqs = HashMap::new();

    for ((ch, _), n) in pairs {
        *freqs.entry(ch).or_insert(0 as u64) += n;
    }

    *freqs.entry(polymer.chars().last().unwrap()).or_insert(0) += 1;
    freqs
}

mod parse {
    use std::collections::HashMap;

    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, anychar, newline};
    use nom::combinator::{map, opt};
    use nom::IResult;
    use nom::multi::fold_many1;
    use nom::sequence::{pair, separated_pair, terminated};

    fn template(input: &str) -> IResult<&str, String> {
        map(terminated(alpha1, newline), String::from)(input)
    }

    fn rules(input: &str) -> IResult<&str, HashMap<(char, char), char>> {
        fold_many1(
            terminated(separated_pair(pair(anychar, anychar),
                                      tag(" -> "),
                                      anychar), opt(newline)),
            HashMap::new,
            |mut acc, ((k1, k2), v)| {
                acc.insert((k1, k2), v);
                acc
            })(input)
    }

    pub(crate) fn polymer(input: &str) -> IResult<&str, (String, HashMap<(char, char), char>)> {
        separated_pair(template, newline, rules)(input)
    }
}

mod test {
    use crate::util::puzzle_input;

    use super::*;

    const EXAMPLE_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_parse() {
        let (_, (template, rules)) = parse::polymer(EXAMPLE_INPUT).unwrap();
        assert_eq!(template, "NNCB");
        assert_eq!(rules[&('C', 'N')], 'C');
        assert_eq!(rules[&('C', 'H')], 'B');
    }

    #[test]
    fn test_step() {
        let (_, (template, rules)) = parse::polymer(EXAMPLE_INPUT).unwrap();
        let freqs = steps(template.as_str(), 10, &rules);
        assert_eq!(freqs[&'B'], 1749)
    }

    #[test]
    fn part1() {
        let input = puzzle_input("14");
        let (_, (template, rules)) = parse::polymer(&input).unwrap();
        let freqs = steps(template.as_str(), 10, &rules);

        let least = *freqs.values().min().unwrap();
        let most = *freqs.values().max().unwrap();

        assert_eq!(most - least, 2068);
    }

    #[test]
    fn test_step_long() {
        let (_, (template, rules)) = parse::polymer(EXAMPLE_INPUT).unwrap();
        steps(template.as_str(), 1, &rules);
        let freqs = steps(template.as_str(), 40, &rules);
        assert_eq!(freqs[&'B'], 2192039569602)
    }

    #[test]
    fn part2() {
        let input = puzzle_input("14");
        let (_, (template, rules)) = parse::polymer(&input).unwrap();
        let freqs = steps(template.as_str(), 40, &rules);

        let least = *freqs.values().min().unwrap();
        let most = *freqs.values().max().unwrap();

        assert_eq!(most - least, 2158894777814);
    }
}


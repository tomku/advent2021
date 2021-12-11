#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChunkPart {
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,
    AngleOpen,
    AngleClose,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ChunkError {
    Incomplete(Vec<ChunkPart>),
    Corrupt(ChunkPart),
}

impl ChunkPart {
    fn is_match(&self, other: &ChunkPart) -> bool {
        self.close() == *other
    }

    fn close(&self) -> ChunkPart {
        match self {
            ChunkPart::ParenOpen => ChunkPart::ParenClose,
            ChunkPart::BracketOpen => ChunkPart::BracketClose,
            ChunkPart::BraceOpen => ChunkPart::BraceClose,
            ChunkPart::AngleOpen => ChunkPart::AngleClose,
            _ => panic!("Can't close a closer!")
        }
    }
}

fn fix_error(line: &Vec<ChunkPart>) -> ChunkError {
    let mut stack = Vec::new();

    for &part in line {
        match part {
            ChunkPart::ParenOpen => stack.push(part),
            ChunkPart::BracketOpen => stack.push(part),
            ChunkPart::BraceOpen => stack.push(part),
            ChunkPart::AngleOpen => stack.push(part),
            other => {
                match stack.pop() {
                    None => continue,
                    Some(part) => if part.is_match(&other) {
                        continue;
                    } else {
                        return ChunkError::Corrupt(other);
                    }
                }
            }
        }
    }

    stack.reverse();
    ChunkError::Incomplete(stack.iter().map(|part| part.close()).collect())
}

fn score_corrupt(subsystem: &Vec<Vec<ChunkPart>>) -> u64 {
    subsystem.iter().filter_map(|line| {
        match fix_error(line) {
            ChunkError::Incomplete(_) => None,
            ChunkError::Corrupt(ChunkPart::ParenClose) => Some(3),
            ChunkError::Corrupt(ChunkPart::BracketClose) => Some(57),
            ChunkError::Corrupt(ChunkPart::BraceClose) => Some(1197),
            ChunkError::Corrupt(ChunkPart::AngleClose) => Some(25137),
            _ => panic!("Invalid corrupt value")
        }
    }).sum()
}

fn score_incomplete(subsystem: &Vec<Vec<ChunkPart>>) -> u64 {
    let mut scores: Vec<u64> = subsystem.iter().filter_map(|line| {
        match fix_error(line) {
            ChunkError::Incomplete(closers) => {
                let mut score = 0;
                for c in closers {
                    score *= 5;
                    score += match c {
                        ChunkPart::ParenClose => 1,
                        ChunkPart::BracketClose => 2,
                        ChunkPart::BraceClose => 3,
                        ChunkPart::AngleClose => 4,
                        _ => panic!("Invalid closer!")
                    }
                }
                Some(score)
            }
            ChunkError::Corrupt(_) => None,
        }
    }).collect();

    scores.sort();
    scores[scores.len() / 2]
}

mod parse {
    use nom::character::complete::{newline, one_of};
    use nom::combinator::map;
    use nom::IResult;
    use nom::multi::{many1, separated_list1};

    use super::*;

    fn part(input: &str) -> IResult<&str, ChunkPart> {
        map(one_of("(){}[]<>"), |ch| {
            match ch {
                '(' => ChunkPart::ParenOpen,
                ')' => ChunkPart::ParenClose,
                '[' => ChunkPart::BracketOpen,
                ']' => ChunkPart::BracketClose,
                '{' => ChunkPart::BraceOpen,
                '}' => ChunkPart::BraceClose,
                '<' => ChunkPart::AngleOpen,
                '>' => ChunkPart::AngleClose,
                _ => panic!("Shouldn't happen...")
            }
        })(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<ChunkPart>> {
        many1(part)(input)
    }

    pub(crate) fn subsystem(input: &str) -> IResult<&str, Vec<Vec<ChunkPart>>> {
        separated_list1(newline, line)(input)
    }
}

mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_find_corrupt() {
        let (_, subsystem) = parse::subsystem(TEST_INPUT).unwrap();
        assert_eq!(fix_error(&subsystem[2]), ChunkError::Corrupt(ChunkPart::BraceClose));
        assert_eq!(fix_error(&subsystem[4]), ChunkError::Corrupt(ChunkPart::ParenClose));
        assert_eq!(fix_error(&subsystem[5]), ChunkError::Corrupt(ChunkPart::BracketClose));
        assert_eq!(fix_error(&subsystem[7]), ChunkError::Corrupt(ChunkPart::ParenClose));
        assert_eq!(fix_error(&subsystem[8]), ChunkError::Corrupt(ChunkPart::AngleClose));
    }

    #[test]
    fn test_score_corrupt() {
        let (_, subsystem) = parse::subsystem(TEST_INPUT).unwrap();
        assert_eq!(score_corrupt(&subsystem), 26397);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("10");
        let (_, subsystem) = parse::subsystem(&input).unwrap();
        assert_eq!(score_corrupt(&subsystem), 265527);
    }

    #[test]
    fn test_score_incomplete() {
        let (_, subsystem) = parse::subsystem(TEST_INPUT).unwrap();
        assert_eq!(score_incomplete(&subsystem), 288957);
    }

    #[test]
    fn part2() {
        let input = puzzle_input("10");
        let (_, subsystem) = parse::subsystem(&input).unwrap();
        assert_eq!(score_incomplete(&subsystem), 3969823589);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    pub depth: i32,
    pub distance: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AimedPosition {
    pub pos: Position,
    pub aim: i32,
}

pub fn perform_direct(pos: Position, cmd: &Command) -> Position {
    match cmd {
        Command::Forward(d) => Position { distance: pos.distance + d, ..pos },
        Command::Up(d) => Position { depth: pos.depth - d, ..pos },
        Command::Down(d) => Position { depth: pos.depth + d, ..pos }
    }
}

pub const INITIAL: Position = Position { depth: 0, distance: 0 };

pub fn perform_aimed(state: AimedPosition, cmd: &Command) -> AimedPosition {
    match cmd {
        Command::Forward(d) => AimedPosition {
            pos: Position {
                distance: state.pos.distance + d,
                depth: state.pos.depth + state.aim * d,
            },
            ..state
        },
        Command::Up(d) => AimedPosition { aim: state.aim - d, ..state },
        Command::Down(d) => AimedPosition { aim: state.aim + d, ..state }
    }
}

pub const INITIAL_AIM: AimedPosition = AimedPosition { aim: 0, pos: INITIAL };

pub mod parse {
    use nom::{IResult, Parser};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, newline};
    use nom::combinator::map_res;
    use nom::multi::separated_list1;
    use nom::sequence::preceded;

    use super::Command::{self, Down, Forward, Up};

    pub fn digits_as_i32(input: &str) -> IResult<&str, i32> {
        map_res(digit1, |num| str::parse(num))(input)
    }

    pub fn forward(input: &str) -> IResult<&str, Command> {
        preceded(tag("forward "),
                 digits_as_i32.map(Forward))(input)
    }

    pub fn up(input: &str) -> IResult<&str, Command> {
        preceded(tag("up "),
                 digits_as_i32.map(Up))(input)
    }

    pub fn down(input: &str) -> IResult<&str, Command> {
        preceded(tag("down "),
                 digits_as_i32.map(Down))(input)
    }

    pub fn direction(input: &str) -> IResult<&str, Command> {
        alt((forward, up, down))(input)
    }

    pub fn commands(input: &str) -> IResult<&str, Vec<Command>> {
        separated_list1(newline, direction)(input)
    }
}

#[cfg(test)]
mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";

    #[test]
    fn test_parse() {
        use super::Command::*;

        let (_, commands) = super::parse::commands(TEST_INPUT).unwrap();
        assert_eq!(commands, [Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)])
    }

    #[test]
    fn test_perform_direct() {
        let (_, commands) = super::parse::commands(TEST_INPUT).unwrap();
        let last = commands.iter().fold(INITIAL, perform_direct);

        assert_eq!(last.depth, 10);
        assert_eq!(last.distance, 15);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("02");
        let (_, commands) = super::parse::commands(&input).unwrap();
        let last = commands.iter().fold(INITIAL, perform_direct);

        assert_eq!(last.depth, 738);
        assert_eq!(last.distance, 2011);
    }

    #[test]
    fn test_perform_aimed() {
        let (_, commands) = super::parse::commands(TEST_INPUT).unwrap();
        let last = commands.iter().fold(INITIAL_AIM, perform_aimed);

        assert_eq!(last.pos.depth, 60);
        assert_eq!(last.pos.distance, 15);
    }

    #[test]
    fn part2() {
        let input = puzzle_input("02");
        let (_, commands) = super::parse::commands(&input).unwrap();
        let last = commands.iter().fold(INITIAL_AIM, perform_aimed);

        assert_eq!(last.pos.depth, 727910);
        assert_eq!(last.pos.distance, 2011);
    }
}

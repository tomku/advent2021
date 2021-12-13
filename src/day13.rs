use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Direction {
    Up,
    Left,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Fold {
    coord: u32,
    dir: Direction,
}

pub struct Instructions {
    paper: HashSet<(u32, u32)>,
    folds: Vec<Fold>,
}

fn fold_paper(paper: HashSet<(u32, u32)>, f: &Fold) -> HashSet<(u32, u32)> {
    let mut folded = HashSet::new();
    for (x, y) in paper {
        let d = match f.dir {
            Direction::Up => y,
            Direction::Left => x
        };

        if d < f.coord {
            folded.insert((x, y));
        } else {
            let new_d = 2 * f.coord - d;
            match f.dir {
                Direction::Up => folded.insert((x, new_d)),
                Direction::Left => folded.insert((new_d, y))
            };
        }
    }
    folded
}

mod parse {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{newline, u32 as num};
    use nom::combinator::{map, value};
    use nom::IResult;
    use nom::multi::{fold_many1, separated_list1};
    use nom::sequence::{preceded, separated_pair, terminated};

    use super::*;

    fn coord(input: &str) -> IResult<&str, (u32, u32)> {
        terminated(separated_pair(num, tag(","), num),
                   newline)(input)
    }

    fn fold(input: &str) -> IResult<&str, Fold> {
        let left = value(Direction::Left, tag("x"));
        let up = value(Direction::Up, tag("y"));
        let dir = alt((left, up));
        let pair = preceded(tag("fold along "),
                            separated_pair(dir,
                                           tag("="),
                                           num));
        map(pair, |(dir, coord)|
            Fold { coord, dir },
        )(input)
    }

    pub(crate) fn instructions(input: &str) -> IResult<&str, Instructions> {
        let coords =
            fold_many1(coord, HashSet::new,
                       |mut acc, p| {
                           acc.insert(p);
                           acc
                       });
        let folds = separated_list1(newline, fold);
        map(separated_pair(coords, newline, folds),
            |(c, f)| Instructions { paper: c, folds: f })(input)
    }
}

mod test {
    use crate::util::puzzle_input;

    use super::*;

    const EXAMPLE_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_parse() {
        let (_, instr) = parse::instructions(EXAMPLE_INPUT).unwrap();
        assert_eq!(instr.folds[1], Fold { coord: 5, dir: Direction::Left });
        assert!(instr.paper.contains(&(9, 0)));
        assert!(instr.paper.contains(&(0, 14)));
    }

    #[test]
    fn test_single_fold() {
        let (_, instr) = parse::instructions(EXAMPLE_INPUT).unwrap();
        let folded = fold_paper(instr.paper, &instr.folds[0]);
        assert_eq!(folded.len(), 17);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("13");
        let (_, instr) = parse::instructions(&input).unwrap();
        let folded = fold_paper(instr.paper, &instr.folds[0]);
        assert_eq!(folded.len(), 682);
    }

    #[test]
    fn part2() {
        use std::fmt::Write;

        let input = puzzle_input("13");
        let (_, instr) = parse::instructions(&input).unwrap();
        let folded = instr.folds.iter().fold(instr.paper, fold_paper);

        let mut actual = String::with_capacity(280);
        for y in 0..6 {
            for x in 0..39 {
                let p = folded.contains(&(x, y));
                if p { actual.write_char('*').unwrap(); } else { actual.write_char(' ').unwrap(); }
            }
            actual.write_char('\n').unwrap();
        }

        let expected = concat!(
        "****  **   **  *  * ***  **** *  * ****\n",
        "*    *  * *  * *  * *  *    * *  * *   \n",
        "***  *  * *    *  * *  *   *  **** *** \n",
        "*    **** * ** *  * ***   *   *  * *   \n",
        "*    *  * *  * *  * * *  *    *  * *   \n",
        "*    *  *  ***  **  *  * **** *  * ****\n"
        );

        assert_eq!(expected, actual);
    }
}
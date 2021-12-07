#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BingoSquare {
    Filled(u32),
    Open(u32),
}

impl BingoSquare {
    fn is_filled(&self) -> bool {
        match self {
            BingoSquare::Filled(_) => true,
            BingoSquare::Open(_) => false
        }
    }
}

#[derive(Clone)]
pub struct BingoBoard(Vec<BingoSquare>);

impl std::ops::Deref for BingoBoard {
    type Target = Vec<BingoSquare>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BingoBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BingoBoard {
    pub fn mark(&mut self, num: u32) {
        for sq in self.iter_mut() {
            if let BingoSquare::Open(n) = sq {
                if *n == num { *sq = BingoSquare::Filled(*n) }
            }
        }
    }

    pub fn open_nums(&self) -> u32 {
        self.iter().map(|&sq| match sq {
            BingoSquare::Filled(_) => 0,
            BingoSquare::Open(n) => n
        }).sum()
    }

    pub fn is_winner(&self) -> bool {
        for col in 0..5 {
            if self.iter().skip(col).step_by(5).all(|sq| sq.is_filled()) {
                return true;
            }
        }

        for row in self.chunks(5) {
            if row.iter().all(|sq| sq.is_filled()) {
                return true;
            }
        }

        return false;
    }
}

pub fn call(num: u32, boards: &mut Vec<BingoBoard>) {
    for b in boards {
        b.mark(num);
    }
}

pub fn play_until_winner(nums: Vec<u32>, boards: &mut Vec<BingoBoard>) -> Option<(u32, BingoBoard)> {
    for n in nums {
        call(n, boards);
        if let Some(w) = boards.iter().find(|b| b.is_winner()) {
            return Some((n, w.clone()));
        }
    }
    return None;
}

pub fn play_until_last_winner(nums: Vec<u32>, boards: &mut Vec<BingoBoard>) -> Option<(u32, BingoBoard)> {
    for n in nums {
        if boards.len() == 1 {
            call(n, boards);
            return Some((n, boards[0].clone()));
        }
        call(n, boards);
        boards.retain(|b| !b.is_winner());
    }
    return None;
}

mod parse {
    use nom::{IResult, Parser};
    use nom::bytes::complete::tag;
    use nom::character::complete::{multispace0, newline, u32 as num};
    use nom::combinator::map;
    use nom::multi::{count, many1, separated_list1};
    use nom::sequence::{preceded, terminated, tuple};

    use super::{BingoBoard, BingoSquare};

    fn call_sequence(input: &str) -> IResult<&str, Vec<u32>> {
        terminated(
            separated_list1(tag(","), num),
            many1(newline),
        )(input)
    }

    fn board(input: &str) -> IResult<&str, BingoBoard> {
        map(count(
            preceded(multispace0, num.map(BingoSquare::Open)),
            25),
            BingoBoard)(input)
    }

    pub fn bingo_game(input: &str) -> IResult<&str, (Vec<u32>, Vec<BingoBoard>)> {
        tuple((
            call_sequence,
            many1(board))
        )(input)
    }
}

#[cfg(test)]
mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_parse() {
        let (_, (calls, boards)) = parse::bingo_game(TEST_INPUT).unwrap();
        assert_eq!(calls[0], 7);
        assert_eq!(calls[26], 1);
        assert_eq!(boards[2][0], BingoSquare::Open(14));
        assert_eq!(boards[2][24], BingoSquare::Open(7));
    }

    #[test]
    fn test_play_until_winner() {
        let (_, (calls, mut boards)) = parse::bingo_game(TEST_INPUT).unwrap();
        let (last_num, winner) = play_until_winner(calls, &mut boards).unwrap();
        assert_eq!(last_num, 24);
        assert_eq!(winner.open_nums(), 188)
    }

    #[test]
    fn test_play_until_last_winner() {
        let (_, (calls, mut boards)) = parse::bingo_game(TEST_INPUT).unwrap();
        let (last_num, winner) = play_until_last_winner(calls, &mut boards).unwrap();
        assert_eq!(last_num, 13);
        assert_eq!(winner.open_nums(), 148)
    }

    #[test]
    fn part1() {
        let input = puzzle_input("04");
        let (_, (calls, mut boards)) = parse::bingo_game(&input).unwrap();
        let (last_num, winner) = play_until_winner(calls, &mut boards).unwrap();

        assert_eq!(last_num, 78);
        assert_eq!(winner.open_nums(), 715)
    }

    #[test]
    fn part2() {
        let input = puzzle_input("04");
        let (_, (calls, mut boards)) = parse::bingo_game(&input).unwrap();
        let (last_num, winner) = play_until_last_winner(calls, &mut boards).unwrap();

        assert_eq!(last_num, 10);
        assert_eq!(winner.open_nums(), 298)
    }
}
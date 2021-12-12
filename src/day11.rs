pub struct Grid {
    cells: Vec<u8>,
    flash_queue: Vec<usize>,
    seen: u128,
}

impl Grid {
    fn new(cells: Vec<u8>) -> Grid {
        Grid {
            cells,
            flash_queue: Vec::new(),
            seen: 0,
        }
    }

    fn incr(&mut self, i: i32) {
        if i < 0 || i >= self.cells.len() as i32 { return; }

        let mask: u128 = 1 << (i as u128);
        if self.seen & mask != 0 { return; }

        let u = i as usize;
        self.cells[u] += 1;
        if self.cells[u] > 9 {
            self.seen |= mask;
            self.cells[u] = 0;

            self.flash_queue.push(u);
        }
    }

    pub fn step(&mut self) -> usize {
        for i in 0..100 {
            self.incr(i)
        }

        let mut total = 0;
        loop {
            if let Some(next) = self.flash_queue.pop() {
                total += 1;
                self.flash(next as i32);
            } else {
                self.seen = 0;
                break total;
            }
        }
    }

    pub fn steps(&mut self, num: usize) -> usize {
        let mut flashes = 0;
        for _ in 0..num {
            flashes += self.step();
        }
        flashes
    }

    pub fn steps_until_sync(&mut self) -> usize {
        let mut steps = 0;
        loop {
            self.step();
            steps += 1;
            if self.cells.iter().all(|&c| c == 0) {
                break steps;
            }
        }
    }

    fn flash(&mut self, start: i32) {
        let x = start % 10;
        if x > 0 {
            self.incr(start - 11);
            self.incr(start - 1);
            self.incr(start + 9);
        }
        self.incr(start - 10);
        self.incr(start + 10);
        if x < 9 {
            self.incr(start - 9);
            self.incr(start + 1);
            self.incr(start + 11);
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..10 {
            for x in 0..10 {
                let c = self.cells[y * 10 + x];
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Result::Ok(())
    }
}

mod parse {
    use nom::character::complete::{newline, one_of};
    use nom::combinator::{map, opt};
    use nom::IResult;
    use nom::multi::fold_many1;
    use nom::sequence::terminated;

    use super::*;

    pub(crate) fn grid(input: &str) -> IResult<&str, Grid> {
        let digit = terminated(one_of("0123456789"), opt(newline));
        let digit_as_u8 = map(digit, |c: char| c.to_digit(10).unwrap() as u8);
        map(fold_many1(digit_as_u8, || Vec::with_capacity(100), |mut acc: Vec<u8>, d| {
            acc.push(d);
            acc
        }), |v| Grid::new(v))(input)
    }
}

mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_parse() {
        let (_, grid) = parse::grid(TEST_INPUT).unwrap();

        assert_eq!(grid.cells.len(), 100);
        assert_eq!(grid.cells[99], 6);
    }

    #[test]
    fn test_step() {
        let (_, mut grid) = parse::grid(TEST_INPUT).unwrap();

        let flashes = grid.steps(100);
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("11");
        let (_, mut grid) = parse::grid(&input).unwrap();

        let flashes = grid.steps(100);
        assert_eq!(flashes, 1617);
    }

    #[test]
    fn test_sync() {
        let (_, mut grid) = parse::grid(TEST_INPUT).unwrap();

        let steps = grid.steps_until_sync();
        assert_eq!(steps, 195);
    }

    #[test]
    fn part2() {
        let input = puzzle_input("11");
        let (_, mut grid) = parse::grid(&input).unwrap();

        let steps = grid.steps_until_sync();
        assert_eq!(steps, 258);
    }
}
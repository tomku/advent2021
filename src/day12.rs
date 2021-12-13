use std::collections::HashSet;

pub struct CaveMap {
    connections: Vec<(usize, usize)>,
    names: Vec<String>,
    sizes: Vec<Size>,
}

impl CaveMap {
    fn new() -> CaveMap {
        CaveMap {
            connections: Vec::new(),
            names: Vec::new(),
            sizes: Vec::new(),
        }
    }

    fn count_paths(&self) -> u32 {
        let start = self.get_id("start").unwrap();
        let mut results = HashSet::new();
        self.count_paths_from(start, &Vec::new(), false, &mut results);

        results.len() as u32
    }

    fn count_paths_cheat(&self) -> u32 {
        let start = self.get_id("start").unwrap();
        let mut results = HashSet::new();
        self.count_paths_from(start, &Vec::new(), true, &mut results);

        results.len() as u32
    }

    fn count_paths_from(&self, start_id: usize, so_far: &Vec<usize>, allow_single_dupe: bool, output: &mut HashSet<Vec<usize>>) {
        let final_end = self.get_id("end").unwrap();
        let beginning = self.get_id("start").unwrap();
        if start_id == final_end {
            output.insert(so_far.clone());
        } else {
            let conns =
                self.connections.iter().filter(|(start, end)| {
                    if *start != start_id { return false; }
                    if self.sizes[*end] == Size::Large { return true; }
                    if !so_far.contains(&end) { return true; }
                    return false;
                });
            let mut new_so_far = so_far.clone();
            new_so_far.push(start_id);

            for &(_, end) in conns {
                self.count_paths_from(end, &new_so_far, allow_single_dupe, output)
            }

            if allow_single_dupe {
                let revisits =
                    self.connections.iter().filter(|(start, end)| {
                        if *start != start_id { return false; }
                        if *end == beginning { return false; }
                        if self.sizes[*end] == Size::Large { return false; }
                        if so_far.contains(&end) { return true; }
                        return false;
                    });

                for &(_, end) in revisits {
                    self.count_paths_from(end, &new_so_far, false, output)
                }
            }
        }
    }

    fn get_id(&self, cave: &str) -> Option<usize> {
        self.names.iter().position(|n| n == cave)
    }

    fn ensure_id(&mut self, cave: &str) -> usize {
        match self.get_id(cave) {
            Some(id) => id,
            None => {
                let new_id = self.names.len();
                self.names.push(cave.to_owned());
                let size = if cave.starts_with(|ch: char| ch.is_lowercase()) {
                    Size::Small
                } else {
                    Size::Large
                };
                self.sizes.push(size);
                new_id
            }
        }
    }

    fn connect(&mut self, start: &str, end: &str) {
        let start_id = self.ensure_id(start);
        let end_id = self.ensure_id(end);
        self.connections.push((start_id, end_id));
        self.connections.push((end_id, start_id));
    }

    fn is_connected(&self, start: &str, end: &str) -> bool {
        let start_id = self.get_id(start).unwrap();
        let end_id = self.get_id(end).unwrap();
        self.connections.contains(&(start_id, end_id))
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum Size {
    Small,
    Large,
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, newline};
    use nom::combinator::opt;
    use nom::IResult;
    use nom::multi::fold_many1;
    use nom::sequence::{separated_pair, terminated};

    use super::*;

    fn cave(input: &str) -> IResult<&str, &str> {
        alpha1(input)
    }

    fn connection(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(cave, tag("-"), cave)(input)
    }

    pub(crate) fn cave_system(input: &str) -> IResult<&str, CaveMap> {
        fold_many1(
            terminated(connection, opt(newline)),
            || CaveMap::new(),
            |mut acc, (from, to)| {
                acc.connect(from, to);
                acc
            },
        )(input)
    }
}

mod test {
    use crate::util::puzzle_input;

    use super::*;

    const TRIVIAL_EXAMPLE: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const SLIGHTLY_LARGER_EXAMPLE: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const BIG_EXAMPLE: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_parse() {
        let (_, caves) = parse::cave_system(TRIVIAL_EXAMPLE).unwrap();
        assert!(caves.is_connected("start", "A"));
        assert!(caves.is_connected("A", "end"));
        assert!(!caves.is_connected("start", "end"))
    }

    #[test]
    fn test_trivial_paths() {
        let (_, caves) = parse::cave_system(TRIVIAL_EXAMPLE).unwrap();
        assert_eq!(caves.count_paths(), 10);
    }

    #[test]
    fn test_slightly_larger_paths() {
        let (_, caves) = parse::cave_system(SLIGHTLY_LARGER_EXAMPLE).unwrap();
        assert_eq!(caves.count_paths(), 19);
    }

    #[test]
    fn test_big_paths() {
        let (_, caves) = parse::cave_system(BIG_EXAMPLE).unwrap();
        assert_eq!(caves.count_paths(), 226);
    }

    #[test]
    fn part1() {
        let input = puzzle_input("12");
        let (_, caves) = parse::cave_system(&input).unwrap();
        assert_eq!(caves.count_paths(), 4378);
    }

    #[test]
    fn test_trivial_paths_cheat() {
        let (_, caves) = parse::cave_system(TRIVIAL_EXAMPLE).unwrap();
        assert_eq!(caves.count_paths_cheat(), 36);
    }

    #[test]
    fn test_slightly_larger_paths_cheat() {
        let (_, caves) = parse::cave_system(SLIGHTLY_LARGER_EXAMPLE).unwrap();
        assert_eq!(caves.count_paths_cheat(), 103);
    }

    #[test]
    fn test_big_paths_cheat() {
        let (_, caves) = parse::cave_system(BIG_EXAMPLE).unwrap();
        assert_eq!(caves.count_paths_cheat(), 3509);
    }

    #[test]
    fn part2() {
        let input = puzzle_input("12");
        let (_, caves) = parse::cave_system(&input).unwrap();
        assert_eq!(caves.count_paths_cheat(), 133621);
    }
}
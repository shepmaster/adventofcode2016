extern crate itertools;

use std::str::FromStr;

type Error = Box<::std::error::Error>;
type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        Ok(match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => return Err(format!("Unknown tile '{}'", c).into()),
        })
    }
}

fn rule(parents: &[Tile]) -> Tile {
    use Tile::*;

    match (parents[0], parents[1], parents[2]) {
        (Trap, Trap, Safe) |
        (Safe, Trap, Trap) |
        (Trap, Safe, Safe) |
        (Safe, Safe, Trap) => Trap,
        _ => Safe,
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Row(Vec<Tile>);

impl Row {
    fn step(&self) -> Row {
        use Tile::*;

        let mut padded = Vec::with_capacity(self.0.len() + 2);
        padded.push(Safe); // The wall
        padded.extend_from_slice(&self.0);
        padded.push(Safe); // Other wall

        Row(padded.windows(3).map(rule).collect())
    }

    fn safe_rows(&self) -> usize {
        self.0.iter().filter(|&&tile| tile == Tile::Safe).count()
    }
}

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tiles: Result<_> = s.chars().map(Tile::from_char).collect();
        tiles.map(Row)
    }
}

use std::mem;

fn puzzle(s: &str, row_count: usize) -> Result<usize> {
    let row: Row = s.parse()?;
    let rows = itertools::unfold(row, |row| {
        let next = row.step();
        let current = mem::replace(row, next);
        Some(current)
    });

    Ok(rows.take(row_count).map(|r| r.safe_rows()).sum())
}

fn main() {
    let input = include_str!("input.txt");
    println!("There were {:?} spaces", puzzle(input, 40));
}

#[test]
fn test_parsing() {
    use Tile::*;

    assert_eq!(
        "..^^.".parse::<Row>().unwrap(),
        Row(vec![Safe, Safe, Trap, Trap, Safe])
    );
}

#[test]
fn test_step() {
    use Tile::*;

    assert_eq!(
        Row(vec![Safe, Safe, Trap, Trap, Safe]).step(),
        Row(vec![Safe, Trap, Trap, Trap, Trap])
    );
}

#[test]
fn test_puzzle() {
    assert_eq!(puzzle(".^^.^.^^^^", 10).unwrap(), 38);
}

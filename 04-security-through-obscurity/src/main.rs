extern crate revord;

use std::collections::BTreeMap;
use std::error::Error;
use std::str::FromStr;

use revord::RevOrd;

struct Room {
    id: String,
    frequency: BTreeMap<char, usize>,
    sector: u16,
    code: String,
}

impl Room {
    fn sector_id(&self) -> u16 { self.sector }

    fn common_letters(&self) -> Vec<char> {
        let mut frequency: Vec<_> = self.frequency.iter().collect();
        frequency.sort_by_key(|&(c, f)| (RevOrd(f), c));
        frequency.into_iter().take(5).map(|(&c, _)| c).collect()
    }

    fn is_real(&self) -> bool {
        self.common_letters().into_iter().eq(self.code.chars())
    }

    fn decrypted_id(&self) -> String {
        shift_cipher(&self.id, self.sector as usize)
    }
}

impl FromStr for Room {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Let's be lazy and do some allocation
        let s = s.replace("[", "-").replace("]", "");

        let mut parts = s.rsplitn(3, "-");

        let code = parts.next().ok_or("Missing code".to_string())?;
        let sector = parts.next().ok_or("Missing sector".to_string())?;
        let id = parts.next().ok_or("Missing characters".to_string())?;

        let mut frequency = BTreeMap::new();
        for c in id.chars().filter(|c| c.is_alphabetic()) {
            *frequency.entry(c).or_insert(0) += 1;
        }

        Ok(Room {
            id: id.into(),
            frequency: frequency,
            sector: sector.parse()?,
            code: code.into(),
        })
    }
}

fn shift_cipher(s: &str, count: usize) -> String {
    let count = (count % 26) as u8;
    s.bytes().map(|byte| {
        let shifted_byte = match byte {
            b'-' => b' ',
            b => ((((b - b'a') + count) % 26) + b'a'),
        };
        shifted_byte as char
    }).collect()
}

fn main() {
    let input = include_str!("input.txt");

    let valid_rooms: Vec<_> = input.lines()
        .filter_map(|l| l.parse::<Room>().ok())
        .filter(Room::is_real)
        .collect();

    let sum: u64 = valid_rooms.iter().map(|r| r.sector_id() as u64).sum();
    println!("Sum of valid sectors: {}", sum);

    for r in valid_rooms.iter().find(|r| r.decrypted_id() == "northpole object storage") {
        println!("Storage in sector {}", r.sector_id());
    }
}

#[test]
fn example1() {
    let room = Room::from_str("aaaaa-bbb-z-y-x-123[abxyz]").expect("Unable to parse room");

    assert_eq!(room.common_letters(), ['a', 'b', 'x', 'y', 'z']);
    assert!(room.is_real());
}

#[test]
fn example2() {
    let room = Room::from_str("a-b-c-d-e-f-g-h-987[abcde]").expect("Unable to parse room");
    assert!(room.is_real())
}

#[test]
fn example3() {
    let room = Room::from_str("not-a-real-room-404[oarel]").expect("Unable to parse room");
    assert!(room.is_real())
}

#[test]
fn example4() {
    let room = Room::from_str("totally-real-room-200[decoy]").expect("Unable to parse room");
    assert!(!room.is_real())
}

#[test]
fn shift_cipher_example() {
    assert_eq!(shift_cipher("qzmt-zixmtkozy-ivhz", 343), "very encrypted name");
}

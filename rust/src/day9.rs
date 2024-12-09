use std::iter;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = DiskMap;
type FileId = u32;

#[derive(Eq, PartialEq, Clone)]
struct DiskMap {
    disk: Vec<Option<FileId>>,
}

impl std::fmt::Debug for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for on_disk in &self.disk {
            match on_disk {
                Some(id) => s.push_str(&id.to_string()),
                None => s.push('.'),
            }
        }
        write!(f, "{s}")
    }
}

impl DiskMap {
    fn defrag(&mut self) {
        let mut dst = 0;
        let mut src = self.disk.len() - 1;
        while src != dst {
            if self.disk[src].is_none() {
                src -= 1;
                continue;
            };
            if self.disk[dst].is_some() {
                dst += 1;
                continue;
            }
            self.disk.swap(src, dst);
        }
    }

    fn checksum(&self) -> u64 {
        self.disk
            .iter()
            .enumerate()
            .map(|(i, block)| {
                if let Some(b) = block {
                    *b as u64 * i as u64
                } else {
                    0
                }
            })
            .sum()
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    let mut disk: Vec<Option<FileId>> = Vec::new();
    let mut is_file = false;
    let mut file_id = 0;
    for ch in input.trim().chars() {
        is_file = !is_file;
        let num_blocks = ch.to_digit(10).unwrap();
        let blocks = if is_file {
            let b = Some(file_id);
            file_id += 1;
            b
        } else {
            None
        };
        disk.extend(iter::repeat_n(blocks, num_blocks as usize));
    }
    DiskMap { disk }
}

#[aoc(day9, part1)]
fn q1(input: &Input) -> u64 {
    let mut solved = input.to_owned();
    solved.defrag();
    solved.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        for (input, expected) in [
            ("12345", "0..111....22222"),
            (
                "2333133121414131402",
                "00...111...2...333.44.5555.6666.777.888899",
            ),
        ] {
            let mut actual = parse(input);
            assert_eq!(format!("{actual:?}"), expected);

            println!("Fragmentd: {actual:?}");
            actual.defrag();
            println!("Defragged: {actual:?}");
        }
    }

    #[test]
    fn test_real() {
        let mut disk = parse(&std::fs::read_to_string("input/2024/day9.txt").unwrap());
        disk.defrag();
        let expected = 6241633730082;
        assert_eq!(q1(&disk), expected);
    }

    #[test]
    fn test_example() {
        let mut disk = parse(&"2333133121414131402");
        disk.defrag();
        let expected = 1928;
        assert_eq!(q1(&disk), expected);
    }
}

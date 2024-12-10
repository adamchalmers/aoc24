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

    /// Find a free space big enough to fit all `want_size` blocks.
    /// Search in the given range (inclusive, exclusive).
    fn find_free_space_at_least(
        &self,
        want_size: usize,
        starting_at: usize,
        ending_at: usize,
    ) -> Option<usize> {
        let mut candidate_dst = starting_at;
        while candidate_dst < ending_at {
            if self.disk[candidate_dst].is_some() {
                candidate_dst += 1;
                continue;
            }
            let mut len_dst = 1;
            while self.disk[candidate_dst + len_dst] == self.disk[candidate_dst] {
                len_dst += 1;
            }

            // Is this free space big enough to hold the file?
            if len_dst >= want_size {
                return Some(candidate_dst);
            }
            // If not, then skip over this free space, let's try the next one.
            candidate_dst += len_dst;
        }
        None
    }

    fn defrag_entire_files(&mut self) {
        let mut dst = 0;
        let mut src = self.disk.len() - 1;
        while src != dst {
            // Find the end of the rightmost file.
            if self.disk[src].is_none() {
                src -= 1;
                continue;
            };
            // Find the start of the leftmost free space.
            if self.disk[dst].is_some() {
                dst += 1;
                continue;
            }
            // How long is the file?
            let mut filesize = 1;
            while self.disk[src - filesize] == self.disk[src] {
                filesize += 1;
            }

            if let Some(dst_start) = self.find_free_space_at_least(filesize, dst, src) {
                // Swap the free space and the file.
                for i in 0..filesize {
                    self.disk.swap(dst_start + i, src - i);
                }
                // Look for a new file to defragment, and re-examine all possible
                // free spaces (because the new file might be smaller than the current one,
                // and fit into spaces this one couldn't).
                src -= filesize;
                dst = 0;
            } else {
                // We never found a free space big enough for the file.
                // So you should just move on to the next file, this one can't be defragmented.
                src -= filesize;
            }
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

#[aoc(day9, part2)]
fn q2(input: &Input) -> u64 {
    let mut solved = input.to_owned();
    solved.defrag_entire_files();
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
        let disk = parse("2333133121414131402");
        let actual = q1(&disk);
        let expected = 1928;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example_q2() {
        let disk = parse("2333133121414131402");
        let actual = q2(&disk);
        let expected = 2858;
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_real_q2() {
        let disk = parse(&std::fs::read_to_string("input/2024/day9.txt").unwrap());
        let actual = q2(&disk);
        let expected = 6265268809555;
        assert_eq!(actual, expected);
    }
}
